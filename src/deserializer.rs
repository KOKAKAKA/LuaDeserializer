use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::{structs::chunk::Chunk, util::read_stream::ReadStream};

pub struct Deserializer {
    memory_stream: ReadStream,
}

impl Deserializer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            memory_stream: ReadStream::new(bytes),
        }
    }

    pub fn from_file(file_name: &str) -> Self {
        let mut reader = BufReader::new(
            File::open(file_name).expect(&format!("Failed to read file {}", file_name)),
        );
        let mut buffer = Vec::new();

        reader
            .read_to_end(&mut buffer)
            .expect(&format!("Failed to read file {}", file_name));

        Deserializer::new(buffer)
    }

    pub fn deserialize(&mut self) -> Chunk {
        assert_eq!(
            self.memory_stream.read_string_len(4),
            String::from_utf8(vec![27]).expect("Failed to create string") + "Lua",
            "Invalid file header"
        );

        self.memory_stream.int_size = self.memory_stream.read_int8();

        self.memory_stream.size_t_size = self.memory_stream.read_int8();

        Chunk::new(&mut self.memory_stream)
    }
}
