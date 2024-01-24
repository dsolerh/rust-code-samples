use clap::{App, Arg};

type MyResult<T> = Result<T, String>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Daniel Soler <dolerh.cinter95@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
            .value_name("FILES").help("files to concatenate, if not specified the stdin will be used as the source")
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("add line numbers for all lines in the resulting file, defaults to false")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .help("add line numbers for the non blank lines in the resulting file, defaults to false")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .values_of_lossy("files")
            .ok_or("The argument was not provided at runtime")?,
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}
