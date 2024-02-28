use std::error::Error;

use sdl2::audio::{AudioCVT, AudioCallback, AudioDevice, AudioSpecDesired, AudioSpecWAV};
use sdl2::Sdl;

pub struct Beep {
    audio_device: AudioDevice<Callback>,
}

pub struct Callback {
    buffer: Vec<u8>,
    position: usize,
}

impl AudioCallback for Callback {
    type Channel = i16;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for value in out.iter_mut() {
            *value = if self.position < self.buffer.len() {
                let sample = i16::from_le_bytes([
                    self.buffer[self.position],
                    self.buffer[self.position + 1],
                ]);
                self.position += 2;
                sample
            } else {
                0
            }
        }
    }
}

impl Beep {
    pub fn init(sdl: &Sdl) -> Result<Self, Box<dyn Error>> {
        let audio = sdl.audio()?;

        let spec = AudioSpecDesired {
            freq: None,
            channels: None,
            samples: None,
        };

        let wav = AudioSpecWAV::load_wav("./audio/beep.wav")?;

        let audio_device = audio.open_playback(None, &spec, |spec| {
            let converter = AudioCVT::new(
                wav.format,
                wav.channels,
                wav.freq,
                spec.format,
                spec.channels,
                spec.freq,
            )
            .expect("converter creation error");

            let buffer = converter.convert(wav.buffer().to_vec());

            Callback {
                buffer,
                position: 0,
            }
        })?;

        Ok(Self { audio_device })
    }

    pub fn beep(&mut self) {
        self.audio_device.resume();
        let mut lock = self.audio_device.lock();
        lock.position = 0;
    }
}
