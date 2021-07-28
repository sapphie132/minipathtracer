mod bsdf;
mod face;
mod image;
mod render_arena;
mod scene;
mod util;

use scene::*;
use std::io::Read;
use std::env::args;

type float = f32;

pub trait Object {
    fn deserialize<R: Read>(r: &mut R) -> Result<Self, String> where Self: Sized;
}

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

    let scene = match Scene::read(&input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return
        }
    };

    let image = render_arena::render(&scene, num_threads) ;
    image.write(&output);
}

fn print_usage() {
    eprintln!("Usage: {} INPUT_FILE OUTPUT_FILE [NUM_THREADS]", args().next().unwrap());
}