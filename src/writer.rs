use super::FileError;
use std::io::Write;
use std::fs::File;

pub struct Writer {
    file : File,
    path : String,
}

impl Writer {
    pub fn new(path : &str) -> Result<Writer, FileError> {
        let file = match File::create(&path) {
            Ok(f) => f,
            Err(e) => { return Err(FileError::new(e.kind(), path)); }
        };
        Ok (Writer {
            file,
            path : String::from(path)
        })
    }

    pub fn write(&mut self, buf : &[u8]) -> Result<usize, FileError> {
        match self.file.write(buf) {
            Ok(len_wrote) => Ok(len_wrote),
            Err(e)        => Err(FileError::new(e.kind(), &self.path))
        }
    }
}