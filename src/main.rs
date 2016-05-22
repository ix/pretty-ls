extern crate term;
mod fileutils;
mod sorting;

use std::fs::{read_dir, DirEntry};
use fileutils::Printer;
use sorting::{Sorter, SortType};

fn main() {
    if let Some(mut console) = term::stdout() {
        if std::env::args().len() == 1 {
            // Just print the current directory.
            ls_dir(&read_dir(".").unwrap().sort(SortType::Modified), &mut console);
        }

        else {
            // Skip the name of the program.
            for arg in std::env::args().skip(1) {
                let exists = fileutils::exists(&arg);
                let directory = fileutils::is_directory(&arg);

                // Directory doesn't exist so we'll just print
                // a warning, like GNU ls does. But prettier.
                if !exists {
                    console.fg(term::color::RED).unwrap();
                    println!("cannot access `{}: No such file or directory", &arg);
                    console.reset().unwrap();
                }

                else {
                    // If the argument is a directory then print its
                    // contents as we would the current one.
                    if directory {
                        console.fg(term::color::BLUE).unwrap();
                        println!("{}:", &arg);
                        console.reset().unwrap();
                        ls_dir(&read_dir(&arg).unwrap().sort(SortType::Modified), &mut console);
                        print!("\n");
                    }

                    // TODO: construct a DirEntry from the filename
                    // and print as we would normally.
                    else {
                        console.fg(term::color::CYAN).unwrap();
                        println!("{}", &arg);
                        console.reset().unwrap();
                    }
                }
            }
        }
    }

    else {
        println!("Couldn't open a terminal.")
    }
}


// Accepts a Vec of DirEntry rather than a ReadDir so
// that sorting can be applied beforehand.
fn ls_dir(dir: &Vec<DirEntry>, mut term: &mut Box<term::StdoutTerminal>) {
    for item in dir {
        item.print(&mut term).unwrap();
        print!("\n");
    }
}
