use std::process::Command;
use std::{fs, io};
use std::{thread, time};
use image::imageops::{resize, grayscale};
use image::open;
use terminal_size::{Width, Height, terminal_size};

pub fn split_frames(file: String, framerate: Option<i32>) {
    let framerate = match framerate {
        Some(v) => v,
        None => 10,
    };

    let _ = fs::remove_dir_all("/tmp/termplay.cache");

    fs::create_dir("/tmp/termplay.cache")
        .expect("could not create cache folder to store frame data");

    //ffmpeg -i 2022-02-05_16-20-01.mp4 -vf fps=10 %03d.jpg
    let frames = format!("fps={framerate}");
    let ffmpeg = Command::new("ffmpeg")
        .args(["-i", &file, "-vf", &frames, "/tmp/termplay.cache/%03d.jpg"])
        .status()
        .expect("failed to run ffmpeg");

    if !ffmpeg.success() {
        panic!("ffmpeg failed to install");
    }
    let total = fs::read_dir("/tmp/termplay.cache").unwrap().count();

    resize_frames(total);
    print_frames(framerate);

}

fn resize_frames(total: usize) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        let mut x = 0;
        for i in fs::read_dir("/tmp/termplay.cache").unwrap() {
            x += 1;
            println!("resizing [{x}/{total}]");
            let path = i.unwrap().path();
            let img = open(&path).expect("image failed to open");
            let img = resize(
                &grayscale(&img), 
                w as u32, 
                h as u32, 
                image::imageops::FilterType::Nearest
            );
            img.save(path).expect("failed to resive frame");
        }
    }
}

pub fn print_frames(framerate: i32) {
    let chars = vec!["#", "&", "@", "$", "%", "*", ".", " "]; 
    let mut frames = fs::read_dir("/tmp/termplay.cache").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
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
}
