use backgammon::{
    agents::{agent::TestingAgent, random::RandomAgent},
    game::game::play_game,
};
use std::{collections::HashMap, time::Instant};
fn main() {
    // train::<Autodiff<Wgpu>>(WgpuDevice::default());

    // let device = WgpuDevice::default();
    // println!("starting...");
    // test(
    //     ModelConfig::new(198, 80).init::<Autodiff<Wgpu>>(&device),
    //     1,
    //     1,
    //     false,
    // );
    // println!("end...");
    //
    // // let black_agent: Box<dyn Agent> = Box::new(LogicAgent::new_hard());
    // let red_agent: Box<dyn Agent> = Box::new(ModelAgent::<Autodiff<Wgpu>>::new(&device, 198, 80));

    let total = 4000;
    let mut red_wins = 0;
    let mut red_score = 0;
    let mut turns = 0;

    let black_agent: Box<dyn TestingAgent> = Box::new(RandomAgent::new());
    let red_agent: Box<dyn TestingAgent> = Box::new(RandomAgent::new());

    let mut agents = HashMap::new();
    agents.insert(1, &red_agent);
    agents.insert(-1, &black_agent);

    let start = Instant::now();
    for _ in 0..total {
        let game_result = play_game(&agents, false);
        if game_result.winner == 1 {
            red_wins += 1;
            red_score += game_result.victory_type.value()
        } else {
            red_score -= game_result.victory_type.value();
        }
        turns += game_result.turn;
    }
    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);
    println!("Red avg score: {}", red_score as f32 / total as f32);
    println!("Red win ratio: {}", red_wins as f32 / total as f32 * 100.0);
    println!("Avg turns: {}", turns as f32 / total as f32)
}
