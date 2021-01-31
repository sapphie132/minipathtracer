mod image;
mod render_arena;
mod scene;

use scene::*;
use image::Image;
use std::env::args;

fn main() {
    if args().len() < 2 {
        print_usage();
        return
    }

    let input = args().nth(1).unwrap();
    let output = args().nth(2).unwrap();
    let num_threads = if let Some(s) = args().nth(3) {
        if let Ok(n) = s.parse() {
            n
        } else {
            print_usage();
            return
        }
    } else {
        1
    };

    let scene = Scene::read(&input);
    let image = render_arena::render(&scene);
    image.write(&output);
}

fn print_usage() {
    eprintln!("Usage: {} INPUT_FILE OUTPUT_FILE [NUM_THREADS]", args().next().unwrap());
}