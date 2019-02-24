use std::process;
use std::path::PathBuf;

use structopt::StructOpt;
use id3_image::extract_first_image;

#[derive(StructOpt, Debug)]
#[structopt(name = "id3-image-embed")]
struct Opt {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Music file to extract image from
    #[structopt(name = "music-file.mp3", required = true, parse(from_os_str))]
    music_filename: PathBuf,

    /// (Optional) Output image: defaults to music filename with .jpg extension
    #[structopt(name = "image-file.jpg", parse(from_os_str))]
    image_filename: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let image_filename = opt.image_filename.clone().
        unwrap_or_else(|| opt.music_filename.with_extension("jpg"));

    if let Err(e) = extract_first_image(&opt.music_filename, &image_filename) {
        eprintln!("{}", e);
        process::exit(1);
    }

    if opt.verbose == 1 {
        // then just print the output filename for scripting purposes:
        println!("{}", image_filename.display());
    } else if opt.verbose >= 2 {
        // show a longer status message:
        println!("Extracted cover art from {:?} to {:?}", opt.music_filename, image_filename);
    }
}
