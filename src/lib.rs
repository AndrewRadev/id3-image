#![warn(missing_docs)]

//! A command-line tool to embed images into mp3 files. The real work is done by the "id3" crate,
//! but this project makes it easier to deal with embedded cover art in particular.

use std::path::Path;
use std::io::Cursor;

use anyhow::anyhow;
use id3::TagLike;

/// Embed the image from `image_filename` into `music_filename`, in-place. Any errors reading ID3
/// tags from the music file or parsing the image get propagated upwards.
///
/// The image is encoded as a JPEG with a 90% quality setting, and embedded as a "Front cover".
/// Tags get written as ID3v2.3.
///
pub fn embed_image(music_filename: &Path, image_filename: &Path) -> anyhow::Result<()> {
    let mut tag = read_tag(music_filename)?;
    let image = image::open(&image_filename).
        map_err(|e| anyhow!("Error reading image {:?}: {}", image_filename, e))?;

    let mut encoded_image_bytes = Cursor::new(Vec::new());
    // Unwrap: Writing to a Vec should always succeed;
    image.write_to(&mut encoded_image_bytes, image::ImageOutputFormat::Jpeg(90)).unwrap();

    tag.add_frame(id3::frame::Picture {
        mime_type: "image/jpeg".to_string(),
        picture_type: id3::frame::PictureType::CoverFront,
        description: String::new(),
        data: encoded_image_bytes.into_inner(),
    });

    tag.write_to_path(music_filename, id3::Version::Id3v23).
        map_err(|e| anyhow!("Error writing image to music file {:?}: {}", music_filename, e))?;

    Ok(())
}

/// Extract the first found embedded image from `music_filename` and write it as a file with the
/// given `image_filename`. The image file will be silently overwritten if it exists.
///
/// Any errors from parsing id3 tags will be propagated. The function will also return an error if
/// there's no embedded images in the mp3 file.
///
pub fn extract_first_image(music_filename: &Path, image_filename: &Path) -> anyhow::Result<()> {
    let tag = read_tag(music_filename)?;
    let first_picture = tag.pictures().next();

    if let Some(p) = first_picture {
        match image::load_from_memory(&p.data) {
            Ok(image) => {
                image.save(&image_filename).
                    map_err(|e| anyhow!("Couldn't write image file {:?}: {}", image_filename, e))?;
            },
            Err(e) => return Err(anyhow!("Couldn't load image: {}", e)),
        };

        Ok(())
    } else {
        Err(anyhow!("No image found in music file"))
    }
}

/// Remove all embedded images from the given `music_filename`. In effect, this removes all tags of
/// type "APIC".
///
/// If the mp3 file's ID3 tags can't be parsed, the error will be propagated upwards.
///
pub fn remove_images(music_filename: &Path) -> anyhow::Result<()> {
    let mut tag = read_tag(music_filename)?;
    tag.remove("APIC");

    tag.write_to_path(music_filename, id3::Version::Id3v23).
        map_err(|e| anyhow!("Error updating music file {:?}: {}", music_filename, e))?;

    Ok(())
}

fn read_tag(path: &Path) -> anyhow::Result<id3::Tag> {
    id3::Tag::read_from_path(&path).or_else(|e| {
        eprintln!("Warning: file metadata is corrupted, trying to read partial tag: {}", path.display());
        e.partial_tag.clone().ok_or_else(|| anyhow!("Error reading music file {:?}: {}", path, e))
    })
}
