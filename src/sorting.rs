use std::fs;
use std::cmp::{Ordering, Ord};

pub enum SortType {
    Name,
    Unsorted
}

pub trait Sorter {
    fn sort(&mut self, method: SortType) -> Vec<fs::DirEntry>;
}

impl Sorter for fs::ReadDir {
    fn sort(&mut self, method: SortType) -> Vec<fs::DirEntry> {
        match method {
            SortType::Name => self.sort_by_name(),
            SortType::Unsorted => self.unsorted()
        }
    }
}

trait RawSorter {
    fn sort_by_name(&mut self) -> Vec<fs::DirEntry>;
    fn unsorted(&mut self) -> Vec<fs::DirEntry>;
}

impl RawSorter for fs::ReadDir {
    fn sort_by_name(&mut self) -> Vec<fs::DirEntry> {
        let mut files = Vec::new();

        for file in self {
            match file {
                Ok(file) => files.push(file),
                _ => {}
            }
        }

        &files.sort_by(|a, b| {
            if let Ok(a_filename) = a.file_name().into_string() {
                if let Ok(b_filename) = b.file_name().into_string() {
                    a_filename.cmp(&b_filename)
                }

                else {
                    Ordering::Equal
                }
            }

            else {
                Ordering::Equal
            }
        });

        files
    }

    fn unsorted(&mut self) -> Vec<fs::DirEntry> {
        let mut files = Vec::new();

        for file in self {
            match file {
                Ok(file) => files.push(file),
                _ => {}
            }
        }

        files
    }
}
