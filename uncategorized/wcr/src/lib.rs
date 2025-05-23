use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::AddAssign,
};

use anyhow::Result;
use clap::Parser;

/// Arguments for the app
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// the files to search
    #[arg(default_values_t = ["-".to_owned()])]
    files: Vec<String>,
    ///
    #[arg(short, long)]
    lines: bool,
    ///
    #[arg(short, long)]
    words: bool,
    ///
    #[arg(short = 'c', long)]
    bytes: bool,
    ///
    #[arg(short = 'm', long, conflicts_with = "bytes")]
    chars: bool,
}

impl Args {
    pub fn new() -> Self {
        let mut args = Args::parse();

        if [args.lines, args.words, args.bytes, args.chars]
            .iter()
            .all(|v| !v)
        {
            args.lines = true;
            args.words = true;
            args.bytes = true;
        }

        return args;
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

impl FileInfo {
    fn new() -> Self {
        FileInfo::default()
    }
}

impl AddAssign for FileInfo {
    fn add_assign(&mut self, rhs: FileInfo) {
        self.num_bytes += rhs.num_bytes;
        self.num_chars += rhs.num_chars;
        self.num_lines += rhs.num_lines;
        self.num_words += rhs.num_words;
    }
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

pub fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;

        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn run(args: Args) -> Result<()> {
    let mut total_info = FileInfo::new();

    for filename in &args.files {
        let file = open(filename)?;
        let info = count(file)?;
        println!(
            "{}{}{}{}{}",
            format_field(info.num_lines, args.lines),
            format_field(info.num_words, args.words),
            format_field(info.num_bytes, args.bytes),
            format_field(info.num_chars, args.chars),
            if filename == "-" {
                "".to_string()
            } else {
                format!(" {}", filename)
            }
        );

        total_info += info
    }

    if args.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_info.num_lines, args.lines),
            format_field(total_info.num_words, args.words),
            format_field(total_info.num_bytes, args.bytes),
            format_field(total_info.num_chars, args.chars),
        )
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
