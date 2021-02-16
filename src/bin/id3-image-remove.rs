use std::process;
use std::io::{self, Write};
use std::path::PathBuf;

use structopt::StructOpt;
use id3_image::remove_images;

#[derive(StructOpt, Debug)]
#[structopt(name = "id3-image-remove")]
struct Opt {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: i8,

    /// Quiet mode, implies no verbosity, and also no error explanations
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Don't ask for confirmation before removing
    #[structopt(long = "no-confirm")]
    no_confirm: bool,

    /// Music file to remove images from
    #[structopt(name = "music-file.mp3", required = true, parse(from_os_str))]
    music_filename: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let verbosity = if opt.quiet { -1 } else { opt.verbose };
    let confirm = !opt.no_confirm;

    if verbosity >= 0 && confirm {
        print_prompt();
        let mut input = String::new();
        while let Err(_) = io::stdin().read_line(&mut input) {
            if verbosity >= 0 {
                eprintln!("Could not read your input, please try again.");
            }
            print_prompt();
        }

        let choice = input.to_lowercase().trim().chars().next().unwrap_or('y');
        if choice != 'y' {
            if verbosity >= 1 {
                println!("Exiting without removing images");
            }
            return;
        }
    }

    if let Err(e) = remove_images(&opt.music_filename) {
        if verbosity >= 0 {
            eprintln!("{}", e);
        }
        process::exit(1);
    }
    if verbosity >= 1 {
        println!("Removed images on {:?}", opt.music_filename);
    }
}

fn print_prompt() {
    print!("Are you sure you'd like to clear all embedded images? [Y/n] ");
    io::stdout().flush().unwrap();
}
