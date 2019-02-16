use std::env;
use std::process;
use std::path::PathBuf;

use id3_image::extract_image;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("USAGE: id3-image-extract <mp3-file> [image-file]");
        process::exit(1);
    }

    let music_filename = args[1].clone();
    let image_filename = args.get(2).cloned().unwrap_or_else(|| replace_extension(&music_filename, "jpg"));

    if let Err(e) = extract_image(&music_filename, &image_filename) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn replace_extension(path: &str, replacement: &str) -> String {
    let mut path = PathBuf::from(&path);
    path.set_extension(replacement);
    path.to_string_lossy().to_string()
}
