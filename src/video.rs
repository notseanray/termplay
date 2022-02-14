use std::process::Command;
use std::io::Result;
use std::fs;
use std::{thread, time};
use image::imageops::{resize, grayscale};
use image::open;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use terminal_size::{Width, Height, terminal_size};

pub fn split_frames(file: String, framerate: i32) -> std::io::Result<()> {
    fs::remove_dir_all("/tmp/termplay.cache")?;

    fs::create_dir("/tmp/termplay.cache")?;

    let frames = format!("fps={framerate}");
    let ffmpeg = Command::new("ffmpeg")
        .args(["-i", &file, "-vf", &frames, "/tmp/termplay.cache/%03d.jpg"])
        .status()
        .expect("failed to run ffmpeg");

    if !ffmpeg.success() {
        panic!("ffmpeg failed to install");
    }
    resize_frames()?;
    print_frames(framerate)?;

    Ok(())
}

fn resize_frames() -> std::io::Result<()> {
    let (Width(w), Height(h)) = match terminal_size() {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let mut files = Vec::new();
    fs::read_dir("/tmp/termplay.cache")?.for_each(|x| files.push(x.unwrap().path()));
    files.par_iter()
        .for_each(|i| {
            let img = open(i).expect("image failed to open");
            let img = resize(
                &grayscale(&img), 
                w as u32, 
                h as u32, 
                image::imageops::FilterType::Nearest
            );
            img.save(i).expect("failed to resive frame");
            println!("resized: {:#?}", i);
        });
    Ok(())
}

pub fn print_frames(framerate: i32) -> std::io::Result<()> {
    let chars = vec!["#", "&", "@", "$", "%", "*", ".", " "]; 
    let mut frames = fs::read_dir("/tmp/termplay.cache")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;
    frames.sort();
    for i in frames {
        match open(i) {
            Ok(v) => {
                let mut frame: String = "\x1B[2J".to_string();
                let img = v.into_bytes();
                for i in img {
                    frame.push_str(chars[(i / 36) as usize]);
                }
                print!("{frame}");
                let delay = time::Duration::from_millis((1000 / framerate)
                                                        .try_into()
                                                        .unwrap());
                thread::sleep(delay);
            },
            Err(e) => println!("{e}"),
        }
    }
    Ok(())
}
