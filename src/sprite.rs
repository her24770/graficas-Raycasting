use nalgebra_glm::Vec2;

/// Un frame de sprite: imagen pequeña con transparencia (`None` = píxel
/// transparente), generada por código en vez de cargada desde archivo.
pub struct SpriteFrame {
    width: usize,
    height: usize,
    pixels: Vec<Option<u32>>,
}

impl SpriteFrame {
    /// Muestrea en coordenadas normalizadas (0.0..1.0, 0.0..1.0).
    pub fn sample(&self, u: f32, v: f32) -> Option<u32> {
        let x = ((u.clamp(0.0, 0.999) * self.width as f32) as usize).min(self.width - 1);
        let y = ((v.clamp(0.0, 0.999) * self.height as f32) as usize).min(self.height - 1);
        self.pixels[y * self.width + x]
    }
}

pub struct Sprite {
    pub pos: Vec2,
    frames: Vec<SpriteFrame>,
    frame_time: f32,
    elapsed: f32,
    current: usize,
}

impl Sprite {
    pub fn torch(pos: Vec2) -> Self {
        Sprite {
            pos,
            frames: generate_torch_frames(),
            frame_time: 0.15,
            elapsed: 0.0,
            current: 0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        while self.elapsed >= self.frame_time {
            self.elapsed -= self.frame_time;
            self.current = (self.current + 1) % self.frames.len();
        }
    }

    pub fn current_frame(&self) -> &SpriteFrame {
        &self.frames[self.current]
    }
}

/// Genera unos pocos frames de una llama simple (forma triangular, ancha en
/// la base y angosta en la punta), variando el ancho por frame para dar
/// sensación de parpadeo. No depende de ningún archivo externo.
fn generate_torch_frames() -> Vec<SpriteFrame> {
    const SIZE: usize = 24;
    const FLICKER_FACTORS: [f32; 4] = [1.0, 0.8, 1.15, 0.9];

    FLICKER_FACTORS
        .iter()
        .map(|&flicker| {
            let mut pixels = vec![None; SIZE * SIZE];
            let base_half_width = SIZE as f32 * 0.28 * flicker;

            for y in 0..SIZE {
                // t=0 en la punta (arriba), t=1 en la base (abajo).
                let t = y as f32 / (SIZE - 1) as f32;
                let half_width = base_half_width * t.powf(1.4);

                let r = 255u32;
                let g = (120.0 + 135.0 * (1.0 - t)) as u32;
                let b = (30.0 + 60.0 * (1.0 - t)) as u32;
                let color = (r << 16) | (g << 8) | b;

                for x in 0..SIZE {
                    let fx = x as f32 - SIZE as f32 / 2.0;
                    if fx.abs() <= half_width {
                        pixels[y * SIZE + x] = Some(color);
                    }
                }
            }

            SpriteFrame {
                width: SIZE,
                height: SIZE,
                pixels,
            }
        })
        .collect()
}
