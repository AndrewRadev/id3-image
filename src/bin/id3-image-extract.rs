use std::env;
use std::process;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("USAGE: id3-image-extract <mp3-file> [image-file]");
        process::exit(1);
    }

    let music_filename = args[1].clone();
    let image_filename = args.get(2).cloned().unwrap_or_else(|| replace_extension(&music_filename, "jpg"));

    let tag = match id3::Tag::read_from_path(&music_filename) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading music file {}: {}", music_filename, e);
            process::exit(1);
        },
    };

    let first_picture = tag.pictures().next();

    if let Some(p) = first_picture {
        match image::load_from_memory(&p.data) {
            Ok(image) => {
                image.save(&image_filename);
                println!("{}", image_filename);
            },
            Err(e) => {
                eprintln!("Couldn't load image: {}", e);
                process::exit(1);
            }
        };
    }
}

fn replace_extension(path: &str, replacement: &str) -> String {
    let mut path = PathBuf::from(&path);
    path.set_extension(replacement);
    path.to_string_lossy().to_string()
}
