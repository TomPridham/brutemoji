use image::{
    imageops::overlay, load_from_memory, load_from_memory_with_format, GenericImageView,
    ImageError, ImageFormat, RgbaImage,
};
use rand::random;
use std::path::Path;

mod emoji;

fn sums_chunked(samples_a: &[u8], samples_b: &[u8]) -> (i32, i32, i32) {
    samples_a
        .chunks_exact(3)
        .zip(samples_b.chunks_exact(3))
        .fold((0, 0, 0), |(r, g, b), (p_a, p_b)| {
            (
                r + (p_a[0] as i32 - p_b[0] as i32),
                g + (p_a[1] as i32 - p_b[1] as i32),
                b + (p_a[2] as i32 - p_b[2] as i32),
            )
        })
}

pub fn generate_image(
    image_buffer: &[u8],
    iterations: u64,
    save_progress: bool,
    path: &Path,
) -> Result<Vec<u8>, ImageError> {
    let mut emoji_cache = emoji::EmojiCache::new();
    let orig = load_from_memory(image_buffer)?;
    let (width, height) = orig.dimensions();
    let orig = orig.to_bytes();
    let mut new_img = RgbaImage::new(width, height);
    let mut dist = sums_chunked(&orig, &new_img);

    for _ in 0..iterations {
        let e = &emoji_cache.get_emoji();
        let w: u32 = random::<u32>() % width;
        let h: u32 = random::<u32>() % height;
        let mut temp_img = new_img.clone();
        overlay(&mut temp_img, &**e, w, h);
        let temp_dist = sums_chunked(&orig, &temp_img);
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
