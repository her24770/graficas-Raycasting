use std::collections::HashMap;

/// Imagen decodificada a memoria, en el mismo formato de color que usa el
/// framebuffer (0xRRGGBB), para poder muestrearla píxel a píxel sin
/// reconvertir en cada frame.
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pixels: Vec<u32>,
}

impl Texture {
    fn from_path(path: &str) -> Option<Texture> {
        let img = image::open(path).ok()?.to_rgba8();
        let (width, height) = img.dimensions();

        let pixels = img
            .pixels()
            .map(|p| {
                let [r, g, b, _a] = p.0;
                ((r as u32) << 16) | ((g as u32) << 8) | b as u32
            })
            .collect();

        Some(Texture { width, height, pixels })
    }

    /// Muestrea la textura en coordenadas normalizadas (0.0..1.0, 0.0..1.0).
    pub fn sample(&self, u: f32, v: f32) -> u32 {
        let x = ((u.clamp(0.0, 0.999) * self.width as f32) as u32).min(self.width - 1);
        let y = ((v.clamp(0.0, 0.999) * self.height as f32) as u32).min(self.height - 1);
        self.pixels[(y * self.width + x) as usize]
    }
}

/// Mapa de carácter de pared -> textura cargada. Si falta el archivo de un
/// material, ese carácter simplemente no queda en el mapa y el renderer usa
/// su color plano de respaldo (nunca crashea por un asset ausente).
pub struct TextureAtlas {
    by_wall_char: HashMap<char, Texture>,
}

impl TextureAtlas {
    pub fn load() -> Self {
        let mut by_wall_char = HashMap::new();

        for wall_char in ['1', '2', '3', '4'] {
            let base = format!("assets/textures/wall_{wall_char}");
            let candidates = [
                format!("{base}.png"),
                format!("{base}.jpg"),
                format!("{base}.jpeg"),
            ];

            let texture = candidates.iter().find_map(|path| Texture::from_path(path));

            match texture {
                Some(tex) => {
                    by_wall_char.insert(wall_char, tex);
                }
                None => {
                    eprintln!(
                        "[texturas] no se encontró textura para '{wall_char}' en assets/textures/ (se usa color plano de respaldo)"
                    );
                }
            }
        }

        TextureAtlas { by_wall_char }
    }

    pub fn get(&self, wall_char: char) -> Option<&Texture> {
        self.by_wall_char.get(&wall_char)
    }
}
