use crate::core::{
    board::BackgammonBoard, dice::DiceRoll, moves::MoveSequence, players::PlayerColor,
};

pub trait TestingAgent {
    fn get_best_action(
        &self,
        board: &BackgammonBoard,
        player_color: PlayerColor,
        roll: &DiceRoll,
    ) -> Option<MoveSequence>;
}
