use image::open;
use std::path::Path;
use std::time::Instant;
use std::env;

mod brutemoji;

fn main() {
        let now = Instant::now();
        let args: Vec<String> = env::args().collect();
        let config = Config::new(&args);
        let img = open(Path::new(&config.filename)).unwrap();
        let new_path = String::from(format!("./output/{}.png", "1"));
        let path = Path::new(&new_path);

        let new_img = brutemoji::generate_image(&img, config.iterations, false, path);
        println!("Took {}.{} seconds",
            now.elapsed().as_secs(),
            now.elapsed().subsec_millis());
        match new_img {
            Ok(_) => println!("OK"),
            Err(e) => println!("{}", e),
        }
}

struct Config {
    filename: String,
    iterations: u64,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let iterations : u64;
        let filename : String;
        if args.len() < 2 {
            iterations = 10_000;
            filename = String::from("./assets/georgia.jpg");
        } else if args.len() < 3 {
            iterations = 10_000;
            filename = String::from("./assets/georgia.jpg");
            return Config {filename, iterations }
        } else {
        iterations = args[1].clone().parse::<u64>().unwrap();
        filename = args[2].clone();
        }
        Config {filename, iterations }
    }
}
