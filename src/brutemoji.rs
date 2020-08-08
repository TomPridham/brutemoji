use image::{imageops::overlay, load_from_memory, ImageError, RgbaImage};
extern crate rand;
use rand::thread_rng;
use std::path::Path;
// use rand::seq::SliceRandom;
use rand::prelude::*;
use std::time::Instant;

mod emoji;

fn measure_dist(rng: &mut ThreadRng, samples_a: &[u8], samples_b: &[u8]) -> i32 {
    //let mut rng = thread_rng();
    let sample_length = samples_a.len();
    let v1 = (1..sample_length).choose_multiple(rng, sample_length/10);
    let v1_iter = v1.iter();
    let mut diff = 0;
    for val in v1_iter {
        diff = diff + (samples_a[*val] as i32 - samples_b[*val] as i32).abs();
    }

    return diff;
}

fn sums_chunked(samples_a: &[u8], samples_b: &[u8]) -> i32 {
    samples_a
        .chunks_exact(1)
        .zip(samples_b.chunks_exact(1))
        .fold(0, |rgba, (p_a, p_b)| {
                rgba + (p_a[0] as i32 - p_b[0] as i32).abs()
        })
}

pub fn generate_image(
    image_buffer: &[u8],
    iterations: u64,
    save_progress: bool,
    path: &Path,
) -> Result<Vec<u8>, ImageError> {
    let now = Instant::now();

    let mut emoji_cache = emoji::EmojiCache::new();
    let orig = load_from_memory(image_buffer)?.into_rgba();
    let (width, height) = orig.dimensions();
    let canvas_size = width * height;
    let mut rng = thread_rng();
    let mut new_img = RgbaImage::new(width, height);
    println!("image is {} by {} pixels", width, height);
    let mut dist = sums_chunked(&orig, &new_img);
    println!("dist is {}", dist);

    for _  in 0..canvas_size/40 {
        let e = emoji_cache.get_emoji();
        let x: u32 = (1..width).choose(&mut rng).unwrap();
        let y: u32 = (1..height).choose(&mut rng).unwrap();
        overlay(&mut new_img, e, x, y);
    }

    dist = sums_chunked(&orig, &new_img);
    println!("dist is {}", dist);
    let mut placed_count = 0;
    
    for index in 0..iterations {
        let e = emoji_cache.get_emoji();
        let x: u32 = (1..width).choose(&mut rng).unwrap();
        let y: u32 = (1..height).choose(&mut rng).unwrap();
        let mut temp_img = new_img.clone();
        overlay(&mut temp_img, e, x, y);
        let temp_dist = sums_chunked(&orig, &temp_img);
        if dist > temp_dist {
            new_img = temp_img;
            dist = temp_dist;
            placed_count= placed_count+1;
            if save_progress {
                new_img.save(path)?;
            }
        }
        if index%1000==0 {
            println!("iteration: {}, dist: {}, time: {}.{}, emoji: {}",
                index,
                temp_dist,
                now.elapsed().as_secs(),
                now.elapsed().subsec_millis(),
                placed_count
                )
        }
    }
    println!("placed {} emoji", placed_count);
    new_img.save(path)?;

    Ok(new_img.to_vec())
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        use image::{jpeg::JPEGEncoder, open};
        use std::path::Path;
        use std::time::Instant;
        let now = Instant::now();

        let img = open(Path::new("./assets/georgia.jpg")).unwrap();
        let mut output = Vec::new();
        JPEGEncoder::new(&mut output).encode_image(&img).unwrap();
        let path = Path::new("./g.png");

        let new_img = crate::brutemoji::generate_image(&output, 30_000, false, path);
        println!("{}", now.elapsed().as_secs());
        match new_img {
            Ok(_) => println!("OK"),
            Err(e) => println!("{}", e),
        }
    }
}
