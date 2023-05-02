use cmd_basic::{run, Config};
use std::{env, process};

fn main() {
    let cfg = Config::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(cfg) {
        eprintln!("Problem reading file: {}", e);
        process::exit(1);
    }
}
