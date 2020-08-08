use image::{imageops::overlay, load_from_memory, ImageError, RgbaImage};
use rand::random;
use std::path::Path;

mod emoji;

fn sums_chunked(samples_a: &[u8], samples_b: &[u8]) -> (i32, i32, i32) {
    samples_a
        .chunks_exact(3)
        .zip(samples_b.chunks_exact(3))
        .fold((0, 0, 0), |(r, g, b), (p_a, p_b)| {
            (
                r + (p_a[0] as i32 - p_b[0] as i32).abs(),
                g + (p_a[1] as i32 - p_b[1] as i32).abs(),
                b + (p_a[2] as i32 - p_b[2] as i32).abs(),
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
    let orig = load_from_memory(image_buffer)?.into_rgba();
    let (width, height) = orig.dimensions();
    let mut new_img = RgbaImage::new(width, height);
    let mut dist = sums_chunked(&orig, &new_img);
    let mut xs: Vec<u32> = (0..iterations*2).map( |_| {random::<u32>() % width}).collect();
    let mut ys: Vec<u32> = (0..iterations*2).map( |_| {random::<u32>() % height}).collect();
//    let mut es: Vec<u32> = (0..iterations*2).map( |_| {random::<u32>() % height}).collect();

    for _  in 0..iterations {
        let e = emoji_cache.get_emoji();
        let x: u32 = xs.pop().unwrap();
        let y: u32 = ys.pop().unwrap();
        let mut temp_img = new_img.clone();
        overlay(&mut temp_img, e, x, y);
            new_img = temp_img;
    }
    for _ in 0..iterations {
        let e = emoji_cache.get_emoji();
        let x: u32 = xs.pop().unwrap();
        let y: u32 = ys.pop().unwrap();
        let mut temp_img = new_img.clone();
        overlay(&mut temp_img, e, x, y);
        let temp_dist = sums_chunked(&orig, &temp_img);
        if dist > temp_dist {
            new_img = temp_img;
            dist = temp_dist;
            if save_progress {
                new_img.save(path)?;
            }
        }
    }
    new_img.save(path)?;

    Ok(new_img.to_vec())
}

