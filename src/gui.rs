// Copyright (C) 2022-2025 Luana Martins Barbosa
//
// This file is part of tempest-lcd.
// tempest-lcd is free software, released under the
// GNU Public License, version 2 only.
// See COPYING.txt.

use std::{
    f64::consts,
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
}

impl Gui {
    pub fn create(horiz_refresh_rate: f64) -> Self {
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
        }
    }

    pub fn draw_single_square_wave(&mut self, note_freq: f64) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);
        for y in 0..self.res_y {
            // approx time when arriving at this row
            let t = (y as f64) / self.horiz_refresh_rate;
            // Note that `cosine_is_positive` is true if and only if
            //      cos(2pi*t*note_freq) > 0
            //
            // PROOF: let's abbreviate note_freq to f.
            // Since t > 0 and f > 0, the cast to i64 works as floor(), thus
            //      cosine_is_positive <==> floor(2tf) mod 2 == 0
            //      <==> floor(2tf) == 2n for some integer n
            //      <==> 2n <= 2tf < 2n + 1
            //      <==> 2pi*n <= 2pi*tf < 2pi*n + pi
            // In this interval, for any integer n, `cos` is monotonically
            // decreasing, and so
            //      cosine_is_positive
            //      <==> cos(2pi*n + pi) < cos(2pi*tf) <= cos(2pi*n)
            //      <==> 0 < cos(2pi*tf) <= 1                           QED
            //
            let cosine_is_positive = ((2.0 * t * note_freq) as i64) % 2 == 0;
            if cosine_is_positive {
                let origin = Point::new(0, y);
                let dest = Point::new(self.res_x, y);
                self.canvas.draw_line(origin, dest)
                    .unwrap_or_else(|e| panic!("failed to draw line: {}", e));
            }
        }
        self.canvas.present();
    }

    pub fn draw_single_cosine_wave(&mut self, note_freq: f64) {
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

    pub fn draw_square_waves(&mut self, freqs: &[f64]) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        for y in 0..self.res_y {
            // approx time when arriving at this row
            let t = (y as f64) / self.horiz_refresh_rate;
            let mut level : i32 = 0;
            for note_freq in freqs {
                // Same logic as in `draw_single_square_wave`
                let cosine_is_positive = ((2.0 * t * note_freq) as i64) % 2 == 0;
                if cosine_is_positive {
                    level += 1;
                } else {
                    level -= 1;
                }
            }
            let level_norm = ((level as f64) + (freqs.len() as f64)) / (freqs.len() as f64);
            let color_comp = (level_norm * 127.5) as u8;
            let color = Color::RGB(color_comp, color_comp, color_comp);
            self.canvas.set_draw_color(color);
            let origin = Point::new(0, y);
            let dest = Point::new(self.res_x, y);
            self.canvas.draw_line(origin, dest)
                .unwrap_or_else(|e| panic!("failed to draw line: {}", e));

        }
        self.canvas.present();
    }

    pub fn handle_events(&mut self, running: &mut bool, paused: &mut bool) {
        for ev in self.event_pump.poll_iter() {
            match ev {
                Event::Quit {..} => *running = false,
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Close => *running = false,
                    WindowEvent::FocusLost => {
                        clear_and_present(&mut self.canvas, Color::BLACK);
                        *paused = true;
                    },
                    WindowEvent::Shown
                        | WindowEvent::Exposed
                        | WindowEvent::FocusGained => {
                            clear_and_present(&mut self.canvas, Color::BLACK);
                    },
                    _ => {},
                },
                Event::KeyDown { keycode: Some(key), .. } => match key {
                    Keycode::Q => *running = false,
                    Keycode::P | Keycode::Space => {
                        if !*paused {
                            clear_and_present(&mut self.canvas, Color::BLACK);
                            *paused = true;
                        } else {
                            *paused = false;
                            // since we don't have access to the notes,
                            // let the player re-render them.
                        }
                    },
                    _ => {},
                },
                _ => {},
            }
        }
    }

    pub fn clear_and_present(&mut self, clear_color: Color) {
        clear_and_present(&mut self.canvas, clear_color);
    }
}

fn clear_and_present(canvas: &mut WindowCanvas, clear_color: Color) {
    canvas.set_draw_color(clear_color);
    canvas.clear();
    canvas.present();
}
