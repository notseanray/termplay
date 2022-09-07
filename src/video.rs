use image::imageops::{grayscale, resize};
use image::open;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::cmp::Ordering;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;
use std::{thread, time};
use terminal_size::{terminal_size, Height, Width};

pub fn split_frames(file: String, framerate: i32) -> std::io::Result<()> {
    let _ = fs::remove_dir_all("/tmp/termplay.cache");

    let _ = fs::create_dir("/tmp/termplay.cache");

    let frames = format!("fps={framerate}");
    let ffmpeg = Command::new("ffmpeg")
        .args(["-i", &file, "-vf", &frames, "/tmp/termplay.cache/%d.jpg"])
        .status()
        .expect("failed to run ffmpeg");

    if !ffmpeg.success() {
        panic!("ffmpeg failed to install");
    }
    resize_frames()?;
    print_frames(framerate)?;
    println!("\x1Bc");
    println!("\x1B[?251]");

    Ok(())
}

fn resize_frames() -> std::io::Result<()> {
    let (Width(w), Height(h)) = match terminal_size() {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let mut files = Vec::new();
    fs::read_dir("/tmp/termplay.cache")?.for_each(|x| files.push(x.unwrap().path()));
    files.par_iter().for_each(|i| {
        let img = open(i).expect("image failed to open");
        let img = resize(
            &grayscale(&img),
            w as u32,
            h as u32,
            image::imageops::FilterType::Nearest,
        );
        img.save(i).expect("failed to resize frame");
        println!("resized: {:#?}", i);
    });
    Ok(())
}

#[allow(clippy::ptr_arg)]
#[inline(always)]
fn format(l: &PathBuf, r: &PathBuf) -> Ordering {
    // janky but this takes only the number bit of the file name from ffmpeg
    let left = l.to_string_lossy().split('.').collect::<Vec<_>>()[1]
        .split('/')
        .collect::<Vec<_>>()[1]
        .parse::<usize>()
        .unwrap_or(0);
    let right = r.to_string_lossy().split('.').collect::<Vec<_>>()[1]
        .split('/')
        .collect::<Vec<_>>()[1]
        .parse::<usize>()
        .unwrap_or(0);
    match left > right {
        true => Ordering::Greater,
        false => Ordering::Less,
    }
}

#[inline(always)]
pub fn print_frames(framerate: i32) -> std::io::Result<()> {
    let chars = vec!["#", "&", "@", "$", "%", "*", ".", " "];
    let mut frames = fs::read_dir("/tmp/termplay.cache")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;
    frames.sort_by(format);
    for i in frames {
        match open(i) {
            Ok(v) => {
                let img = v.into_bytes();
                let mut frame: String = String::with_capacity(img.len());
                for i in img {
                    frame.push_str(chars[(i / 36) as usize]);
                }
                print!("{frame}");
                let delay = time::Duration::from_millis((1000 / framerate).try_into().unwrap());
                thread::sleep(delay);
            }
            Err(e) => println!("{e}"),
        }
    }
    Ok(())
}
