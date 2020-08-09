use image::open;
use std::path::Path;
use std::time::Instant;
use std::env;

mod brutemoji;

fn main() {
        let now = Instant::now();
        let args: Vec<String> = env::args().collect();
        let config = Config::new(&args);
        let orig_path = format!("./assets/{}", config.filename);
        let img = open(Path::new(&orig_path)).unwrap();
        let name = config.filename.split(".").collect::<Vec<&str>>()[0];
        let new_path = String::from(format!("./output/{}_emoji.png", &name));
        let path = Path::new(&new_path);

        let new_img = brutemoji::generate_image(&img, config.iterations, true, path);
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
        let iterations;
        let mut filename = String::from("georgia.jpg");
        if args.len() == 1 {
            iterations = 10_000;
        } else if args.len() == 2 {
            iterations = args[1].clone().parse::<u64>().unwrap();
        } else {
        iterations = args[1].clone().parse::<u64>().unwrap();
        filename = args[2].clone();
        }
        Config {filename, iterations }
    }
}
