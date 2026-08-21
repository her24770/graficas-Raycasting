use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

const BUMP_SAMPLE_RATE: u32 = 44100;
const BUMP_DURATION_SECS: f32 = 0.15;

/// Sonido de "golpe" generado por código (sin ningún archivo de audio):
/// un tono corto que baja de frecuencia y se apaga rápido, como un "boink"
/// seco. Se recalcula al instante cada vez que se pide, así que nunca
/// depende de decodificar un archivo (esa era la causa del problema con
/// los efectos de sonido anteriores).
struct BumpTone {
    total_samples: usize,
    sample_index: usize,
}

impl BumpTone {
    fn new() -> Self {
        BumpTone {
            total_samples: (BUMP_SAMPLE_RATE as f32 * BUMP_DURATION_SECS) as usize,
            sample_index: 0,
        }
    }
}

impl Iterator for BumpTone {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.sample_index >= self.total_samples {
            return None;
        }

        let t = self.sample_index as f32 / BUMP_SAMPLE_RATE as f32;
        let progress = self.sample_index as f32 / self.total_samples as f32;

        // el tono arranca agudo y baja, para que suene a golpe seco, no a pitido plano.
        let freq = 220.0 - 130.0 * progress;
        // se apaga rápido (al cuadrado), no de forma lineal, para que se sienta corto.
        let envelope = (1.0 - progress).powf(2.0);

        self.sample_index += 1;
        Some((2.0 * PI * freq * t).sin() * envelope * 0.6)
    }
}

impl Source for BumpTone {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        BUMP_SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(BUMP_DURATION_SECS))
    }
}

/// Wrapper sobre rodio para música de fondo en loop + efectos de sonido
/// cortos. Nunca falla de forma fatal: si un archivo no existe o no se
/// puede decodificar, se loguea un aviso y el juego sigue sin ese sonido.
/// Esto es obligatorio porque la música (con copyright) no se sube al
/// repo, así que un clon limpio del proyecto siempre va a arrancar sin ella.
pub struct AudioEngine {
    // Se mantiene vivo mientras exista el AudioEngine; si se libera, el
    // audio deja de sonar aunque el Sink siga existiendo.
    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
    music_sink: Option<Sink>,
}

impl AudioEngine {
    pub fn new() -> Self {
        match OutputStream::try_default() {
            Ok((stream, handle)) => AudioEngine {
                _stream: Some(stream),
                handle: Some(handle),
                music_sink: None,
            },
            Err(e) => {
                eprintln!("[audio] no se pudo abrir el dispositivo de sonido, se continúa sin audio: {e}");
                AudioEngine {
                    _stream: None,
                    handle: None,
                    music_sink: None,
                }
            }
        }
    }

    /// Reproduce `path` en loop infinito como música de fondo. Si el
    /// archivo no existe (por ejemplo, el usuario todavía no colocó su
    /// propia música con copyright en assets/audio/music/), se loguea un
    /// aviso y el juego sigue en silencio.
    pub fn play_music_loop(&mut self, path: &str) {
        let Some(handle) = &self.handle else { return };

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                eprintln!(
                    "[audio] no se encontró música de fondo en {path} (ver assets/audio/music/README.md), se continúa sin música"
                );
                return;
            }
        };

        let source = match Decoder::new(BufReader::new(file)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[audio] no se pudo decodificar {path}: {e}, se continúa sin música");
                return;
            }
        };

        match Sink::try_new(handle) {
            Ok(sink) => {
                sink.append(source.repeat_infinite());
                self.music_sink = Some(sink);
            }
            Err(e) => {
                eprintln!("[audio] no se pudo iniciar la música de fondo: {e}");
            }
        }
    }

    /// Reproduce `path` una sola vez, sin bloquear. Pensado para un sonido
    /// puntual (por ejemplo, al llegar a la meta), no para dispararse muy
    /// seguido: decodifica el archivo en el momento en que se llama.
    pub fn play_sound_once(&self, path: &str) {
        let Some(handle) = &self.handle else { return };

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("[audio] no se encontró el sonido {path}");
                return;
            }
        };

        let source = match Decoder::new(BufReader::new(file)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[audio] no se pudo decodificar {path}: {e}");
                return;
            }
        };

        if let Err(e) = handle.play_raw(source.convert_samples()) {
            eprintln!("[audio] no se pudo reproducir {path}: {e}");
        }
    }

    /// Sonido de golpe al chocar contra una pared, generado en el momento
    /// (ver `BumpTone`), sin depender de ningún archivo.
    pub fn play_bump(&self) {
        let Some(handle) = &self.handle else { return };

        if let Err(e) = handle.play_raw(BumpTone::new()) {
            eprintln!("[audio] no se pudo reproducir el sonido de golpe: {e}");
        }
    }
}
