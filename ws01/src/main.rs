use std::path;

use bmp::{Image, Pixel};

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let operation = std::env::args()
        .nth(2)
        .expect("You must provide an operation.");

    

    if operation.as_str() == "pixel" {
        draw_pixel(path.as_str());
    } else if operation.as_str() == "diagonal" {
        diagonal(&path.as_str());
    } else if operation.as_str() == "x_shape" {
        x_shape(&path.as_str());
    } else if operation.as_str() == "draw_square" {
        let width = std::env::args()
        .nth(3)
        .expect("You must provide the width of square");
        draw_square(&path.as_str(), width);
    } else if operation.as_str() == "draw_house" {
        draw_house(&path.as_str());
    }
    else {
        eprintln!("The operation {operation} was not recognised!");
    }
}

fn draw_pixel(path: &str) {
    let mut image = Image::new(100, 100);
    image.set_pixel(50, 50, Pixel::new(255, 255, 255));
    image.save(path).expect("This should save correctly.");
}

fn diagonal(path: &str) {
    let mut image = Image::new(100, 100);
    for i in 0..100 {
        image.set_pixel(i, i, bmp::consts::AQUA);
    }

    image.save(path).expect("This should save correctly.");
}

fn x_shape(path: &str) {
    let mut image = Image::new(100, 100);
    let temp = 99;
    for i in 0..100 {
        image.set_pixel(i, i, bmp::consts::AQUA);
        image.set_pixel(i, temp - i, bmp::consts::AQUA);
    }
    image.save(path).expect("This should save correctly.");
}

fn draw_house(path: &str) {
    let mut image = Image::new(100, 100);
    for i in 25..50 {
        image.set_pixel(i, i-25, bmp::consts::AQUA);
    }
    for i in 0..=25 {
        image.set_pixel(25-i, i, bmp::consts::AQUA);
    }
    for i in 0..=50 {
        image.set_pixel(i, 25, bmp::consts::AQUA);
    }
    for i in 0..=25 {
        image.set_pixel(0, i+25, bmp::consts::AQUA);
    }
    for i in 0..=25 {
        image.set_pixel(50, i+25, bmp::consts::AQUA);
    }
    for i in 0..=50 {
        image.set_pixel(i, 50, bmp::consts::AQUA);
    }
    image.save(path).expect("This should save correctly.");
}

fn draw_square(path: &str, width: String) {
    let mut image = Image::new(100, 100);

    let int_width = width.parse().unwrap();

    for i in 0..int_width {
        image.set_pixel(0, i, bmp::consts::AQUA);
    }
    for i in 0..50 {
        image.set_pixel(int_width, i + 1, bmp::consts::AQUA);
    }
    for i in 0..50 {
        image.set_pixel(i, 0, bmp::consts::AQUA);
    }
    for i in 0..50 {
        image.set_pixel(i, int_width, bmp::consts::AQUA);
    }
    image.save(path).expect("This should save correctly.");
}
