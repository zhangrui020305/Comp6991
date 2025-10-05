use serde::Deserialize;
use std::collections::VecDeque;
use std::io;

#[derive(Debug, Deserialize)]
enum Instruction {
    Set(i32),
    Left,
    Right,
    Reset,
}

#[derive(Debug)]
struct Light {
    brightness: i32,
    left: Option<Box<Light>>,
    right: Option<Box<Light>>,
}

impl Light {
    fn new(brightness: i32) -> Self {
        Self {
            brightness,
            left: None,
            right: None,
        }
    }
}

fn get_instructions_from_stdin() -> VecDeque<Instruction> {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).unwrap();
    ron::from_str(&instructions).unwrap()
}

fn collect_brightness(light: &Light, values: &mut Vec<i32>) {
    values.push(light.brightness);
    if let Some(ref l) = light.left {
        collect_brightness(l, values);
    }
    if let Some(ref r) = light.right {
        collect_brightness(r, values);
    }
}

fn main() {
    let instructions = get_instructions_from_stdin();
    let mut root = Box::new(Light::new(0));

    let mut current: *mut Light = &mut *root;

    for instr in instructions {
        unsafe {
            match instr {
                Instruction::Set(x) => {
                    (*current).brightness = x;
                }
                Instruction::Left => {
                    if (*current).left.is_none() {
                        (*current).left = Some(Box::new(Light::new(0)));
                    }
                    current = &mut *(*current).left.as_mut().unwrap();
                }
                Instruction::Right => {
                    if (*current).right.is_none() {
                        (*current).right = Some(Box::new(Light::new(0)));
                    }
                    current = &mut *(*current).right.as_mut().unwrap();
                }
                Instruction::Reset => {
                    current = &mut *root;
                }
            }
        }
    }

    let mut values = vec![];
    collect_brightness(&root, &mut values);

    if values.is_empty() {
        println!("0");
    } else {
        let sum: i32 = values.iter().sum();
        let avg = sum / values.len() as i32;
        println!("{avg}");
    }
}
