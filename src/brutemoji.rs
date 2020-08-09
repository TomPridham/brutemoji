use image::{imageops::overlay, ImageError, DynamicImage};
extern crate rand;
use rand::thread_rng;
use std::path::Path;
// use rand::seq::SliceRandom;
use rand::prelude::*;
use std::time::Instant;

mod emoji;

fn measure_dist(rng: &mut ThreadRng, samples_a: &Vec<u8>, samples_b: Vec<u8>) -> i64 {
    //let mut rng = thread_rng();
    let sample_length = samples_a.len();
    let v1 = (1..sample_length).choose_multiple(rng, sample_length/10);
    let v1_iter = v1.iter();
    let mut diff = 0;
    for val in v1_iter {
        diff = diff + (samples_a[*val] as i64 - samples_b[*val] as i64).abs();
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

fn sums_chunked_targeted(samples_a: &DynamicImage, samples_b: &DynamicImage, x: u32, y: u32 ) -> i32 {
    let sub_img = samples_a.crop_imm(x-8,y-8,32,32).to_rgb();
    let sub_img_b = samples_b.crop_imm(x-8,y-8,32,32).to_rgb();
   sub_img
        .chunks_exact(1)
        .zip(sub_img_b.chunks_exact(1))
        .fold(0, |rgba, (p_a, p_b)| {
                rgba + (p_a[0] as i32 - p_b[0] as i32).abs()
        })
}

pub fn generate_image(
    image_buffer: &DynamicImage,
    iterations: u64,
    save_progress: bool,
    path: &Path,
) -> Result<Vec<u8>, ImageError> {
    let now = Instant::now();

    let mut emoji_cache = emoji::EmojiCache::new();

    // let orig = load_from_memory(image_buffer.clone())?.into_rgba();
    let image_buffer_rgb = image_buffer.clone().to_rgb();
    let (width, height) = image_buffer_rgb.dimensions();
    // let orig_vec = orig.into_vec();
    let canvas_size = width * height;
    let mut rng = thread_rng();
    let mut new_img = DynamicImage::new_rgb16(width, height);
    println!("image is {} by {} pixels", width, height);
    // let mut dist = measure_dist(&mut rng, &orig_vec,  new_img.clone().into_vec());
    let mut dist = sums_chunked(&image_buffer_rgb, &new_img.to_rgb());
    println!("dist is {}", dist);

    let mut placed_count = 0;
    for _  in 0..canvas_size/20 {
        let e = emoji_cache.get_emoji();
        let x: u32 = (1..width).choose(&mut rng).unwrap();
        let y: u32 = (1..height).choose(&mut rng).unwrap();
        overlay(&mut new_img, e, x, y);
        placed_count= placed_count+1;
    }

    dist = sums_chunked(&image_buffer_rgb, &new_img.to_rgb());
    // ist = measure_dist(&mut rng, &orig_vec, new_img.clone().into_vec());
    println!("dist is {}", dist);
    
    for index in 0..iterations {
        let e = emoji_cache.get_emoji();
        let x: u32 = (1..width).choose(&mut rng).unwrap();
        let y: u32 = (1..height).choose(&mut rng).unwrap();
        let mut temp_img = new_img.clone();
        let temp_dist1 = sums_chunked_targeted(&image_buffer, &temp_img, x, y);
        overlay(&mut temp_img, e, x, y);
        let temp_dist2 = sums_chunked_targeted(&image_buffer, &temp_img, x, y);
        // let temp_dist = measure_dist(&mut rng, &orig_vec, temp_img.clone().into_vec());
        if temp_dist1 > temp_dist2 {
            new_img = temp_img;
            placed_count= placed_count+1;
            if save_progress {
                new_img.save(path)?;
            }
        }
        if index%1000==0 {
        dist = sums_chunked(&image_buffer_rgb, &new_img.to_rgb());
            println!("iteration: {}, dist: {}, time: {}.{}, emoji: {}",
                index,
                dist,
                now.elapsed().as_secs(),
                now.elapsed().subsec_millis(),
                placed_count
                )
        }
    }
    println!("placed {} emoji", placed_count);
    new_img.save(path)?;

    Ok(new_img.to_rgb().to_vec())
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
