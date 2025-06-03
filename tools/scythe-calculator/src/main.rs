fn main() {
    println!("Hello, world!");
}

enum Faction {
    Rusviet,
    Crimea,
    Nordic,
    Polania,
    Saxony,
}

enum Popularity {
    Tier1,
    Tier2,
    Tier3,
}

struct FactionData {
    faction: Faction,
    coins: i32,
    stars: i32,
    popularity: Popularity,
    territories: i32,
    resources: i32,
    bonus_structures: i32,
    structures: i32,
    power: i32,
}
