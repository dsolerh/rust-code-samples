use std::collections::HashMap;

use crate::{
    agents::agent::TestingAgent,
    core::{
        board::BackgammonBoard,
        dice::DiceRoll,
        players::{get_opponent, get_random_player, PlayerColor, BLACK_PLAYER, RED_PLAYER},
        positions::{bar_pos, norm_pos, BLACK_OFF, RED_OFF},
    },
};

use super::board_printer::BoardPrinter;

pub(crate) fn get_winner(board: &BackgammonBoard) -> Option<PlayerColor> {
    if board.checkers_at(BLACK_PLAYER, BLACK_OFF) == 15 {
        return Some(BLACK_PLAYER);
    }
    if board.checkers_at(RED_PLAYER, RED_OFF) == 15 {
        return Some(RED_PLAYER);
    }
    return None;
}

pub enum VictoryType {
    Game,
    Gammon,
    Backgammon,
}

impl VictoryType {
    pub fn value(&self) -> i32 {
        match self {
            VictoryType::Game => 1,
            VictoryType::Gammon => 2,
            VictoryType::Backgammon => 3,
        }
    }
}

pub(crate) fn get_victory_type(board: &BackgammonBoard, player_color: PlayerColor) -> VictoryType {
    let opponent = get_opponent(player_color);
    if board.checkers_at(opponent, norm_pos(BLACK_OFF, opponent)) == 0 {
        if board.checkers_at(opponent, bar_pos(opponent)) > 0
            || board.checkers_at(opponent, norm_pos(1, player_color)) > 0
            || board.checkers_at(opponent, norm_pos(2, player_color)) > 0
            || board.checkers_at(opponent, norm_pos(3, player_color)) > 0
            || board.checkers_at(opponent, norm_pos(4, player_color)) > 0
            || board.checkers_at(opponent, norm_pos(5, player_color)) > 0
            || board.checkers_at(opponent, norm_pos(6, player_color)) > 0
        {
            VictoryType::Backgammon
        } else {
            VictoryType::Gammon
        }
    } else {
        VictoryType::Game
    }
}

pub struct GameResult {
    pub winner: PlayerColor,
    pub turn: usize,
    pub victory_type: VictoryType,
}

pub fn play_game(agents: HashMap<PlayerColor, &Box<dyn TestingAgent>>, print: bool) -> GameResult {
    let mut board = BackgammonBoard::default_board();
    let mut player_color = get_random_player();
    let mut roll = DiceRoll::get_dice_roll();

    let printer = BoardPrinter;
    if print {
        printer.print_board(&board, player_color, &roll, None)
    }

    let mut turn = 0;
    loop {
        let agent = agents.get(&player_color).unwrap();

        let move_sequence = agent.get_best_action(&board, player_color, &roll);
        if let Some(move_sequence) = move_sequence.clone() {
            board = board.apply_move_sequence(&move_sequence, player_color);

            if let Some(winner) = get_winner(&board) {
                return GameResult {
                    winner,
                    turn,
                    victory_type: get_victory_type(&board, player_color),
                };
            }
        }

        player_color = get_opponent(player_color);
        roll = DiceRoll::get_dice_roll();

        if print {
            printer.print_board(&board, player_color, &roll, move_sequence);
        }
        turn += 1;
    }
}

#[derive(Default)]
pub struct EvaluateMetrics {
    wins: usize,
    overall_score: i32,
    score_on_wins: i32,
    score_on_lose: i32,
    hits_to_opponent: i32,
    hits_from_opponent: i32,
}

impl EvaluateMetrics {}

pub fn run_n_games(
    agents: HashMap<PlayerColor, &Box<dyn TestingAgent>>,
    print: bool,
    parallel: bool,
) -> EvaluateMetrics {
    let mut metrics = EvaluateMetrics::default();

    metrics
}
