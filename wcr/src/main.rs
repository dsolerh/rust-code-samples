use anyhow::Result;
use wcr;

fn main() -> Result<()> {
    let args = wcr::Args::new();
    wcr::run(args)
}
