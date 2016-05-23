extern crate getopts;

mod fileutils;
mod sorting;

use getopts::Options;

use std::env;
use std::fs::{read_dir, DirEntry};

use fileutils::Printer;
use sorting::{Sorter, SortType};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();

    // Initial (unsorted) sort mode.
    let mut sortmode: SortType = SortType::Unsorted;
    let mut opts = Options::new();

    opts.optopt("s", "sort", "set sort mode to one of: size, name, modified", "MODE");
    opts.optflag("h", "help", "print the help screen");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        print!("{}", opts.usage(&*format!("usage: {} [opts] [files]", program)));
        return;
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
        ls_dir(&read_dir(".").unwrap().sort(&sortmode));
        return;
    }

    for arg in &matches.free {
        if !fileutils::exists(arg) {
            println!("No such file or directory: {}", arg);
        }

        else {
            if fileutils::is_directory(arg) {
                println!("{}:", arg);
                ls_dir(&read_dir(arg).unwrap().sort(&sortmode));
            }

            else {
                match std::fs::metadata(arg) {
                    Ok(m) => m.print(arg).unwrap(),
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
            m.print(&item.file_name().into_string().unwrap()).unwrap();
        }
    }
}
