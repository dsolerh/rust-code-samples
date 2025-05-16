use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
    time::Duration,
};

use anyhow::Context;
use reqwest::header::{ACCEPT, COOKIE};

const AOC_URL: &str = "https://adventofcode.com";
const COPY_TO_MANY_BUFFER_SIZE: usize = 8192;

pub fn get_data(session: &str, year: usize, day: usize) -> anyhow::Result<Vec<String>> {
    let fname = format!("data/{year}/day_{day}.data");
    let filename = Path::new(&fname);
    if filename.exists() {
        let file = File::open(filename).context("could not open file for reading data")?;
        let data = BufReader::new(file)
            .lines()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(data)
    } else {
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(format!("{AOC_URL}/{year}/day/{day}/input"))
            .timeout(Duration::from_secs(10))
            .header(ACCEPT, "text/html")
            .header(COOKIE, format!("session={session}"))
            .send()
            .context("could not send the request to get the data from the api")?
            .error_for_status()
            .context("api reponded with an invalid status")?;

        let mut file = File::create(filename)
            .context("could not create the file to cache the data")
            .context(fname)?;
        let mut buffvec = vec![];
        copy_to_multiple_writers(res, &mut file, &mut buffvec)
            .context("could not cache the api data into the file")?;

        let data = BufReader::new(buffvec.as_slice())
            .lines()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(data)
    }
}

fn copy_to_multiple_writers<R, W1, W2>(
    mut reader: R,
    writer1: &mut W1,
    writer2: &mut W2,
) -> std::io::Result<()>
where
    R: Read,
    W1: Write,
    W2: Write,
{
    let mut buffer = [0u8; COPY_TO_MANY_BUFFER_SIZE];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        writer1.write_all(&buffer[..n])?;
        writer2.write_all(&buffer[..n])?;
    }
    Ok(())
}
