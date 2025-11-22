use macroquad::prelude::*;

pub struct BoardFrame {
    texture: Texture2D,
}

impl BoardFrame {
    pub async fn load(path: &str) -> Self {
        let tex = load_texture(path).await.unwrap();
        tex.set_filter(FilterMode::Nearest); // optional depending on pixel art style
        Self { texture: tex }
    }

    pub fn draw(&self, tile_size: f32) {
        let board_size = tile_size * 8.0;

        // Top and bottom
        for col in 0..8 {
            draw_texture_ex(
                &self.texture,
                col as f32 * tile_size,
                -tile_size * 0.2, // slight overhang
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(tile_size, tile_size)),
                    ..Default::default()
                },
            );

            draw_texture_ex(
                &self.texture,
                col as f32 * tile_size,
                board_size - tile_size * 0.8, // bottom edge
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(tile_size, tile_size)),
                    ..Default::default()
                },
            );
        }

        // Left and right
        for row in 0..8 {
            draw_texture_ex(
                &self.texture,
                -tile_size * 0.2,
                row as f32 * tile_size,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(tile_size, tile_size)),
                    ..Default::default()
                },
            );

            draw_texture_ex(
                &self.texture,
                board_size - tile_size * 0.8,
                row as f32 * tile_size,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(tile_size, tile_size)),
                    ..Default::default()
                },
            );
        }
    }
}