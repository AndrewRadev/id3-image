use std::error::Error;

pub fn embed_image(music_filename: &str, image_filename: &str) -> Result<(), Box<Error>> {
    let mut tag = id3::Tag::read_from_path(&music_filename).
        map_err(|e| format!("Error reading music file {}: {}", music_filename, e))?;

    let image = image::open(&image_filename).
        map_err(|e| format!("Error reading image {}: {}", image_filename, e))?;

    let mut encoded_image_bytes = Vec::new();
    // Unwrap: Writing to a Vec should always succeed;
    image.write_to(&mut encoded_image_bytes, image::ImageOutputFormat::JPEG(90)).unwrap();

    tag.add_picture(id3::frame::Picture {
        mime_type: "image/jpeg".to_string(),
        picture_type: id3::frame::PictureType::CoverFront,
        description: String::new(),
        data: encoded_image_bytes,
    });

    tag.write_to_path(music_filename, id3::Version::Id3v23).
        map_err(|e| format!("Error writing image to file: {}", e))?;

    Ok(())
}

pub fn extract_image(music_filename: &str, image_filename: &str) -> Result<(), Box<Error>> {
    let tag = id3::Tag::read_from_path(&music_filename).
        map_err(|e| format!("Error reading music file {}: {}", music_filename, e))?;

    let first_picture = tag.pictures().next();

    if let Some(p) = first_picture {
        match image::load_from_memory(&p.data) {
            Ok(image) => {
                image.save(&image_filename);
                println!("{}", image_filename);
            },
            Err(e) => return Err(format!("Couldn't load image: {}", e).into()),
        };
    }

    Ok(())
}
