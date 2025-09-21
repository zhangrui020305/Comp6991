use bmp::open;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for filepath in &args[1..] {
        println!("===== {} =====", filepath);

        match open(filepath) {
            Ok(image) => {
                for y in 0..image.get_height() {
                    for x in 0..image.get_width() {
                        let pixel = image.get_pixel(x, y);
                        let c = match (pixel.r, pixel.g, pixel.b) {
                            (255, 0, 0) => 'R',
                            (0, 255, 0) => 'G',
                            (0, 0, 255) => 'B',
                            (255, 255, 255) => 'W',
                            _ => ' ',
                        };
                        print!("{} ", c);
                    }
                    println!();
                }
            }
            Err(e) => println!("Error! {:?}", e),
        }
    }
}
