use std::env;
use std::fs;
use std::path::{PathBuf, Path};
use std::ops::Deref;

use tempfile::TempDir;
use id3_image::*;

struct Fixture {
    path: PathBuf,
    source: PathBuf,
    _tempdir: TempDir,
}

impl Fixture {
    fn blank(fixture_filename: &str) -> Self {
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source = PathBuf::from(root_dir);
        source.push("tests/fixtures");
        source.push(&fixture_filename);

        let tempdir = tempfile::tempdir().unwrap();
        let mut path = PathBuf::from(&tempdir.path());
        path.push(&fixture_filename);

        Fixture { _tempdir: tempdir, source, path }
    }

    fn copy(fixture_filename: &str) -> Self {
        let fixture = Fixture::blank(fixture_filename);
        fs::copy(&fixture.source, &fixture.path).unwrap();
        fixture
    }
}

impl Deref for Fixture {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.path.deref()
    }
}

fn read_tag(path: &Path) -> id3::Tag {
    id3::Tag::read_from_path(path).unwrap()
}

#[test]
fn test_unsuccessful_image_embedding() {
    let song  = Fixture::copy("attempt_1_no_image.mp3");
    let image = Fixture::copy("attempt_1.jpg");

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
    let song  = Fixture::copy("attempt_1_no_image.mp3");
    let image = Fixture::copy("attempt_1.jpg");

    let tag = read_tag(&song);
    assert!(tag.pictures().count() == 0);

    embed_image(&song, &image).unwrap();

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);
}

#[test]
fn test_successful_png_image_embedding() {
    let song  = Fixture::copy("attempt_1_no_image.mp3");
    let image = Fixture::copy("attempt_1.png");

    let tag = read_tag(&song);
    assert!(tag.pictures().count() == 0);

    embed_image(&song, &image).unwrap();

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);
}

#[test]
fn test_successful_image_embedding_in_a_file_that_already_has_an_image() {
    let song  = Fixture::copy("attempt_1.mp3");
    let image = Fixture::copy("attempt_1.jpg");

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);

    embed_image(&song, &image).unwrap();

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);
}

#[test]
fn test_removing_and_adding_an_image() {
    let song  = Fixture::copy("attempt_1.mp3");
    let image = Fixture::copy("attempt_1.jpg");

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);

    remove_images(&song).unwrap();

    let tag = read_tag(&song);
    assert!(tag.pictures().count() == 0);

    embed_image(&song, &image).unwrap();

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);
}

#[test]
fn test_extracting_a_jpg_image() {
    let song  = Fixture::copy("attempt_1.mp3");
    let image = Fixture::blank("attempt_1.jpg");

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);
    assert!(!image.exists());

    extract_first_image(&song, &image).unwrap();

    assert!(image.exists());
}

#[test]
fn test_extracting_a_png_image() {
    let song  = Fixture::copy("attempt_1.mp3");
    let image = Fixture::blank("attempt_1.png");

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);
    assert!(!image.exists());

    extract_first_image(&song, &image).unwrap();

    assert!(image.exists());
}

#[test]
fn test_overwriting_an_existing_image() {
    let song  = Fixture::copy("attempt_1.mp3");
    let image = Fixture::copy("attempt_1.jpg");

    let tag = read_tag(&song);
    assert!(tag.pictures().count() > 0);
    assert!(image.exists());

    extract_first_image(&song, &image).unwrap();

    assert!(image.exists());
}
