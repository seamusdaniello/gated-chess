use macroquad::prelude::*;

pub struct StartMenu;

impl StartMenu {
    pub fn draw() -> bool {
        let menu_width = 400.0;
        let menu_height = 500.0;
        let menu_x = (screen_width() - menu_width) / 2.0;
        let menu_y = (screen_height() - menu_height) / 2.0;
        
        // Draw background
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(20, 20, 30, 255));
        
        // Draw menu panel
        draw_rectangle(menu_x, menu_y, menu_width, menu_height, Color::from_rgba(40, 40, 50, 255));
        draw_rectangle_lines(menu_x, menu_y, menu_width, menu_height, 4.0, GOLD);
        
        // Draw title
        draw_text("GATED CHESS", menu_x + 60.0, menu_y + 80.0, 48.0, WHITE);
        
        // Draw buttons
        let button_width = 300.0;
        let button_height = 60.0;
        let button_x = menu_x + (menu_width - button_width) / 2.0;
        
        // Start Game button
        let start_button_y = menu_y + 180.0;
        let start_hovered = Self::is_button_hovered(button_x, start_button_y, button_width, button_height);
        Self::draw_button("Start Game", button_x, start_button_y, button_width, button_height, start_hovered);
        
        // Instructions button
        let instructions_button_y = menu_y + 260.0;
        let instructions_hovered = Self::is_button_hovered(button_x, instructions_button_y, button_width, button_height);
        Self::draw_button("Instructions", button_x, instructions_button_y, button_width, button_height, instructions_hovered);
        
        // Quit button
        let quit_button_y = menu_y + 340.0;
        let quit_hovered = Self::is_button_hovered(button_x, quit_button_y, button_width, button_height);
        Self::draw_button("Quit", button_x, quit_button_y, button_width, button_height, quit_hovered);
        
        // Check for clicks
        if is_mouse_button_pressed(MouseButton::Left) {
            if start_hovered {
                return true; // Start the game
            } else if quit_hovered {
                std::process::exit(0);
            }
        }
        
        false // Stay in menu
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
}