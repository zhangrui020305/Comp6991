use bmp::{Image, Pixel};

fn main() {
    draw_pixel("R.bmp");
}

fn draw_pixel(path: &str) {
    let mut image = Image::new(200, 200);

    let orange = Pixel::new(255, 165, 0);

    for y in 20..180 {
        image.set_pixel(40, y, orange);
    }

    for x in 40..120 {
        image.set_pixel(x, 20, orange);
    }

    for x in 40..120 {
        image.set_pixel(x, 100, orange);
    }

    for y in 20..100 {
        image.set_pixel(120, y, orange);
    }

    for i in 0..80 {
        image.set_pixel(40 + i, 100 + i, orange);
    }

    image.save(path).expect("This should save correctly.");
}
