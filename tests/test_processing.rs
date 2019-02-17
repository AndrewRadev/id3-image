use std::env;
use std::fs;
use std::path::PathBuf;

use id3_image::*;

macro_rules! in_temp_dir {
    ($block:block) => {
        let tmpdir = env::temp_dir();
        env::set_current_dir(&tmpdir).unwrap();
        $block;
    }
}

macro_rules! fixture {
    ($filename:expr) => {
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source_path = PathBuf::from(root_dir);
        source_path.push("tests/fixtures");
        source_path.push($filename);

        fs::copy(source_path, $filename).unwrap();
    }
}

#[test]
fn test_successful_jpeg_image_embedding() {
    in_temp_dir!({
        fixture!("attempt_1_no_image.mp3");
        fixture!("attempt_1.jpg");

        {
            let tag = id3::Tag::read_from_path("attempt_1_no_image.mp3").unwrap();
            assert!(tag.pictures().count() == 0);
        }

        embed_image("attempt_1_no_image.mp3", "attempt_1.jpg").unwrap();

        {
            let tag = id3::Tag::read_from_path("attempt_1_no_image.mp3").unwrap();
            assert!(tag.pictures().count() > 0);
        }
    });
}

#[test]
fn test_successful_png_image_embedding() {
    in_temp_dir!({
        fixture!("attempt_1_no_image.mp3");
        fixture!("attempt_1.png");

        {
            let tag = id3::Tag::read_from_path("attempt_1_no_image.mp3").unwrap();
            assert!(tag.pictures().count() == 0);
        }

        embed_image("attempt_1_no_image.mp3", "attempt_1.png").unwrap();

        {
            let tag = id3::Tag::read_from_path("attempt_1_no_image.mp3").unwrap();
            assert!(tag.pictures().count() > 0);
        }
    });
}

#[test]
fn test_successful_image_embedding_in_a_file_that_already_has_an_image() {
    in_temp_dir!({
        fixture!("attempt_1.mp3");
        fixture!("attempt_1.jpg");

        {
            let tag = id3::Tag::read_from_path("attempt_1.mp3").unwrap();
            assert!(tag.pictures().count() > 0);
        }

        embed_image("attempt_1.mp3", "attempt_1.jpg").unwrap();

        {
            let tag = id3::Tag::read_from_path("attempt_1.mp3").unwrap();
            assert!(tag.pictures().count() > 0);
        }
    });
}

#[test]
fn test_unsuccessful_image_embedding() {
    in_temp_dir!({
        fixture!("attempt_1_no_image.mp3");
        fixture!("attempt_1.jpg");

        // Nonexistent files
        assert!(embed_image("attempt_1_no_image.mp3", "nonexisting.jpg").is_err());
        assert!(embed_image("nonexisting.mp3", "attempt_1.jpg").is_err());
        assert!(embed_image("nonexisting.mp3", "nonexisting.jpg").is_err());

        // Wrong kinds of files
        assert!(embed_image("attempt_1.jpg", "attempt_1_no_image.mp3").is_err());
        assert!(embed_image("attempt_1_no_image.mp3", "attempt_1_no_image.mp3").is_err());
        assert!(embed_image("attempt_1.jpg", "attempt_1.jpg").is_err());
    });
}
