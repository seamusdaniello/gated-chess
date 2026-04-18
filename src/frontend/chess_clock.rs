use macroquad::prelude::*;

use crate::pieces::Color as PieceColor;
use crate::time_control::TimeControl;

pub struct ChessClock {
    white_remaining: f64,
    black_remaining: f64,
    increment_seconds: f64,
    last_tick_at: f64,
    label: String,
}

impl ChessClock {
    pub fn new(time_control: TimeControl, now: f64) -> Self {
        let initial = time_control.initial_seconds as f64;
        Self {
            white_remaining: initial,
            black_remaining: initial,
            increment_seconds: time_control.increment_seconds as f64,
            last_tick_at: now,
            label: time_control.label(),
        }
    }

    pub fn reconfigure(&mut self, time_control: TimeControl, now: f64) {
        let initial = time_control.initial_seconds as f64;
        self.white_remaining = initial;
        self.black_remaining = initial;
        self.increment_seconds = time_control.increment_seconds as f64;
        self.last_tick_at = now;
        self.label = time_control.label();
    }

    pub fn update(
        &mut self,
        active_color: PieceColor,
        now: f64,
        should_run: bool,
    ) -> Option<PieceColor> {
        if !should_run {
            self.last_tick_at = now;
            return None;
        }

        let elapsed = (now - self.last_tick_at).max(0.0);
        self.last_tick_at = now;

        match active_color {
            PieceColor::White => {
                self.white_remaining = (self.white_remaining - elapsed).max(0.0);
                if self.white_remaining <= 0.0 {
                    return Some(PieceColor::Black);
                }
            }
            PieceColor::Black => {
                self.black_remaining = (self.black_remaining - elapsed).max(0.0);
                if self.black_remaining <= 0.0 {
                    return Some(PieceColor::White);
                }
            }
        }

        None
    }

    pub fn apply_increment(&mut self, mover: PieceColor, now: f64) {
        match mover {
            PieceColor::White => self.white_remaining += self.increment_seconds,
            PieceColor::Black => self.black_remaining += self.increment_seconds,
        }
        self.last_tick_at = now;
    }

    pub fn draw(&self, active_color: PieceColor) {
        let panel_x = 20.0;
        let panel_width = 190.0;
        let panel_height = 110.0;
        let top_y = 120.0;
        let bottom_y = screen_height() - panel_height - 120.0;

        draw_text("Clock", panel_x + 18.0, top_y - 50.0, 24.0, LIGHTGRAY);
        draw_text(&self.label, panel_x + 18.0, top_y - 20.0, 30.0, GOLD);

        self.draw_clock_panel(
            panel_x,
            top_y,
            panel_width,
            panel_height,
            "White",
            self.white_remaining,
            active_color == PieceColor::White,
        );
        self.draw_clock_panel(
            panel_x,
            bottom_y,
            panel_width,
            panel_height,
            "Black",
            self.black_remaining,
            active_color == PieceColor::Black,
        );
    }

    fn draw_clock_panel(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        label: &str,
        remaining: f64,
        active: bool,
    ) {
        draw_rectangle(
            x,
            y,
            width,
            height,
            if active {
                Color::from_rgba(58, 68, 54, 240)
            } else {
                Color::from_rgba(32, 32, 40, 230)
            },
        );
        draw_rectangle_lines(x, y, width, height, 3.0, if active { GOLD } else { WHITE });
        draw_text(label, x + 16.0, y + 30.0, 28.0, LIGHTGRAY);
        draw_text(&format_time(remaining), x + 16.0, y + 80.0, 42.0, WHITE);
    }
}

fn format_time(seconds: f64) -> String {
    let total = seconds.ceil().max(0.0) as u32;
    let minutes = total / 60;
    let secs = total % 60;
    format!("{:02}:{:02}", minutes, secs)
}
