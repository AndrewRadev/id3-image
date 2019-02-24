use std::env;
use std::fs;
use std::path::{PathBuf, Path};
use std::ops::Deref;

use tempfile::TempDir;
use id3_image::*;

struct Fixture {
    path_buf: PathBuf,
    _tempdir: TempDir,
}

impl AsRef<Path> for Fixture {
    fn as_ref(&self) -> &Path {
        self.path_buf.as_path()
    }
}

impl Deref for Fixture {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.path_buf.deref()
    }
}

macro_rules! fixture {
    ($filename:expr) => {
        {
            let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
            let mut source_path = PathBuf::from(root_dir);
            source_path.push("tests/fixtures");
            source_path.push($filename);

            let tempdir = tempfile::tempdir().unwrap();
            let mut target_path = PathBuf::from(&tempdir.path());
            target_path.push($filename);

            fs::copy(&source_path, &target_path).unwrap();

            Fixture { _tempdir: tempdir, path_buf: target_path }
        }
    }
}

fn read_tag(path: &Path) -> id3::Tag {
    id3::Tag::read_from_path(path).unwrap()
}

#[test]
fn test_unsuccessful_image_embedding() {
    let song  = fixture!("attempt_1_no_image.mp3");
    let image = fixture!("attempt_1.jpg");

    // Nonexistent files
    assert!(embed_image(&song, &PathBuf::from("nonexistent.jpg")).is_err());
    assert!(embed_image(&PathBuf::from("nonexistent.mp3"), &image).is_err());
    assert!(embed_image(&PathBuf::from("nonexistent.mp3"), &PathBuf::from("nonexistent.jpg")).is_err());

    // Wrong kinds of files
    assert!(embed_image(&image, &song).is_err());
    assert!(embed_image(&song, &song).is_err());
    assert!(embed_image(&image, &image).is_err());
}

#[test]
fn test_successful_jpeg_image_embedding() {
    let song  = fixture!("attempt_1_no_image.mp3");
    let image = fixture!("attempt_1.jpg");

    let tag = read_tag(&song);
    assert!(tag.pictures().count() == 0);

    embed_image(&song, &image).unwrap();

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);
}

#[test]
fn test_successful_png_image_embedding() {
    let song  = fixture!("attempt_1_no_image.mp3");
    let image = fixture!("attempt_1.png");

    let tag = read_tag(&song);
    assert!(tag.pictures().count() == 0);

    embed_image(&song, &image).unwrap();

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);
}

#[test]
fn test_successful_image_embedding_in_a_file_that_already_has_an_image() {
    let song  = fixture!("attempt_1.mp3");
    let image = fixture!("attempt_1.jpg");

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);

    embed_image(&song, &image).unwrap();

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);
}

#[test]
fn test_remove_and_add_image() {
    let song  = fixture!("attempt_1.mp3");
    let image = fixture!("attempt_1.jpg");

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);

    remove_images(&song).unwrap();

    let tag = read_tag(&song);
    assert!(tag.pictures().count() == 0);

    embed_image(&song, &image).unwrap();

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);
}
