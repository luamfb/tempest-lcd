use std::{
    time::{Duration, Instant},
    thread,
};
use sdl2::pixels::Color;

use crate::{
    legacy_parser::Note,
    gui::Gui,
};

const SLEEP_INTERVAL: Duration = Duration::from_millis(5);

pub struct LegacyPlayer {
    gui: Gui,
    running: bool,
    paused: bool,
    wave_is_cosine: bool,
}

impl LegacyPlayer {
    pub fn create(gui: Gui, wave_is_cosine: bool) -> Self {
        LegacyPlayer {
            gui,
            paused: false,
            running: false,
            wave_is_cosine,
        }
    }

    // this is only necessary because we can't pass mutable references
    // of our fields to Gui directly
    pub fn handle_events(&mut self) {
        let mut running = self.running;
        let mut paused = self.paused;

        self.gui.handle_events(&mut running, &mut paused);

        self.running = running;
        self.paused = paused;
    }

    pub fn run(&mut self, notes: &Vec<Note>) {
        if self.running || notes.len() == 0 {
            return;
        }

        self.running = true;
        let mut cur_index = 0;
        let mut time_playing_cur_note = Duration::ZERO;
        let mut iteration_start;
        let mut previously_paused = false;

        // Special care must be taken to ensure first note is actually played.
        // We must handle events before playing it as well, because there's
        // a good chance we'll receive some event (like Shown or FocusGained)
        // that would cause the screen to go blank.
        self.handle_events();
        self.play_note(&notes[0]);

        'main_loop: loop {
            iteration_start = Instant::now();
            let cur_note = &notes[cur_index];
            self.handle_events();
            if !self.running {
                break 'main_loop;
            }
            if self.paused {
                thread::sleep(SLEEP_INTERVAL);
                previously_paused = true;
                continue;
            } else if previously_paused {
                self.play_note(cur_note);
                previously_paused = false;
            }

            if time_playing_cur_note > cur_note.duration {
                time_playing_cur_note = Duration::ZERO;
                cur_index += 1;
                if cur_index >= notes.len() {
                    break 'main_loop;
                }
                let new_note = &notes[cur_index];
                self.play_note(new_note);
            }
            thread::sleep(SLEEP_INTERVAL);
            time_playing_cur_note += iteration_start.elapsed();
        }
        self.running = false;
    }

    fn play_note(&mut self, new_note: &Note) {
        match new_note.freq {
            // note
            Some(freq) => {
                if self.wave_is_cosine {
                    self.gui.draw_single_cosine_wave(freq);
                } else {
                    self.gui.draw_single_square_wave(freq);
                }
            },
            None => self.gui.clear_and_present(Color::BLACK), // rest
        };
    }
}
