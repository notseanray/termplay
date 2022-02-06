mod video;
use video::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("no arguments found!
example usage:
    termplay video.mp4 4
    termplay --play");
        std::process::exit(1);
    }

    let mut framerate = 10;

    if args.len() > 2 {
        framerate = args[2].parse().expect("invalid framerate");
    }

    if args[1] == "--play" {
        print_frames(framerate);
        std::process::exit(0);
    }

    split_frames(args[1].clone(), Some(framerate))
}
