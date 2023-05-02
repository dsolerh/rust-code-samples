use std::env;
use std::fs;
use regex::Regex;
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e)=> {
            eprintln!("{} failed to read from file '{}': {:?}",
                "Error".red().bold(), args.filename, e);
            std::process::exit(1);
       }
    };

    let replaced_data=match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", 
                "Error".red().bold(), e);
            std::process::exit(1);
        },
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}",
                "Error".red().bold(), args.output, e);
            std::process::exit(1);
        },
    };
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String,regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        print_usage(&args[0]);
        eprintln!(
            "{} wrong number of arguments: expected 4 but got {}.",
            "Error".red().bold(),
            args.len() - 1
        );
        std::process::exit(1);
    }

    Arguments {
        target: args[1].clone(),
        replacement: args[2].clone(),
        filename: args[3].clone(),
        output: args[4].clone(),
    }
}

fn print_usage(app_name: &str) {
    eprintln!(
        "{} - change ocurrences of one string into another",
        app_name.green()
    );
    eprintln!(
        "Usage: {} <target> <replacement> <INPUT> <OUTPUT>",
        app_name
    );
}
