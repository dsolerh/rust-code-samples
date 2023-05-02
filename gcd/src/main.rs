use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.is_empty() {
        eprintln!("Usage: gdc NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gdc(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gdc(mut n: u64, mut m: u64) -> u64 {
    assert!((n != 0 && m != 0));
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[test]
fn test_gdc() {
    assert_eq!(gdc(14, 15), 1);

    assert_eq!(gdc(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
