use rand::seq::SliceRandom;

use crate::core::{
    board::BackgammonBoard,
    dice::DiceRoll,
    moves::{generate_all_move_sequences, MoveSequence},
    players::PlayerColor,
};

use super::agent::TestingAgent;

pub struct RandomAgent;

impl RandomAgent {
    pub fn new() -> Self {
        RandomAgent
    }
}

impl TestingAgent for RandomAgent {
    fn get_best_action(
        &self,
        board: &BackgammonBoard,
        player_color: PlayerColor,
        roll: &DiceRoll,
    ) -> Option<MoveSequence> {
        let move_sequences = generate_all_move_sequences(board, player_color, roll, None);

        if move_sequences.len() == 0 {
            None
        } else {
            move_sequences.choose(&mut rand::thread_rng()).cloned()
        }
    }
}
