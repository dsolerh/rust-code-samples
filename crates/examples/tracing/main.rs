use std::{fmt::Display, thread};

use tracing::{Level, debug, error, info, span, trace, warn};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    println!("Hello, tracing!!");
    // the picks the envents sent by the tracing macros and prints them in the console
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_span_events(FmtSpan::FULL)
        // .pretty()
        .init();

    let span = span!(Level::INFO, "main");
    let _guard = span.enter();
    // this will not get print into the stdout/stderr by default
    debug!("this is a debug message");
    info!("this is an info message");
    trace!("this is a trace message");
    warn!("this is a warn message");
    error!("this is an error message");

    read_files_with_threads(vec!["file1".to_string(), "file2".to_string()]);

    debug!(
        expensive = some_expensive_computation(),
        "doing something expensive"
    );

    read_files_with_tasks(vec!["file3".to_string(), "file4".to_string()]).await;
}

#[derive(Debug)]
struct Foo {
    #[allow(dead_code)]
    a: u32,
    #[allow(dead_code)]
    b: bool,
}

struct Bar {
    c: char,
    d: i32,
}

impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Bar({},{})", self.c, self.d))
    }
}

fn read_files_with_threads(file_names: Vec<String>) {
    let mut handlers = Vec::new();
    for file in file_names {
        let handler = thread::spawn(move || handle_file(file));
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}

async fn read_files_with_tasks(file_names: Vec<String>) {
    let mut handlers = Vec::new();
    for file in file_names {
        let handler = tokio::spawn(async move { handle_file_async(file).await });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.await.unwrap();
    }
}

#[tracing::instrument]
fn handle_file(file: String) {
    // let span = span!(Level::INFO, "file", filename = %file);
    // let _guard = span.enter();
    info!("opening the file...");
    info!("reading file content...");
    warn!("possible error while reading file content");
    trace!(parent: None, "file buffer reaing exeded");
    let bar = Bar { c: 'a', d: 50 };
    info!(bar = %bar, "parsing content...");
    let foo = Foo { a: 42, b: true };
    info!(parsed = ?foo, "completed...");
}

#[tracing::instrument]
async fn handle_file_async(file: String) {
    // let span = span!(Level::INFO, "file", filename = %file);
    // let _guard = span.enter();
    info!("opening the file...");
    info!("reading file content...");
    warn!("possible error while reading file content");
    trace!(parent: None, "file buffer reaing exeded");
    let bar = Bar { c: 'a', d: 50 };
    info!(bar = %bar, "parsing content...");
    let foo = Foo { a: 42, b: true };
    info!(parsed = ?foo, "completed...");
}

fn some_expensive_computation() -> i32 {
    println!("doing something really intensive with the CPU");
    55
}

#[test]
fn test_tracing() {
    assert!(false == false);
}
