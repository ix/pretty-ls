extern crate getopts;

mod fileutils;
mod sorting;

use getopts::Options;

use std::env;
use std::fs::{read_dir, DirEntry};
use std::path::Path;

use fileutils::Printer;
use sorting::{Sorter, SortType, RawFilter};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();

    // Initial (unsorted) sort mode.
    let mut sortmode: SortType = SortType::Unsorted;
    let mut dotfiles: bool = false;
    let mut opts = Options::new();

    opts.optopt("s", "sort", "set sort mode to one of: size, name, modified", "MODE");
    opts.optflag("a", "all", "show all files, including dotfiles");
    opts.optflag("h", "help", "print the help screen");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        print!("{}", opts.usage(&*format!("usage: {} [opts] [files]", program)));
        return;
    }

    if matches.opt_present("a") {
        dotfiles = true;
    }

    // If a valid sort mode was given, mutate the sortmode.
    if let Some(sortvalue) = matches.opt_str("s") {
        sortmode = match &*sortvalue {
            "size" => SortType::Size,
            "name" => SortType::Name,
            "modified" => SortType::Modified,
            _ => SortType::Unsorted
        };
    }

    if matches.free.is_empty() {
        // May as well return since we're just
        // printing the current directory.
        let mut files = read_dir(".").unwrap().sort(&sortmode);

        if dotfiles { 
            ls_dir(&files);
        }

        else {
            ls_dir(&files.dotfilter());
        }
        
        return;
    }

    for arg in &matches.free {
        if !Path::new(arg).exists() {
            println!("No such file or directory: {}", arg);
        }

        else {
            if Path::new(arg).is_dir() {
                let mut files = read_dir(arg).unwrap().sort(&sortmode);
                
                println!("{}:", arg);
                if dotfiles {
                    ls_dir(&files);
                }

                else {
                    ls_dir(&files.dotfilter());
                }
            }

            else {
                match std::fs::metadata(arg) {
                    Ok(m) => m.print(arg),
                    _ => panic!("Couldn't get metadata.")
                }
            }
        }
    }
}


// Accepts a Vec of DirEntry rather than a ReadDir so
// that sorting can be applied beforehand.
fn ls_dir(dir: &Vec<DirEntry>) {
    for item in dir {
        if let Ok(m) = item.metadata() {
            m.print(&item.file_name().into_string().unwrap());
        }
    }
}
