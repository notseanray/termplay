mod video;
use video::*;

use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("no arguments found!
example usage:
    termplay video.mp4 30
    termplay --play");
        std::process::exit(1);
    }

    let framerate = match args.len() {
        2.. => args[2].parse().expect("invalid framerate"),
        _ => 10,
    };

    if args[1] == "--play" {
        print_frames(framerate)?;
        std::process::exit(0);
    }

    split_frames(args[1].clone(), framerate)?;
    Ok(())
}
