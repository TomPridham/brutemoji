
mod brutemoji;
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
