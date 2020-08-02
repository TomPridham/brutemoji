use image::{
    imageops::overlay, load_from_memory, load_from_memory_with_format, GenericImageView,
    ImageError, ImageFormat, RgbaImage,
};
use img_hash::{HashAlg, HasherConfig};
use rand::random;
use std::path::Path;

mod emoji;

pub fn generate_image(
    image_buffer: &[u8],
    iterations: u64,
    save_progress: bool,
    path: &Path,
) -> Result<Vec<u8>, ImageError> {
    let orig = load_from_memory(image_buffer)?;
    let (width, height) = orig.dimensions();
    let mut new_img = RgbaImage::new(width, height);
    let hasher = HasherConfig::new().hash_alg(HashAlg::Blockhash).to_hasher();
    let orig_hash = hasher.hash_image(&orig);
    let new_hash = hasher.hash_image(&new_img);
    let mut dist = orig_hash.dist(&new_hash);

    for _ in 0..iterations {
        let e = emoji::get_emoji();
        let w: u32 = random::<u32>() % width;
        let h: u32 = random::<u32>() % height;
        let e = load_from_memory_with_format(e, ImageFormat::Png)?;
        let mut temp_img = new_img.clone();
        overlay(&mut temp_img, &e, w, h);
        let temp_hash = hasher.hash_image(&temp_img);
        let temp_dist = orig_hash.dist(&temp_hash);
        if dist > temp_dist {
            new_img = temp_img;
            dist = temp_dist;
            if save_progress {
                let s = new_img.save(path);
                match s {
                    Ok(_) => println!("ok"),
                    Err(e) => println!("{}", e),
                };
            }
        }
    }
    let s = new_img.save(path);
    match s {
        Ok(_) => println!("ok"),
        Err(e) => println!("{}", e),
    };

    Ok(vec![0])
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::generate_image;
        use image::{jpeg::JPEGEncoder, open};
        use std::path::Path;
        use std::time::Instant;
        let now = Instant::now();

        let img = open(Path::new("./assets/georgia.jpg")).unwrap();
        let mut output = Vec::new();
        JPEGEncoder::new(&mut output).encode_image(&img).unwrap();
        let path = Path::new("./g.png");

        let new_img = generate_image(&output, 100, true, path);
        println!("{}", now.elapsed().as_secs());
        match new_img {
            Ok(_) => println!("OK"),
            Err(e) => println!("{}", e),
        }
    }
}
