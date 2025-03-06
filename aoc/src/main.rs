use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("https://adventofcode.com/{}/day/{}/input", 2024, 1))
        .send()?;

    let status = res.status();
    let response = res.text()?;
    println!("status: {status}");
    println!("response: {response}");

    Ok(())
}
//
