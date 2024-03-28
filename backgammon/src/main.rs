mod backgammon;
mod model;

use burn::backend::{wgpu::WgpuDevice, Autodiff, Wgpu};
use model::{
    model::ModelConfig,
    trainer::{test, train},
};

fn main() {
    // train::<Autodiff<Wgpu>>(WgpuDevice::default());

    let device = WgpuDevice::default();
    println!("starting...");
    test(ModelConfig::new(198, 80).init::<Autodiff<Wgpu>>(&device), 1, 1, false);
    println!("end...");

    // let black_agent: Box<dyn Agent> = Box::new(LogicAgent::new_hard());
    // let red_agent: Box<dyn Agent> = Box::new(ModelAgent::<Autodiff<Wgpu>>::new(&device, 198, 80));

    // let total = 4;
    // let mut red_wins = 0;
    // let mut red_score = 0;
    // let mut turns = 0;

    // let start = Instant::now();
    // for _ in 0..total {
    //     let game_result = play_game((&black_agent, &red_agent), false);
    //     if game_result.winner == RED_PLAYER {
    //         red_wins += 1;
    //         red_score += game_result.victory_type.value()
    //     } else {
    //         red_score -= game_result.victory_type.value();
    //     }
    //     turns += game_result.turn;
    // }
    // let duration = start.elapsed();

    // println!("Time elapsed: {:?}", duration);
    // println!("Red avg score: {}", red_score as f32 / total as f32);
    // println!("Red win ratio: {}", red_wins as f32 / total as f32 * 100.0);
    // println!("Avg turns: {}", turns as f32 / total as f32)
}
