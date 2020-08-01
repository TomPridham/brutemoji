use image::{
    imageops::overlay, load_from_memory, load_from_memory_with_format, GenericImageView,
    ImageError, ImageFormat, RgbaImage,
};
use rand::random;
use std::path::Path;

mod emoji;

pub fn generate_image(image_buffer: &[u8]) -> Result<Vec<u8>, ImageError> {
    let orig = load_from_memory(image_buffer)?;
    let (width, height) = orig.dimensions();
    let mut new_img = RgbaImage::new(width, height);
    let e = emoji::get_emoji();

    for _ in 0..1000 {
        let w: u32 = random::<u32>() % width;
        let h: u32 = random::<u32>() % height;
        let e = load_from_memory_with_format(e, ImageFormat::Png)?;
        overlay(&mut new_img, &e, w, h)
    }
    let s = new_img.save(Path::new("./g.png"));
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

        let img = open(Path::new("./assets/georgia.jpg")).unwrap();
        let mut output = Vec::new();
        JPEGEncoder::new(&mut output).encode_image(&img).unwrap();

        let new_img = generate_image(&output);
        match new_img {
            Ok(_) => println!("OK"),
            Err(e) => println!("{}", e),
        }
    }
}
