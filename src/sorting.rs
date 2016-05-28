use std::fs;
use std::cmp::{Ordering, Ord};
use std::os::unix::fs::MetadataExt;

#[allow(dead_code)]
pub enum SortType {
    Name,
    Size,
    Modified,
    Unsorted
}

pub trait Sorter {
    fn sort(&mut self, method: &SortType) -> Vec<fs::DirEntry>;
}

impl Sorter for fs::ReadDir {
    fn sort(&mut self, method: &SortType) -> Vec<fs::DirEntry> {
        match *method {
            SortType::Name => self.sort_by_name(),
            SortType::Size => self.sort_by_size(),
            SortType::Modified => self.sort_by_modified(),
            SortType::Unsorted => self.unsorted()
        }
    }
}

pub trait RawFilter {
    fn dotfilter(&mut self) -> &Vec<fs::DirEntry>;
}

impl RawFilter for Vec<fs::DirEntry> {
    fn dotfilter(&mut self) -> &Vec<fs::DirEntry> {
        self.retain(|file| {
            if let Ok(filename) = file.file_name().into_string() {
                if filename.as_bytes()[0] == 46 {
                    return false
                }

                else {
                    return true
                }
            }

            else {
                return true
            }
        });
        
        self
    }
}

trait RawSorter {
    fn sort_by_name(&mut self) -> Vec<fs::DirEntry>;
    fn sort_by_size(&mut self) -> Vec<fs::DirEntry>;
    fn sort_by_modified(&mut self) -> Vec<fs::DirEntry>;
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

    fn sort_by_size(&mut self) -> Vec<fs::DirEntry> {
        let mut files = Vec::new();

        for file in self {
            match file {
                Ok(file) => files.push(file),
                _ => {}
            }
        }

        &files.sort_by(|a, b| {
            if let (Ok(m1), Ok(m2)) = (a.metadata(), b.metadata()) {
                m1.len().cmp(&m2.len())
            }

            else {
                Ordering::Equal
            }
        });

        files
    }

    fn sort_by_modified(&mut self) -> Vec<fs::DirEntry> {
        let mut files = Vec::new();

        for file in self {
            match file {
                Ok(file) => files.push(file),
                _ => {}
            }
        }

        &files.sort_by(|a, b|{
            if let (Ok(m1), Ok(m2)) = (a.metadata(), b.metadata()) {
                m1.mtime().cmp(&m2.mtime())
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
