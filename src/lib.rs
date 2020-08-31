use std::path::Path;
use std::error::Error;

/// Embed the image from `image_filename` into `music_filename`, in-place. Any errors reading ID3
/// tags from the music file or parsing the image get propagated upwards.
///
/// The image is encoded as a JPEG with a 90% quality setting, and embedded as a "Front cover".
/// Tags get written as ID3v2.3.
///
pub fn embed_image(music_filename: &Path, image_filename: &Path) -> Result<(), Box<dyn Error>> {
    let mut tag = id3::Tag::read_from_path(&music_filename).
        map_err(|e| format!("Error reading music file {:?}: {}", music_filename, e))?;

    let image = image::open(&image_filename).
        map_err(|e| format!("Error reading image {:?}: {}", image_filename, e))?;

    let mut encoded_image_bytes = Vec::new();
    // Unwrap: Writing to a Vec should always succeed;
    image.write_to(&mut encoded_image_bytes, image::ImageOutputFormat::Jpeg(90)).unwrap();

    tag.add_picture(id3::frame::Picture {
        mime_type: "image/jpeg".to_string(),
        picture_type: id3::frame::PictureType::CoverFront,
        description: String::new(),
        data: encoded_image_bytes,
    });

    tag.write_to_path(music_filename, id3::Version::Id3v23).
        map_err(|e| format!("Error writing image to music file {:?}: {}", music_filename, e))?;

    Ok(())
}

/// Extract the first found embedded image from `music_filename` and write it as a file with the
/// given `image_filename`. The image file will be silently overwritten if it exists.
///
/// Any errors from parsing id3 tags will be propagated. The function will also return an error if
/// there's no embedded images in the mp3 file.
///
pub fn extract_first_image(music_filename: &Path, image_filename: &Path) -> Result<(), Box<dyn Error>> {
    let tag = id3::Tag::read_from_path(&music_filename).
        map_err(|e| format!("Error reading music file {:?}: {}", music_filename, e))?;

    let first_picture = tag.pictures().next();

    if let Some(p) = first_picture {
        match image::load_from_memory(&p.data) {
            Ok(image) => {
                image.save(&image_filename).
                    map_err(|e| format!("Couldn't write image file {:?}: {}", image_filename, e))?;
            },
            Err(e) => return Err(format!("Couldn't load image: {}", e).into()),
        };

        Ok(())
    } else {
        Err("No image found in music file".into())
    }
}

/// Remove all embedded images from the given `music_filename`. In effect, this removes all tags of
/// type "APIC".
///
/// If the mp3 file's ID3 tags can't be parsed, the error will be propagated upwards.
///
pub fn remove_images(music_filename: &Path) -> Result<(), Box<dyn Error>> {
    let mut tag = id3::Tag::read_from_path(&music_filename).
        map_err(|e| format!("Error reading music file {:?}: {}", music_filename, e))?;

    tag.remove("APIC");

    tag.write_to_path(music_filename, id3::Version::Id3v23).
        map_err(|e| format!("Error updating music file {:?}: {}", music_filename, e))?;

    Ok(())
}
