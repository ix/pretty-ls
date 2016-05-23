extern crate term;
extern crate time;

use std::fs;
use std::io;
use std::path::Path;
use std::convert::AsRef;
use std::os::unix::fs::{PermissionsExt, MetadataExt};

pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(path) {
        Ok(_) => true,
        _     => false
    }
}

pub fn is_directory<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => {
            metadata.is_dir()
        },

        _ => false
    }
}

pub trait Printer {
    fn print(&self, filename: &String, term: &mut Box<term::StdoutTerminal>) -> io::Result<()>;
    fn relative(&self) -> io::Result<(String, term::color::Color)>;
    fn formatted_size(&self) -> io::Result<(String, term::color::Color)>;
}

impl Printer for fs::Metadata {
    fn print(&self, filename: &String, term: &mut Box<term::StdoutTerminal>) -> io::Result<()> {
        const S_IRUSR: u32 = 0o0000400;
        const S_IWUSR: u32 = 0o0000200;
        const S_IXUSR: u32 = 0o0000100;
        const S_IRGRP: u32 = 0o0000040;
        const S_IWGRP: u32 = 0o0000020;
        const S_IXGRP: u32 = 0o0000010;
        const S_IROTH: u32 = 0o0000004;
        const S_IWOTH: u32 = 0o0000002;
        const S_IXOTH: u32 = 0o0000001;

        let filetype = self.file_type();
        let permissions = self.permissions().mode();

        if filetype.is_dir() {
            try!(term.fg(term::color::BLUE));
            print!("d");
            try!(term.reset());
        }

        else if filetype.is_symlink() {
            try!(term.fg(term::color::GREEN));
            print!("l");
            try!(term.reset());
        }

        else {
            try!(term.fg(term::color::BRIGHT_BLACK));
            print!("-");
            try!(term.reset());
        }

        // I'm sorry.
        macro_rules! permission {
            ($bit:ident, $color:ident, $char:expr) => (
                if permissions & $bit != 0 {
                    try!(term.fg(term::color::$color));
                    print!($char);
                    try!(term.reset());
                }

                else {
                    try!(term.fg(term::color::BRIGHT_BLACK));
                    print!("-");
                    try!(term.reset());
                }
            )
        }

        permission!(S_IRUSR, RED, "r");
        permission!(S_IWUSR, YELLOW, "w");
        permission!(S_IXUSR, WHITE, "x");

        permission!(S_IRGRP, RED, "r");
        permission!(S_IWGRP, YELLOW, "w");
        permission!(S_IXGRP, WHITE, "x");

        permission!(S_IROTH, RED, "r");
        permission!(S_IWOTH, YELLOW, "w");
        permission!(S_IXOTH, WHITE, "x");

        match self.relative() {
            Ok((s, c)) => {
                try!(term.fg(c));
                print!("{}", s);
                try!(term.reset());
            },

            Err(_) => {}
        }

        match self.formatted_size() {
            Ok((s, c)) => {
                try!(term.fg(c));
                print!("{}", s);
                try!(term.reset());
            },

            Err(_) => {}
        }

        macro_rules! fancy {
            ($filename:expr) => (
                if filetype.is_dir() {
                    try!(term.fg(term::color::BLUE));
                    print!(" {}", $filename);
                    try!(term.reset());
                    print!("/");
                }

                else if filetype.is_symlink() {
                    try!(term.fg(term::color::MAGENTA));
                    print!(" {}", $filename);
                    try!(term.reset());
                    print!("@");
                }

                else if (permissions & S_IXUSR) != 0 {
                    try!(term.fg(term::color::GREEN));
                    print!(" {}", $filename);
                    try!(term.reset());
                    print!("*");
                }

                else {
                    print!(" {}", $filename);
                }
            )
        }

        fancy!(filename);
        print!("\n");

        Ok(())
    }

    fn relative(&self) -> io::Result<(String, term::color::Color)> {
        let diff = time::get_time().sec - self.mtime();
        let day_diff = diff / 86400;

        Ok(match diff {
            0 ... 59 => (format!("{:>3}s", diff), term::color::WHITE),
            60 ... 3600 => (format!("{:>3}m", diff / 60), term::color::RED),
            3600 ... 86400 => (format!("{:>3}h", diff / 3600), term::color::YELLOW),
            _ => {
                if day_diff > 7 {
                    (format!("{:>3}w", day_diff / 7), term::color::BRIGHT_BLACK)
                }

                else {
                    (format!("{:>3}d", day_diff), term::color::GREEN)
                }
            }
        })
    }

    fn formatted_size(&self) -> io::Result<(String, term::color::Color)> {
        let bytes: f64 = self.size() as f64;
        const B: f64 = 1024.0;
        const K: f64 = B * B;
        const M: f64 = K * B;
        const G: f64 = M * B;
        const T: f64 = G * B;
        const P: f64 = T * B;
        const E: f64 = P * B;
        const Z: f64 = E * B;
        const Y: f64 = Z * B;

        macro_rules! sizify {
            ($value:expr, $size:expr) => (
                if $value > 10.0 {
                    format!("{:>4.0}{}", $value.round(), $size)
                }

                else {
                    format!("{:>4.1}{}", $value, $size)
                }
            )
        }

        Ok(match bytes {
            0.0 ... B => (sizify!(bytes, "B"), term::color::WHITE),
            B ... K => (sizify!(bytes / B, "K"), term::color::YELLOW),
            K ... M => (sizify!(bytes / K, "M"), term::color::RED),
            M ... G => (sizify!(bytes / M, "G"), term::color::MAGENTA),
            G ... T => (sizify!(bytes / G, "T"), term::color::GREEN),
            T ... P => (sizify!(bytes / T, "P"), term::color::CYAN),
            P ... E => (sizify!(bytes / P, "E"), term::color::BRIGHT_GREEN),
            E ... Z => (sizify!(bytes / E, "Z"), term::color::BRIGHT_RED),
            Z ... Y => (sizify!(bytes / Z, "Y"), term::color::BRIGHT_WHITE),
            _ => (sizify!(bytes / M, "G"), term::color::BLUE)
        })
    }
}
