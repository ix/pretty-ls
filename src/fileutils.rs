extern crate ansi_term;
extern crate time;

use std::fs;
use std::os::unix::fs::{PermissionsExt, MetadataExt};
use self::ansi_term::Colour;

pub trait Printer {
    fn print(&self, filename: &String);
    fn relative(&self) -> (String, Colour);
    fn formatted_size(&self) -> (String, Colour);
}

impl Printer for fs::Metadata {
    fn print(&self, filename: &String) {
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
            print!("{}", Colour::Blue.paint("d"));
        }

        else if filetype.is_symlink() {
            print!("{}", Colour::Green.paint("l"));
        }

        else {
            print!("{}", Colour::Black.bold().paint("-"));
        }

        // I'm sorry.
        macro_rules! permission {
            ($bit:ident, $color:ident, $char:expr) => (
                if permissions & $bit != 0 {
                    print!("{}", Colour::$color.paint($char));
                }

                else {
                    print!("{}", Colour::Black.bold().paint("-"));
                }
            )
        }

        permission!(S_IRUSR, Red, "r");
        permission!(S_IWUSR, Yellow, "w");
        permission!(S_IXUSR, White, "x");

        permission!(S_IRGRP, Red, "r");
        permission!(S_IWGRP, Yellow, "w");
        permission!(S_IXGRP, White, "x");

        permission!(S_IROTH, Red, "r");
        permission!(S_IWOTH, Yellow, "w");
        permission!(S_IXOTH, White, "x");

        let (s, c) = self.relative(); {
            print!("{}", c.paint(s));
        }

        let (s, c) = self.formatted_size(); {
            print!("{}", c.paint(s));
        }

        macro_rules! fancy {
            ($filename:expr) => (
                if filetype.is_dir() {
                    print!(" {}", Colour::Blue.paint($filename));
                    print!("/");
                }

                else if filetype.is_symlink() {
                    print!(" {}", Colour::Purple.paint($filename));
                    print!("@");
                }

                else if (permissions & S_IXUSR) != 0 {
                    print!(" {}", Colour::Green.paint($filename));
                    print!("*");
                }

                else {
                    print!(" {}", $filename);
                }
            )
        }

        fancy!(filename.as_str());
        print!("\n");
    }

    fn relative(&self) -> (String, Colour) {
        let diff = time::get_time().sec - self.mtime();
        let day_diff = diff / 86400;

        match diff {
            0 ... 59 => (format!("{:>3}s", diff), Colour::White),
            60 ... 3600 => (format!("{:>3}m", diff / 60), Colour::Red),
            3600 ... 86400 => (format!("{:>3}h", diff / 3600), Colour::Yellow),
            _ => {
                if day_diff > 7 {
                    (format!("{:>3}w", day_diff / 7), Colour::Black) // bright
                }
                
                else {
                    (format!("{:>3}d", day_diff), Colour::Green)
                }
            }
        }
    }

    fn formatted_size(&self) -> (String, Colour) {
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

        match bytes {
            0.0 ... B => (sizify!(bytes, "B"), Colour::White),
            B ... K => (sizify!(bytes / B, "K"), Colour::Yellow),
            K ... M => (sizify!(bytes / K, "M"), Colour::Red),
            M ... G => (sizify!(bytes / M, "G"), Colour::Purple),
            G ... T => (sizify!(bytes / G, "T"), Colour::Green),
            T ... P => (sizify!(bytes / T, "P"), Colour::Cyan),
            P ... E => (sizify!(bytes / P, "E"), Colour::Green), // bright
            E ... Z => (sizify!(bytes / E, "Z"), Colour::Red), // bright
            Z ... Y => (sizify!(bytes / Z, "Y"), Colour::White), // bright
            _ => (sizify!(bytes / M, "G"), Colour::Blue)
        }
    }
}
