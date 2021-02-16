use std::process;
use std::path::PathBuf;

use structopt::StructOpt;
use id3_image::embed_image;

#[derive(StructOpt, Debug)]
#[structopt(name = "id3-image-embed")]
struct Opt {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: i8,

    /// Quiet mode, implies no verbosity, and also no error explanations
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Music file to embed image into
    #[structopt(name = "music-file.mp3", required = true, parse(from_os_str))]
    music_filename: PathBuf,

    /// Image file to embed. If not given, will default to the music filename with a .jpg extension
    #[structopt(name = "image-file.jpg", parse(from_os_str))]
    image_filename: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let verbosity = if opt.quiet { -1 } else { opt.verbose };
    let music_filename = opt.music_filename;
    let image_filename = opt.image_filename.
        unwrap_or_else(|| music_filename.with_extension("jpg"));

    if let Err(e) = embed_image(&music_filename, &image_filename) {
        if verbosity >= 0 {
            eprintln!("{}", e);
        }
        process::exit(1);
    }
    if verbosity >= 1 {
        println!("Embedded {:?} into {:?}", image_filename, music_filename);
    }
}
