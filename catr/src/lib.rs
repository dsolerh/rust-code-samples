use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buf) => {
                let mut line_number = 0;
                for result_line in buf.lines() {
                    let line = result_line?;

                    if config.number_lines || config.number_nonblank_lines && !line.is_empty() {
                        line_number += 1;
                        println!("{:>6}\t{}", line_number, line);
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub number_lines: bool,
    pub number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Daniel Soler <dolerh.cinter95@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("files to concatenate, if not specified the stdin will be used as the source")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("add line numbers for all lines in the resulting file, defaults to false")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("add line numbers for the non blank lines in the resulting file, defaults to false")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
