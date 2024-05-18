use core::num;
use std::collections::HashMap;

use burn::{
    config::Config,
    module::AutodiffModule,
    optim::{AdamConfig, GradientsParams, Optimizer},
    tensor::{
        backend::{AutodiffBackend, Backend},
        Tensor,
    },
};

use crate::backgammon::{
    agent::{Agent, LogicAgent, ModelAgent},
    board::BackgammonBoard,
    dice::DiceRoll,
    game::{play_game, won},
    players::{get_opponent, get_random_player, BLACK_PLAYER, RED_PLAYER},
    positions::{BLACK_OFF, RED_OFF},
};

use super::model::{Model, ModelConfig};

fn check_last_pred<B: Backend>(
    board: &BackgammonBoard,
    device: &B::Device,
) -> Option<Tensor<B, 1>> {
    if board.checkers_at(BLACK_PLAYER, BLACK_OFF) == 15 {
        Some(Tensor::from_floats([0.0], device))
    } else if board.checkers_at(RED_PLAYER, RED_OFF) == 15 {
        Some(Tensor::from_floats([1.0], device))
    } else {
        None
    }
}

// def check_terminal(board):
//     if board[BLACK_GOAL] == -15:
//         return 0, True

//     elif board[RED_GOAL] == 15:
//         return 1, True

//     return None, False

#[derive(Config)]
pub(crate) struct ModelTrainingConfig {
    #[config(default = 1_000)]
    pub num_epochs: usize,
    #[config(default = 1_000)]
    pub num_tests: usize,
    #[config(default = 100)]
    pub test_interval: usize,
    #[config(default = 0.1)]
    pub alpha: f64,
    #[config(default = 0.7)]
    pub lambda: f64,
    // pub eligibility_traces: []
    pub model_config: ModelConfig,
    pub optimizer: AdamConfig,
}

pub fn train<B: AutodiffBackend>(device: B::Device) {
    // Create the configuration.
    let config_model = ModelConfig::new(198, 80);
    let config_optimizer = AdamConfig::new();
    let config = ModelTrainingConfig::new(config_model, config_optimizer);

    // Create the model and optimizer.
    let mut model: Model<B> = config.model_config.init(&device);
    let mut optim = config.optimizer.init::<B, Model<B>>();

    // Iterate over our training and validation loop for X epochs.
    for epoch in 1..config.num_epochs + 1 {
        // Implement our training loop.
        let mut board = BackgammonBoard::default_board();
        let mut player_color = get_random_player();
        let mut roll = DiceRoll::get_dice_roll();

        let prev_pred = model.forward(model.get_board_features(&board, player_color));
        let mut turn = 0;
        while !won(&board) {
            let move_sequence = model.get_action(&board, player_color, &roll);
            if let Some(move_sequence) = move_sequence.clone() {
                board = board.apply_move_sequence(&move_sequence, player_color);

                let pred = check_last_pred(&board, &device)
                    .unwrap_or(model.forward(model.get_board_features(&board, player_color)));

                // println!("pred: {}", pred);

                let loss = pred.clone() - prev_pred.clone();

                // Gradients for the current backward pass
                let grads = loss.backward();
                // Gradients linked to each parameter of the model.
                let grads = GradientsParams::from_grads(grads, &model);
                // Update the model using the optimizer.
                model = optim.step(config.alpha, model, grads);
            }

            player_color = get_opponent(player_color);
            roll = DiceRoll::get_dice_roll();
            turn += 1;
        }

        println!("Train - Epoch {} - Turns {}", epoch, turn);

        if epoch != 0 && epoch % config.test_interval == 0 {
            test(model.valid(), epoch, config.num_tests, false)
        }
    }
}

pub fn test<B: Backend>(model: Model<B>, epoch: usize, num_tests: usize, parallel: bool) {
    let logic_agent: Box<dyn Agent> = Box::new(LogicAgent::new_hard());
    let model_agent: Box<dyn Agent> = Box::new(ModelAgent::with_model(model));

    let mut avg_score = 0.0;
    let mut win_ratio = 0.0;
    let mut avg_turn = 0.0;

    if parallel {
        let (wins, score, turns) = (0..num_tests)
            .map(|_| {
                let mut agents = HashMap::new();
                let model_color = get_random_player();
                agents.insert(model_color, &model_agent);
                agents.insert(get_opponent(model_color), &logic_agent);
                (play_game(agents, false), model_color)
            })
            .fold(
                (0, 0, 0),
                |(wins, score, turns), (game_result, model_color)| {
                    let mut wins = wins;
                    let mut score = score;
                    let mut turns = turns;

                    if game_result.winner == model_color {
                        wins += 1;
                        score += game_result.victory_type.value()
                    } else {
                        score -= game_result.victory_type.value();
                    }
                    turns += game_result.turn;
                    (wins, score, turns)
                },
            );

        avg_score = score as f32 / num_tests as f32;
        win_ratio = wins as f32 / num_tests as f32 * 100.0;
        avg_turn = turns as f32 / num_tests as f32;
    } else {
        let (wins, score, turns) = (0..num_tests)
            .map(|_| {
                let mut agents = HashMap::new();
                let model_color = get_random_player();
                agents.insert(model_color, &model_agent);
                agents.insert(get_opponent(model_color), &logic_agent);
                (play_game(agents, false), model_color)
            })
            .fold(
                (0, 0, 0),
                |(wins, score, turns), (game_result, model_color)| {
                    let mut wins = wins;
                    let mut score = score;
                    let mut turns = turns;

                    if game_result.winner == model_color {
                        wins += 1;
                        score += game_result.victory_type.value()
                    } else {
                        score -= game_result.victory_type.value();
                    }
                    turns += game_result.turn;
                    (wins, score, turns)
                },
            );

        avg_score = score as f32 / num_tests as f32;
        win_ratio = wins as f32 / num_tests as f32 * 100.0;
        avg_turn = turns as f32 / num_tests as f32;
    }
    println!(
        "Testing after {} epochs | avg score {:.3} | win ratio {:.3} | Avg turn {:.3}",
        epoch, avg_score, win_ratio, avg_turn
    )
}
