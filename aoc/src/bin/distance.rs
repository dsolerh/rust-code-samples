fn main() {
    let time: i64 = 47986698;
    let max_distance: i64 = 400121310111540;

    let amount = (1..time)
        .map(|x| x * (time - x))
        .filter(|&x| x > max_distance)
        .count();

    println!("amount: {amount}")
}
