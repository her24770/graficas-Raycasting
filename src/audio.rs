use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

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

    /// Reproduce `path` una vez, sin bloquear. Si falta el archivo o falla
    /// la decodificación, solo se loguea un aviso.
    pub fn play_sfx(&self, path: &str) {
        let Some(handle) = &self.handle else { return };

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("[audio] no se encontró el efecto de sonido {path}");
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
}
