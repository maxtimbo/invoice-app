use anyhow::{anyhow, Result};
use std::path::Path;

pub fn validate_image(path: &Path) -> Result<Vec<u8>> {
    let mime = mime_guess::from_path(&path)
        .first()
        .ok_or_else(|| anyhow!("Could not determine file type for {:?}", path))?;
    if mime.type_() != "image" {
        return Err(anyhow!("File must be an image (got {})", mime));
    }
    let accepted = ["jpeg", "jpg", "png", "webp"];
    if !accepted.contains(&mime.subtype().as_str()) {
        return Err(anyhow!(
            "Unsupported image format '{}'. Use jpeg, png, or webp.",
            mime.subtype()
        ));
    }

    const MAX_BYTES: u64 = 1_000_000;
    let size = std::fs::metadata(&path)?.len();
    if size > MAX_BYTES {
        return Err(anyhow!(
            "Image is {:.1} KB, maximum is 1000 KB.",
            size as f64 / 1024.0
        ));
    }
    Ok(std::fs::read(&path)?)
}
