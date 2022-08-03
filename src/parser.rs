// Copyright (C) 2022 Luana Martins Barbosa
//
// This file is part of tempest-lcd.
// tempest-lcd is free software, released under the
// GNU Public License, version 2 only.
// See COPYING.txt.

use std::time::Duration;

pub struct Note {
    pub freq: Option<f64>, // in Hz; None means the note is a rest
    pub duration: Duration,
}

pub fn parse_file_contents(file_contents: &str) -> Vec<Note> {
    let mut lines = file_contents.lines();
    let bpm = lines.next()
        .expect("empty first line, should have BPM value")
        .parse::<f64>()
        .unwrap_or_else(|e| panic!("failed to parse BPM (1st line) as f64: {}", e));

    lines.filter(|line| !line.is_empty())
        .flat_map(|line| line.split(' '))
        .filter(|word| !word.is_empty())
        .map(|word| parse_note(bpm, word))
        .collect()
}

fn parse_note(bpm: f64, note: &str) -> Note {
    let mut chars = note.chars();
    // we've filtered for word.is_empty() so this should have at least one char
    let note_name = chars.next().unwrap();
    let semitone_offset = semitone_offset_from_la(note_name);
    let freq = if let Some(mut semitone) = semitone_offset {
        if let Some('#') = chars.clone().next() {
            chars.next(); // skip sharp
            semitone += 1;
        }
        let octave_char = chars.next()
            .expect("missing octave number in note!");
        let note_octave = octave_char.to_digit(10)
            .expect(&format!("note octave should be a digit: got {}", octave_char));

        // counted from A0 = 0. This is NOT the same as the MIDI number.
        let key_number = ((note_octave as i32) * 12 + semitone as i32) as f64; 
        let note_freq = 440.0 * 2.0_f64.powf(key_number / 12.0 - 4.0);
        Some(note_freq)
    } else {
        None
    };
    let duration_factor = match chars.next() {
        Some(c) => duration_factor_from_quarter(c),
        None => 1.0, // notes are quarters by default
    };
    let duration_ms = (duration_factor * 60_000.0 / bpm).round();
    Note {
        freq,
        duration: Duration::from_millis(duration_ms as u64),
    }
}

fn semitone_offset_from_la(note_name: char) -> Option<i8> {
    match note_name.to_ascii_lowercase() {
        'r' => None, // rest
        'b' => Some(2),
        'a' => Some(0),
        'g' => Some(-2),
        'f' => Some(-4),
        'e' => Some(-5),
        'd' => Some(-7),
        'c' => Some(-9),
        chr => panic!("unknown note name `{}'", chr),
    }
}

fn duration_factor_from_quarter(note_dur: char) -> f64 {
    match note_dur.to_ascii_lowercase() {
        'w' => 4.0, // whole
        'h' => 2.0, // half
        'q' => 1.0, // quarter
        'e' => 0.5, // eighth
        's' => 0.25, // sixteenth
        't' => 0.125, // thirthy-second
        chr => panic!("unknown note duration name `{}'", chr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FREQ_TOLERANCE: f64 = 0.01;

    fn test_note(note: &Note, expected_freq: f64, expected_duration: Duration) {
        let note_freq = note.freq.unwrap();

        assert_eq!(note.duration, expected_duration);
        if (note_freq - expected_freq).abs() >= FREQ_TOLERANCE {
            panic!("note_freq = {}, expected_freq = {}", note_freq, expected_freq);
        }
    }

    #[test]
    fn parse_a4() {
        let note = parse_note(120.0, "a4");
        let expected_freq = 440.0;
        let expected_duration = Duration::from_millis(500);
        test_note(&note, expected_freq, expected_duration);
    }

    #[test]
    fn parse_c3h() {
        let note = parse_note(150.0, "c3h");
        let expected_freq = 130.81;
        let expected_duration = Duration::from_millis(800);
        test_note(&note, expected_freq, expected_duration);
    }

    #[test]
    fn parse_gsharp5e() {
        let note = parse_note(125.0, "G#5e");
        let expected_freq = 830.61;
        let expected_duration = Duration::from_millis(240);
        test_note(&note, expected_freq, expected_duration);
    }

    #[test]
    fn test_parse_contents1() {
        let contents = "150\na2 c#3 e3";
        let notes = parse_file_contents(contents);
        let expected_duration = Duration::from_millis(400);
        assert_eq!(notes.len(), 3);
        test_note(&notes[0], 110.0, expected_duration);
        test_note(&notes[1], 138.59, expected_duration);
        test_note(&notes[2], 164.81, expected_duration);
    }

    #[test]
    fn test_parse_contents2() {
        let contents = "80\nc5h e4w\ng3q f5e";
        let notes = parse_file_contents(contents);
        assert_eq!(notes.len(), 4);
        test_note(&notes[0], 523.25, Duration::from_millis(1500));
        test_note(&notes[1], 329.63, Duration::from_millis(3000));
        test_note(&notes[2], 196.0, Duration::from_millis(750));
        test_note(&notes[3], 698.46, Duration::from_millis(375));
    }
}
