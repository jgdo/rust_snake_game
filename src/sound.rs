use crate::engine::GameEvent;

use rodio::{Decoder, OutputStreamHandle, OutputStream, source::Source};
use rodio::source::{Buffered, SamplesConverter};
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

pub struct Sound {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    turn_sample: Buffered<SamplesConverter<Decoder<BufReader<File>>, f32>>,
    collision_sample: Buffered<SamplesConverter<Decoder<BufReader<File>>, f32>>,
    teleport_sample: Buffered<SamplesConverter<Decoder<BufReader<File>>, f32>>,
    eat_sample: Buffered<SamplesConverter<Decoder<BufReader<File>>, f32>>,
}


impl Sound {
    pub fn new(assets: &Path) -> Sound {
        // Get a output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let turn_file = BufReader::new(File::open(assets.join("sound").join("turn.wav")).unwrap());
        // Decode that sound file into a source
        let turn_source = Decoder::new(turn_file).unwrap();
        let turn_sample = turn_source.convert_samples().buffered();

        // Load a sound from a file, using a path relative to Cargo.toml
        let collision_file = BufReader::new(File::open(assets.join("sound").join("collision.mp3")).unwrap());
        // Decode that sound file into a source
        let collision_source = Decoder::new(collision_file).unwrap();
        let collision_sample = collision_source.convert_samples().buffered();

        // Load a sound from a file, using a path relative to Cargo.toml
        let teleport_file = BufReader::new(File::open(assets.join("sound").join("teleport.mp3")).unwrap());
        // Decode that sound file into a source
        let teleport_source = Decoder::new(teleport_file).unwrap();
        let teleport_sample = teleport_source.convert_samples().buffered();

        // Load a sound from a file, using a path relative to Cargo.toml
        let eat_file = BufReader::new(File::open(assets.join("sound").join("eat.mp3")).unwrap());
        // Decode that sound file into a source
        let eat_source = Decoder::new(eat_file).unwrap();
        let eat_sample = eat_source.convert_samples().buffered();

        return Sound {
            _stream,
            stream_handle,
            turn_sample,
            collision_sample,
            teleport_sample,
            eat_sample,
        };
    }

    pub fn play_for_event(&self, ev: &GameEvent) {
        match ev {
            GameEvent::None => {}
            GameEvent::Turn => { self.play_sample(self.turn_sample.clone()); }
            GameEvent::Teleport => { self.play_sample(self.teleport_sample.clone()); }
            GameEvent::Eat => { self.play_sample(self.eat_sample.clone()); }
            GameEvent::Collision => {
                self.play_sample(self.collision_sample.clone());
            }
        }
    }

    fn play_sample<S>(&self, sample: S)
        where
            S: Source<Item=f32> + Send + 'static,
    {
        self.stream_handle.play_raw(sample).unwrap_or_default();
    }
}
