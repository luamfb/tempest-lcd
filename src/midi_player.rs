// Copyright (C) 2022-2025 Luana Martins Barbosa
//
// This file is part of tempest-lcd.
// tempest-lcd is free software, released under the
// GNU Public License, version 2 only.
// See COPYING.txt.

use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

use midly::{
    Smf,
    Format,
    Timing,
    Track,
    TrackEvent,
    TrackEventKind,
    MetaMessage,
    MidiMessage,
    num::{u24, u28, u7},
};

use crate::{
    gui::Gui,
};

// equals 120BPM if quarter is the beat
const DEFAULT_MICROSEC_PER_QUATER : u32 = 500_000;

const PAUSE_SLEEP_INTERVAL: Duration = Duration::from_millis(5);

pub struct MidiPlayer {
    gui: Gui,
    running: bool,
    paused: bool,
    wave_is_cosine: bool,
}

impl MidiPlayer {
    pub fn create(gui: Gui, wave_is_cosine: bool) -> Self {
        MidiPlayer {
            gui,
            paused: false,
            running: false,
            wave_is_cosine,
        }
    }

    // this is only necessary because we can't pass mutable references
    // of our fields to Gui directly (and it's identical to LegacyPlayer)
    pub fn handle_gui_events(&mut self) {
        let mut running = self.running;
        let mut paused = self.paused;

        self.gui.handle_events(&mut running, &mut paused);

        self.running = running;
        self.paused = paused;
    }


    pub fn run(&mut self, smf: Smf) {
        if self.running {
            return;
        }
        self.running = true;
        match smf.header.format {
            Format::SingleTrack | Format::Parallel =>
                self.run_tracks_parallel(smf.header.timing,
                                         &smf.tracks),
            Format::Sequential => for track in smf.tracks {
                self.run_tracks_parallel(smf.header.timing,
                                         &[track]);
            },
        }
    }

    fn run_tracks_parallel<'a>(&mut self,
                               timing: Timing,
                               tracks: &[Track<'a>]) {
        let mut tick_duration = get_tick_duration(
            timing,
            DEFAULT_MICROSEC_PER_QUATER.into());

        let mut track_iters : Vec<_> = tracks.iter()
            .map(|tr| tr.iter())
            .collect();

        let mut upcoming_midi_ev : Vec<Option<TrackEvent<'a>>> = Vec::new();
        for it in track_iters.iter_mut() {
            upcoming_midi_ev.push(it.next().copied());
        }

        self.handle_gui_events();

        let mut iteration_start;
        let mut notes_currently_on = HashMap::new();
        let mut sleep_drift;
        let mut sleep_duration = tick_duration;
        let mut ticks_elapsed : u28 = 0.into();
        let mut previously_paused = false;
        let mut tracks_ended;

        'main_loop: loop {
            iteration_start = Instant::now();
            self.handle_gui_events();
            if !self.running {
                break 'main_loop;
            }
            if self.paused {
                thread::sleep(PAUSE_SLEEP_INTERVAL);
                previously_paused = true;
                continue;
            } else if previously_paused {
                self.play_notes(&notes_currently_on);
                previously_paused = false;
            }

            tracks_ended = true;
            for i in 0..upcoming_midi_ev.len() {
                if let Some(ref mut ev) = upcoming_midi_ev[i] {
                    tracks_ended = false;
                    if ev.delta <= ticks_elapsed {
                        self.handle_midi_event(ev.kind,
                                               &mut notes_currently_on,
                                               &mut tick_duration,
                                               timing);
                        upcoming_midi_ev[i] = track_iters[i].next().copied();
                    } else {
                        ev.delta -= ticks_elapsed;
                    }
                }
            }
            if tracks_ended {
                break 'main_loop;
            }

            thread::sleep(sleep_duration);

            sleep_drift = iteration_start.elapsed();
            ticks_elapsed = 0.into();
            while sleep_drift > tick_duration {
                sleep_drift -= tick_duration;
                ticks_elapsed += 1.into();
            }
            sleep_duration = tick_duration - sleep_drift;

            // might happen if we slept just a bit shy of 1 tick;
            // still consider it 1 tick
            if ticks_elapsed == 0 {
                ticks_elapsed = 1.into();
            }
        }
    }

    fn handle_midi_event<'a>(&mut self,
                             ev_kind: TrackEventKind<'a>,
                             notes_currently_on: &mut HashMap<u7, u7>,
                             tick_duration: &mut Duration,
                             midi_timing: Timing) {
        match ev_kind {
            TrackEventKind::Midi {
                channel: _,
                message: MidiMessage::NoteOn { key, vel }
            } => {
                if vel == 0 {
                    // if velocity was set to zero, remove note instead
                    notes_currently_on.remove(&key);
                } else if !notes_currently_on.contains_key(&key) {
                    notes_currently_on.insert(key, vel);
                    self.play_notes(notes_currently_on);
                }
            },
            TrackEventKind::Midi {
                channel: _,
                message: MidiMessage::NoteOff { key, vel:_ }
            } => {
                notes_currently_on.remove(&key);
                self.play_notes(notes_currently_on);
            },
            TrackEventKind::Meta(MetaMessage::Tempo(microsec_per_quarter)) => {
                *tick_duration = get_tick_duration(
                    midi_timing,
                    microsec_per_quarter);
            },
            _ => {},
        }
    }

    fn play_notes(&mut self, notes_midi: &HashMap<u7, u7>) {
        // TODO: take velocity into account
        let notes : Vec<f64> = notes_midi.keys()
            .map(|num| midi_number_to_freq(*num))
            .collect();
        // TODO take `wave_is_cosine` into account
        self.gui.draw_square_waves(&notes);
    }
}

