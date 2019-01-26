use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        eprintln!("USAGE: id3-image <mp3-file> <image-file>");
        process::exit(1);
    }

    let music_filename = args[1].clone();
    let image_filename = args[2].clone();

    let mut tag = match id3::Tag::read_from_path(&music_filename) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading music file {}: {}", music_filename, e);
            process::exit(1);
        },
    };

    let image = match image::open(&image_filename) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Error reading image {}: {}", image_filename, e);
            process::exit(1);
        },
    };

    let mut encoded_image_bytes = Vec::new();
    // Unwrap: Writing to a Vec should always succeed;
    image.write_to(&mut encoded_image_bytes, image::ImageOutputFormat::JPEG(90)).unwrap();

    tag.add_picture(id3::frame::Picture {
        mime_type: "image/jpeg".to_string(),
        picture_type: id3::frame::PictureType::Other,
        description: String::new(),
        data: encoded_image_bytes,
    });

    if let Err(e) = tag.write_to_path(music_filename, id3::Version::Id3v23) {
        eprintln!("Error writing image to file: {}", e);
        process::exit(1);
    }
}
