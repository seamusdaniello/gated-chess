use macroquad::prelude::*;

pub struct GateTextures {
    pub tex_vector: Vec<Texture2D>,
}

impl GateTextures {
    pub async fn load() -> Self {
        let gate_images = [
            "images/gates/gate-1.png",
            "images/gates/gate-2.png",
            "images/gates/gate-3.png",
            "images/gates/gate-4.png",
            "images/gates/gate-5.png",
            "images/gates/gate-6.png",
            "images/gates/gate-7.png",
            "images/gates/gate-8.png",
        ];

        let mut tex_vector = Vec::new();

        for file in gate_images.iter() {
            let tex = load_texture(file).await.unwrap();
            tex.set_filter(FilterMode::Nearest);
            tex_vector.push(tex);
        }

        Self { tex_vector }
    }
}