fn get_tick_duration(timing: Timing, microsec_per_quarter: u24) -> Duration {
    let tick_microsec = match timing {
        // See <https://majicdesigns.github.io/MD_MIDIFile/page_timing.html>
        // for an explanation on the Metrical MIDI timing.
        Timing::Metrical(ticks_per_quarter) =>
            (microsec_per_quarter.as_int() as f32) / (ticks_per_quarter.as_int() as f32),
        Timing::Timecode(frames_per_sec, subframes) =>
            1_000_000.0 / frames_per_sec.as_f32() / (subframes as f32),
    };
    Duration::from_micros(tick_microsec as u64)
}

fn midi_number_to_freq(num: u7) -> f64 {
    match num.as_int() {
        0 => 8.175799,
        12 => 16.35160,
        24 => 32.70320,
        36 => 65.40639,
        48 => 130.8128,
        60 => 261.6256,
        72 => 523.2511,
        84 => 1046.502,
        96 => 2093.005,
        108 => 4186.009,
        120 => 8372.018,

        1 => 8.661957,
        13 => 17.32391,
        25 => 34.64783,
        37 => 69.29566,
        49 => 138.5913,
        61 => 277.1826,
        73 => 554.3653,
        85 => 1108.731,
        97 => 2217.461,
        109 => 4434.922,
        121 => 8869.844,

        2 => 9.177024,
        14 => 18.35405,
        26 => 36.70810,
        38 => 73.41619,
        50 => 146.8324,
        62 => 293.6648,
        74 => 587.3295,
        86 => 1174.659,
        98 => 2349.318,
        110 => 4698.636,
        122 => 9397.273,

        3 => 9.722718,
        15 => 19.44544,
        27 => 38.89087,
        39 => 77.78175,
        51 => 155.5635,
        63 => 311.1270,
        75 => 622.2540,
        87 => 1244.508,
        99 => 2489.016,
        111 => 4978.032,
        123 => 9956.063,

        4 => 10.30086,
        16 => 20.60172,
        28 => 41.20344,
        40 => 82.40689,
        52 => 164.8138,
        64 => 329.6276,
        76 => 659.2551,
        88 => 1318.510,
        100 => 2637.020,
        112 => 5274.041,
        124 => 10548.08,

        5 => 10.91338,
        17 => 21.82676,
        29 => 43.65353,
        41 => 87.30706,
        53 => 174.6141,
        65 => 349.2282,
        77 => 698.4565,
        89 => 1396.913,
        101 => 2793.826,
        113 => 5587.652,
        125 => 11175.30,

        6 => 11.56233,
        18 => 23.12465,
        30 => 46.24930,
        42 => 92.49861,
        54 => 184.9972,
        66 => 369.9944,
        78 => 739.9888,
        90 => 1479.978,
        102 => 2959.955,
        114 => 5919.911,
        126 => 11839.82,

        7 => 12.24986,
        19 => 24.49971,
        31 => 48.99943,
        43 => 97.99886,
        55 => 195.9977,
        67 => 391.9954,
        79 => 783.9909,
        91 => 1567.982,
        103 => 3135.963,
        115 => 6271.927,
        127 => 12543.85,

        8 => 12.97827,
        20 => 25.95654,
        32 => 51.91309,
        44 => 103.8262,
        56 => 207.6523,
        68 => 415.3047,
        80 => 830.6094,
        92 => 1661.219,
        104 => 3322.438,
        116 => 6644.875,

        9 => 13.75000,
        21 => 27.50000,
        33 => 55.00000,
        45 => 110.0000,
        57 => 220.0000,
        69 => 440.0000,
        81 => 880.0000,
        93 => 1760.000,
        105 => 3520.000,
        117 => 7040.000,

        10 => 14.56762,
        22 => 29.13524,
        34 => 58.27047,
        46 => 116.5409,
        58 => 233.0819,
        70 => 466.1638,
        82 => 932.3275,
        94 => 1864.655,
        106 => 3729.310,
        118 => 7458.620,

        11 => 15.43385,
        23 => 30.86771,
        35 => 61.73541,
        47 => 123.4708,
        59 => 246.9417,
        71 => 493.8833,
        83 => 987.7666,
        95 => 1975.533,
        107 => 3951.066,
        119 => 7902.133,

        invalid => panic!("error: invalid value for u7: {}", invalid),
    }
}
