use image::{imageops::overlay, ImageError, DynamicImage};
use std::path::Path;
use rand::prelude::*;
use std::time::Instant;

mod emoji;


fn measure_dist_chunks(samples_a: &[u8], samples_b: &[u8]) -> i64 {
    samples_a
        .chunks_exact(1)
        .zip(samples_b.chunks_exact(1))
        .fold(0, |rgba, (p_a, p_b)| {
                rgba + (p_a[0] as i64 - p_b[0] as i64).abs()
        })
}

fn subimage_compare(image_a: &DynamicImage, image_b: &DynamicImage, x: u32, y: u32 ) -> i64 {
    let sub_image_a = image_a.crop_imm(x,y,16,16).to_rgb();
    let sub_image_b = image_b.crop_imm(x,y,16,16).to_rgb();
    measure_dist_chunks(&sub_image_a, &sub_image_b)
}

pub fn generate_image(
    image_buffer: &DynamicImage,
    iterations: u64,
    save_progress: bool,
    path: &Path,
) -> Result<Vec<u8>, ImageError> {
    let now = Instant::now();

    let mut emoji_cache = emoji::EmojiCache::new();

    let image_buffer_rgb = image_buffer.clone().to_rgb();
    let (width, height) = image_buffer_rgb.dimensions();
    let canvas_size = width * height;
    let mut rng = thread_rng();
    let mut new_img = DynamicImage::new_rgb16(width, height);
    println!("image is {} by {} pixels", width, height);
    let mut dist = measure_dist_chunks(&image_buffer_rgb, &new_img.to_rgb());
    println!("dist is {}", dist);

    let mut placed_count = 0;
    for _  in 0..canvas_size/20 {
        let e = emoji_cache.get_emoji();
        let x: u32 = (1..width).choose(&mut rng).unwrap();
        let y: u32 = (1..height).choose(&mut rng).unwrap();
        overlay(&mut new_img, e, x, y);
        placed_count= placed_count+1;
    }

    dist = measure_dist_chunks(&image_buffer_rgb, &new_img.to_rgb());
    println!("dist is {}", dist);
    
    for index in 0..iterations {
        let e = emoji_cache.get_emoji();
        let x: u32 = (1..width).choose(&mut rng).unwrap();
        let y: u32 = (1..height).choose(&mut rng).unwrap();
        let mut temp_img = new_img.clone();
        let temp_dist1 = subimage_compare(&image_buffer, &temp_img, x, y);
        overlay(&mut temp_img, e, x, y);
        let temp_dist2 = subimage_compare(&image_buffer, &temp_img, x, y);
        if temp_dist1 > temp_dist2 {
            new_img = temp_img;
            placed_count= placed_count+1;
        }
        if index%1000==0 {
            dist = measure_dist_chunks(&image_buffer_rgb, &new_img.to_rgb());
            println!("iteration: {}, dist: {}, time: {}.{}, emoji: {}",
                index,
                dist,
                now.elapsed().as_secs(),
                now.elapsed().subsec_millis(),
                placed_count
                )
        }
        if index%10000==0 && save_progress {
            new_img.save(path)?;
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
        use image::open;
        use std::path::Path;
        use std::time::Instant;
        let now = Instant::now();

        let img = open(Path::new("./assets/georgia.jpg")).unwrap();
        let path = Path::new("./g.png");

        let new_img = crate::generate_image(&img, 30_000, false, path);
        println!("{}", now.elapsed().as_secs());
        match new_img {
            Ok(_) => println!("OK"),
            Err(e) => println!("{}", e),
        }
    }
}
