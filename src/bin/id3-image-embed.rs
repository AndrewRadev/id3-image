use std::env;
use std::process;

use id3_image::embed_image;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        eprintln!("USAGE: id3-image-embed <mp3-file> <image-file>");
        process::exit(1);
    }

    let music_filename = args[1].clone();
    let image_filename = args[2].clone();

    if let Err(e) = embed_image(&music_filename, &image_filename) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
