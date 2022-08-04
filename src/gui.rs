// Copyright (C) 2022 Luana Martins Barbosa
//
// This file is part of tempest-lcd.
// tempest-lcd is free software, released under the
// GNU Public License, version 2 only.
// See COPYING.txt.

use std::{
    f64::consts,
    time::{Duration, Instant},
    thread,
};
use rand::Rng;
use rand_distr::StandardNormal;
use sdl2::{
    EventPump,
    Sdl,
    VideoSubsystem,
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
    rect::Point,
    render::WindowCanvas,
};
use crate::parser::Note;

const SLEEP_INTERVAL: Duration = Duration::from_millis(5);

pub struct Gui {
    // note: these two are never used directly, but must be held here to ensure
    // they're not dropped until after the GUI stopped.
    _sdl_context: Sdl,
    _video_subsys: VideoSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
    horiz_refresh_rate: f64,
    res_x: i32,
    res_y: i32,
    running: bool,
    paused: bool,
    wave_is_cosine: bool,
}

impl Gui {
    pub fn create(horiz_refresh_rate: f64, wave_is_cosine: bool) -> Self {
        let sdl_context = sdl2::init()
            .unwrap_or_else(|e| panic!("failed to initialize SDL2: {}", e));
        let video_subsys = sdl_context.video()
            .unwrap_or_else(|e| panic!("failed to initialize video subsystem: {}", e));

        let mut window_builder = video_subsys.window("tempest LCD", 0, 0);
        window_builder.fullscreen_desktop();
        window_builder.borderless();

        let window = window_builder.build()
            .unwrap_or_else(|e| panic!("failed to create window: {}", e));
        let (res_x_uint, res_y_uint) = window.size();

        let res_x: i32 = res_x_uint.try_into()
            .unwrap_or_else(|e| panic!("failed to convert X resolution {} to i32: {}", res_x_uint, e));
        let res_y: i32 = res_y_uint.try_into()
            .unwrap_or_else(|e| panic!("failed to convert Y resolution {} to i32: {}", res_y_uint, e));

        let mut canvas = window.into_canvas()
            .build()
            .unwrap_or_else(|e| panic!("failed to make renderer from window: {}", e));

        let event_pump = sdl_context.event_pump()
            .unwrap_or_else(|e| panic!("failed to get event pump: {}", e));

        clear_and_present(&mut canvas, Color::GRAY);

        Gui {
            _sdl_context: sdl_context,
            _video_subsys: video_subsys,
            canvas,
            event_pump,
            horiz_refresh_rate,
            res_x,
            res_y,
            paused: false,
            running: false,
            wave_is_cosine,
        }
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
                    self.draw_cosine_wave(freq);
                } else {
                    self.draw_square_wave(freq);
                }
            },
            None => clear_and_present(&mut self.canvas, Color::BLACK), // rest
        };
    }

    fn draw_square_wave(&mut self, note_freq: f64) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);
        for y in 0..self.res_y {
            // approx time when arriving at this row
            let t = (y as f64) / self.horiz_refresh_rate;
            if ((2.0 * t * note_freq) as i64) % 2 == 0 {
                let origin = Point::new(0, y);
                let dest = Point::new(self.res_x, y);
                self.canvas.draw_line(origin, dest)
                    .unwrap_or_else(|e| panic!("failed to draw line: {}", e));
            }
        }
        self.canvas.present();
    }

    fn draw_cosine_wave(&mut self, note_freq: f64) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        for y in 0..self.res_y {
            // approx time when arriving at this row
            let t = (y as f64) / self.horiz_refresh_rate;
            let dither: f64 = rand::thread_rng().sample(StandardNormal);
            // note: TAU = 2 * PI
            let raw_ampl = (consts::TAU * t * note_freq).cos();
            let color_component = (127.5 * (1.0 + raw_ampl) + dither) as u8;
            let color = Color::RGB(color_component,
                                   color_component,
                                   color_component);
            self.canvas.set_draw_color(color);
            let origin = Point::new(0, y);
            let dest = Point::new(self.res_x, y);
            self.canvas.draw_line(origin, dest)
                .unwrap_or_else(|e| panic!("failed to draw line: {}", e));
        }
        self.canvas.present();
    }

    fn handle_events(&mut self) {
        for ev in self.event_pump.poll_iter() {
            match ev {
                Event::Quit {..} => self.running = false,
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Close => self.running = false,
                    WindowEvent::FocusLost => {
                        clear_and_present(&mut self.canvas, Color::BLACK);
                        self.paused = true;
                    },
                    WindowEvent::Shown
                        | WindowEvent::Exposed
                        | WindowEvent::FocusGained => {
                            clear_and_present(&mut self.canvas, Color::BLACK);
                    },
                    _ => {},
                },
                Event::KeyDown { keycode: Some(key), .. } => match key {
                    Keycode::Q => self.running = false,
                    Keycode::P | Keycode::Space => {
                        if !self.paused {
                            clear_and_present(&mut self.canvas, Color::BLACK);
                            self.paused = true;
                        } else {
                            self.paused = false;
                            // since we don't have access to current note,
                            // let the main loop re-render it.
                        }
                    },
                    _ => {},
                },
                _ => {},
            }
        }
    }

}

fn clear_and_present(canvas: &mut WindowCanvas, clear_color: Color) {
    canvas.set_draw_color(clear_color);
    canvas.clear();
    canvas.present();
}
