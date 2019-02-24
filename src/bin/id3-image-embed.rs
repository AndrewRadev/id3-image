use std::process;
use std::path::PathBuf;

use structopt::StructOpt;
use id3_image::embed_image;

#[derive(StructOpt, Debug)]
#[structopt(name = "id3-image-embed")]
struct Opt {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Music file to embed image into
    #[structopt(name = "music-file.mp3", required = true, parse(from_os_str))]
    music_filename: PathBuf,

    /// Image file to embed
    #[structopt(name = "image-file.jpg", required = true, parse(from_os_str))]
    image_filename: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    if let Err(e) = embed_image(&opt.music_filename, &opt.image_filename) {
        eprintln!("{}", e);
        process::exit(1);
    }
    if opt.verbose >= 1 {
        println!("Embedded {:?} into {:?}", opt.image_filename, opt.music_filename);
    }
}
