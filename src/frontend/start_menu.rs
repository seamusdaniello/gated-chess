use macroquad::prelude::*;

use crate::network::SessionConfig;
use crate::time_control::{STANDARD_TIME_CONTROLS, TimeControl};

pub struct LaunchConfig {
    pub session: SessionConfig,
    pub time_control: TimeControl,
}

enum StartStep {
    ModeSelect,
    TimeSelect(SessionConfig),
}

pub struct StartMenu {
    address_input: String,
    step: StartStep,
}

impl StartMenu {
    pub fn new() -> Self {
        Self {
            address_input: "127.0.0.1:4000".to_string(),
            step: StartStep::ModeSelect,
        }
    }

    pub fn draw(&mut self) -> Option<LaunchConfig> {
        match &self.step {
            StartStep::ModeSelect => self.draw_mode_select(),
            StartStep::TimeSelect(session) => self.draw_time_select(session.clone()),
        }
    }

    fn draw_mode_select(&mut self) -> Option<LaunchConfig> {
        self.handle_text_input();

        let menu_width = 400.0;
        let menu_height = 640.0;
        let menu_x = (screen_width() - menu_width) / 2.0;
        let menu_y = (screen_height() - menu_height) / 2.0;

        // Draw background
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(20, 20, 30, 255),
        );

        // Draw menu panel
        draw_rectangle(
            menu_x,
            menu_y,
            menu_width,
            menu_height,
            Color::from_rgba(40, 40, 50, 255),
        );
        draw_rectangle_lines(menu_x, menu_y, menu_width, menu_height, 4.0, GOLD);

        // Draw title
        draw_text("GATED CHESS", menu_x + 60.0, menu_y + 80.0, 48.0, WHITE);

        // Draw buttons
        let button_width = 300.0;
        let button_height = 60.0;
        let button_x = menu_x + (menu_width - button_width) / 2.0;

        // Local button
        let start_button_y = menu_y + 180.0;
        let start_hovered =
            Self::is_button_hovered(button_x, start_button_y, button_width, button_height);
        Self::draw_button(
            "Local Game",
            button_x,
            start_button_y,
            button_width,
            button_height,
            start_hovered,
        );

        // Host button
        let instructions_button_y = menu_y + 260.0;
        let instructions_hovered =
            Self::is_button_hovered(button_x, instructions_button_y, button_width, button_height);
        Self::draw_button(
            "Host Online",
            button_x,
            instructions_button_y,
            button_width,
            button_height,
            instructions_hovered,
        );

        // Join button
        let quit_button_y = menu_y + 340.0;
        let quit_hovered =
            Self::is_button_hovered(button_x, quit_button_y, button_width, button_height);
        Self::draw_button(
            "Join Online",
            button_x,
            quit_button_y,
            button_width,
            button_height,
            quit_hovered,
        );

        let find_button_y = menu_y + 420.0;
        let find_hovered =
            Self::is_button_hovered(button_x, find_button_y, button_width, button_height);
        Self::draw_button(
            "Find Match",
            button_x,
            find_button_y,
            button_width,
            button_height,
            find_hovered,
        );

        let input_y = menu_y + 510.0;
        draw_text("Address", button_x, input_y - 12.0, 24.0, LIGHTGRAY);
        draw_rectangle(
            button_x,
            input_y,
            button_width,
            52.0,
            Color::from_rgba(26, 26, 34, 255),
        );
        draw_rectangle_lines(button_x, input_y, button_width, 52.0, 2.0, WHITE);
        draw_text(
            &self.address_input,
            button_x + 12.0,
            input_y + 34.0,
            28.0,
            WHITE,
        );
        draw_text(
            "Use host IP:port for online play",
            button_x - 6.0,
            input_y + 86.0,
            20.0,
            GRAY,
        );

