use image::{jpeg::JPEGEncoder, open};
use std::path::Path;
use std::time::Instant;

mod brutemoji;

fn main() {
        let now = Instant::now();

        // let img = open(Path::new("./assets/hot_tubbie.jpg")).unwrap();
        let img = open(Path::new("./assets/georgia.jpg")).unwrap();
        // let mut output = Vec::new();
        // JPEGEncoder::new(&mut output).encode_image(&img).unwrap();
        let path = Path::new("./output/g.png");

        let new_img = brutemoji::generate_image(&img, 90_000, false, path);
        println!("Took {}.{} seconds",
            now.elapsed().as_secs(),
            now.elapsed().subsec_millis());
        match new_img {
            Ok(_) => println!("OK"),
            Err(e) => println!("{}", e),
        }
}
