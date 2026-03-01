use std::fs;
use std::io::{self};
use std::path::PathBuf;

pub trait ValidImage {
    fn is_valid_image(&self, file_path: &PathBuf) -> bool {
        match mime_guess::from_path(file_path).first() {
            Some(mime) => {
                mime.type_() == "image"
                    && (mime.subtype() == "jpeg"
                        || mime.subtype() == "png"
                        || mime.subtype() == "webp"
                        || mime.subtype() == "jpg")
            }
            None => false,
        }
    }
}

pub trait ValidSize {
    fn read_image(&self, file_path: &PathBuf) -> io::Result<Vec<u8>> {
        const MAX_IMG_SIZE: u64 = 1_000_000;
        let metadata = fs::metadata(file_path)?;
        if metadata.len() > MAX_IMG_SIZE {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File size exceeds limit",
            ))
        } else {
            fs::read(file_path)
        }
    }
}