        // Check for clicks
        if is_mouse_button_pressed(MouseButton::Left) {
            if start_hovered {
                self.step = StartStep::TimeSelect(SessionConfig::Local);
            } else if instructions_hovered {
                self.step = StartStep::TimeSelect(SessionConfig::Host {
                    bind_addr: self.address_input.trim().to_string(),
                });
            } else if quit_hovered {
                return Some(LaunchConfig {
                    session: SessionConfig::Join {
                        server_addr: self.address_input.trim().to_string(),
                    },
                    time_control: STANDARD_TIME_CONTROLS[4],
                });
            } else if find_hovered {
                self.step = StartStep::TimeSelect(SessionConfig::FindMatch {
                    addr: self.address_input.trim().to_string(),
                });
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        None
    }

    fn draw_time_select(&mut self, session: SessionConfig) -> Option<LaunchConfig> {
        let menu_width = 620.0;
        let menu_height = 560.0;
        let menu_x = (screen_width() - menu_width) / 2.0;
        let menu_y = (screen_height() - menu_height) / 2.0;

        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(20, 20, 30, 255),
        );
        draw_rectangle(
            menu_x,
            menu_y,
            menu_width,
            menu_height,
            Color::from_rgba(40, 40, 50, 255),
        );
        draw_rectangle_lines(menu_x, menu_y, menu_width, menu_height, 4.0, GOLD);

        draw_text(
            "Choose Time Control",
            menu_x + 92.0,
            menu_y + 70.0,
            42.0,
            WHITE,
        );
        draw_text(
            "Standard chess formats, including increment",
            menu_x + 84.0,
            menu_y + 108.0,
            24.0,
            LIGHTGRAY,
        );

        let columns = 2;
        let button_width = 230.0;
        let button_height = 54.0;
        let spacing_x = 28.0;
        let spacing_y = 18.0;
        let grid_x = menu_x + 64.0;
        let grid_y = menu_y + 150.0;

        for (index, time_control) in STANDARD_TIME_CONTROLS.iter().enumerate() {
            let col = index % columns;
            let row = index / columns;
            let x = grid_x + col as f32 * (button_width + spacing_x);
            let y = grid_y + row as f32 * (button_height + spacing_y);
            let hovered = Self::is_button_hovered(x, y, button_width, button_height);
            Self::draw_button(
                &time_control.label(),
                x,
                y,
                button_width,
                button_height,
                hovered,
            );

            if hovered && is_mouse_button_pressed(MouseButton::Left) {
                return Some(LaunchConfig {
                    session: session.clone(),
                    time_control: *time_control,
                });
            }
        }

        let back_x = menu_x + 64.0;
        let back_y = menu_y + menu_height - 56.0;
        let back_width = 160.0;
        let back_height = 48.0;
        let back_hovered = Self::is_button_hovered(back_x, back_y, back_width, back_height);
        Self::draw_button(
            "Back",
            back_x,
            back_y,
            back_width,
            back_height,
            back_hovered,
        );

        if back_hovered && is_mouse_button_pressed(MouseButton::Left) {
            self.step = StartStep::ModeSelect;
        }

        if is_key_pressed(KeyCode::Escape) {
            self.step = StartStep::ModeSelect;
        }

        None
    }

    fn draw_button(text: &str, x: f32, y: f32, width: f32, height: f32, hovered: bool) {
        let color = if hovered {
            Color::from_rgba(80, 80, 90, 255)
        } else {
            Color::from_rgba(60, 60, 70, 255)
        };

        let border_color = if hovered { GOLD } else { WHITE };

        draw_rectangle(x, y, width, height, color);
        draw_rectangle_lines(x, y, width, height, 2.0, border_color);

        // Center text
        let text_size = 32.0;
        let text_width = measure_text(text, None, text_size as u16, 1.0).width;
        let text_x = x + (width - text_width) / 2.0;
        let text_y = y + (height + text_size) / 2.0 - 5.0;

        draw_text(text, text_x, text_y, text_size, WHITE);
    }

    fn is_button_hovered(x: f32, y: f32, width: f32, height: f32) -> bool {
        let (mx, my) = mouse_position();
        mx >= x && mx <= x + width && my >= y && my <= y + height
    }

    fn handle_text_input(&mut self) {
        while let Some(ch) = get_char_pressed() {
            if !ch.is_control() {
                self.address_input.push(ch);
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            self.address_input.pop();
        }
    }
}
