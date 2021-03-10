use image::{imageops::overlay, DynamicImage, GenericImageView, ImageError};
use std::path::Path;

mod emoji;

fn measure_dist_chunks(samples_a: &[u8], samples_b: &[u8]) -> i64 {
    samples_a
        .iter()
        .zip(samples_b.iter())
        .fold(0, |rgba, (p_a, p_b)| {
            rgba + (*p_a as i64 - *p_b as i64).abs()
        })
}

fn subimage_compare(image_a: &DynamicImage, image_b: &DynamicImage, x: u32, y: u32) -> i64 {
    let sub_image_a = image_a.crop_imm(x, y, 16, 16).to_rgb8();
    let sub_image_b = image_b.crop_imm(x, y, 16, 16).to_rgb8();
    measure_dist_chunks(&sub_image_a, &sub_image_b)
}

pub fn generate_image(
    image_buffer: &DynamicImage,
    iterations: u64,
    save_progress: bool,
    path: &Path,
) -> Result<Vec<u8>, ImageError> {
    let mut emoji_cache = emoji::EmojiCache::new();

    let (width, height) = image_buffer.dimensions();
    let (width, height) = (std::cmp::min(width, 1000), std::cmp::min(height, 1000));
    let canvas_size = width * height;
    let rng = fastrand::Rng::new();
    let mut new_img = DynamicImage::new_rgba16(width, height);

    for _ in 0..canvas_size / 20 {
        let e = emoji_cache.get_emoji(&rng);
        let x = rng.u32(0..width);
        let y = rng.u32(0..height);
        overlay(&mut new_img, e, x, y);
    }

    for index in 0..iterations {
        let e = emoji_cache.get_emoji(&rng);
        let x = rng.u32(0..width);
        let y = rng.u32(0..height);
        let mut temp_img = new_img.clone();
        let temp_dist1 = subimage_compare(&image_buffer, &temp_img, x, y);
        overlay(&mut temp_img, e, x, y);
        let temp_dist2 = subimage_compare(&image_buffer, &temp_img, x, y);
        if temp_dist1 > temp_dist2 {
            new_img = temp_img;
        }
        if index % 1000 == 0 && save_progress {
            new_img.save(path)?;
        }
    }
    new_img.save(path)?;

    Ok(new_img.to_rgb8().to_vec())
}

#[cfg(test)]

mod tests {

    #[test]
    fn dist_chunks_measures_correctly() {
        let a = vec![0; 10];
        let b = vec![u8::MAX; 10];
        assert_eq!(crate::measure_dist_chunks(&a, &b), u8::MAX as i64 * 10);
    }

    #[test]
    fn it_works() {
        use image::open;
        use std::path::Path;
        use std::time::Instant;
        let now = Instant::now();

        let img = open(Path::new("./assets/georgia.jpg")).unwrap();
        let path = Path::new("./g.png");

        let new_img = crate::generate_image(&img, 30, true, path);
        println!("{}", now.elapsed().as_secs());
        match new_img {
            Ok(_) => println!("OK"),
            Err(e) => println!("{}", e),
        }
    }
}
