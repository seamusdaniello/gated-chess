use macroquad::prelude::*;

pub async fn load_bishop_frames(color: &str) -> Vec<Texture2D> {
    let mut frames = Vec::new();

    for i in 0..12 {
        let path = format!("images/{}/bishop/bishop-{}.png", color, i);
        let tex = load_texture(&path).await.unwrap();
        tex.set_filter(FilterMode::Nearest);
        frames.push(tex);
    }

    frames
}