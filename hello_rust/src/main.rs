use bmp::{Image, Pixel};

fn main() {
    let mut img = Image::new(256, 256);
    
    // img.set_pixel(1, 1, Pixel::new(0, 0, 200));

    for x in 0..=255 {
        for y in 0..=255 {
            img.set_pixel(x, y, Pixel::new(0, 0, 200));
        }
    }

    img.save("img.bmp").unwrap();
}
