use bench_example::play_game;

fn main() {
    for i in 1..=100 {
        play_game(i, true);
    }
}
