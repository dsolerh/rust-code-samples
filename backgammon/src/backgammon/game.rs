use std::{collections::HashMap, io::{self, Read}};

use super::{
    agent::Agent,
    board::BackgammonBoard,
    dice::DiceRoll,
    moves::MoveSequence,
    players::{get_opponent, get_random_player, PlayerColor, BLACK_PLAYER, RED_PLAYER},
    positions::{bar_pos, norm_pos, Position, BLACK_OFF, RED_OFF},
};

pub(crate) fn won(board: &BackgammonBoard) -> bool {
    board.checkers_at(BLACK_PLAYER, BLACK_OFF) == 15 || board.checkers_at(RED_PLAYER, RED_OFF) == 15
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
// func GetVictoryType(board *Board, color Color) VictoryType {
// 	opponent := color.Opponent()
// 	if board.Checkers(opponent, BOARD_OFF.Norm(opponent)) == 0 {
// 		// if there aren't checkers on the opponent's bear off position then it's
// 		// either a Gammon or a Backgammon
// 		if board.Checkers(opponent, BarPos(opponent)) > 0 ||
// 			board.Checkers(opponent, Position(1).Norm(color)) > 0 ||
// 			board.Checkers(opponent, Position(2).Norm(color)) > 0 ||
// 			board.Checkers(opponent, Position(3).Norm(color)) > 0 ||
// 			board.Checkers(opponent, Position(4).Norm(color)) > 0 ||
// 			board.Checkers(opponent, Position(5).Norm(color)) > 0 ||
// 			board.Checkers(opponent, Position(6).Norm(color)) > 0 {
// 			// If there are checkers of the opponent in the opponent bar or the color homeboard
// 			// the result is a Backgammon
// 			return V_BACKGAMMON
// 		}
// 		return V_GAMMON
// 	}
// 	return V_GAME
// }

pub(crate) enum VictoryType {
    Game,
    Gammon,
    Backgammon,
}

impl VictoryType {
    pub(crate) fn value(&self) -> i32 {
        match self {
            VictoryType::Game => 1,
            VictoryType::Gammon => 2,
            VictoryType::Backgammon => 3,
        }
    }
}

pub(crate) struct GameResult {
    pub(crate) winner: PlayerColor,
    pub(crate) turn: usize,
    pub(crate) victory_type: VictoryType,
}

pub(crate) fn play_game(agents: HashMap<PlayerColor, &Box<dyn Agent>>, print: bool) -> GameResult {
    let mut board = BackgammonBoard::default_board();
    let mut player_color = get_random_player();
    let mut roll = DiceRoll::get_dice_roll();

    if print {
        print_board(&board, player_color, &roll, None)
    }

    let mut turn = 0;
    while !won(&board) {
        let agent = agents.get(&player_color).unwrap();

        let move_sequence = agent.get_action(&board, player_color, &roll);
        if let Some(move_sequence) = move_sequence.clone() {
            board = board.apply_move_sequence(&move_sequence, player_color);
        }

        player_color = get_opponent(player_color);
        roll = DiceRoll::get_dice_roll();

        if print {
            print_board(&board, player_color, &roll, move_sequence);
        }
        turn += 1;
    }

    GameResult {
        winner: get_opponent(player_color),
        victory_type: get_victory_type(&board, player_color),
        turn,
    }
}

const UPER_POS: [Position; 13] = [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
const LOWER_POS: [Position; 13] = [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

fn print_board(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    roll: &DiceRoll,
    move_sequence: Option<MoveSequence>,
) {
    println!("||-----------------------------||||||--------- Red Home Board ---------||");
    println!("|| 13 | 14 | 15 | 16 | 17 | 18 |||||| 19 | 20 | 21 | 22 | 23 | 24 | 25 ||");
    print_line(board, "||||", UPER_POS, 1);
    print_line(board, "[27]", UPER_POS, 2);
    print_line(board, &print_bar(board, RED_PLAYER), UPER_POS, 3);
    print_line(board, "||||", UPER_POS, 4);
    print_line(board, "||||", UPER_POS, 5);
    println!("|| -- | -- | -- | -- | -- | -- |||||| -- | -- | -- | -- | -- | -- | -- ||");
    println!("|| -- | -- | -- | -- | -- | -- |||||| -- | -- | -- | -- | -- | -- | -- ||");
    println!("|| -- | -- | -- | -- | -- | -- |||||| -- | -- | -- | -- | -- | -- | -- ||");
    print_line(board, "||||", LOWER_POS, 5);
    print_line(board, "||||", LOWER_POS, 4);
    print_line(board, &print_bar(board, BLACK_PLAYER), LOWER_POS, 3);
    print_line(board, "[26]", LOWER_POS, 2);
    print_line(board, "||||", LOWER_POS, 1);
    println!("|| 12 | 11 | 10 |  9 |  8 |  7 ||||||  6 |  5 |  4 |  3 |  2 |  1 |  0 ||");
    println!("||-----------------------------||||||-------- Black Home Board --------||");
    println!();
    println!("Player Color: {}", player_color);
    println!("Roll: {:?}", roll);
    if let Some(ms) = move_sequence {
        println!("Moves: {:?}", ms)
    }

    let (blk_points, red_points) = board.points();
    if blk_points > 15 {
        println!("black points are wrong");
        println!();
    }
    if red_points > 15 {
        println!("red points are wrong");
        println!();
    }

    pause();
}

fn print_line(board: &BackgammonBoard, bar: &str, pos: [Position; 13], line: usize) {
    println!(
        "|| {} | {} | {} | {} | {} | {} |{}| {} | {} | {} | {} | {} | {} | {} ||",
        // first
        print_point(board, pos[0], line),
        print_point(board, pos[1], line),
        print_point(board, pos[2], line),
        print_point(board, pos[3], line),
        print_point(board, pos[4], line),
        print_point(board, pos[5], line),
        // bar
        bar,
        // second
        print_point(board, pos[6], line),
        print_point(board, pos[7], line),
        print_point(board, pos[8], line),
        print_point(board, pos[9], line),
        print_point(board, pos[10], line),
        print_point(board, pos[11], line),
        print_point(board, pos[12], line),
    );
}

fn print_point(board: &BackgammonBoard, pos: Position, line: usize) -> String {
    let chk = board.checkers_at(BLACK_PLAYER, pos);
    if chk >= line {
        if line >= 5 {
            return format!("{chk}X");
        }
        return " X".to_string();
    }

    let chk = board.checkers_at(RED_PLAYER, pos);
    if chk >= line {
        if line >= 5 {
            return format!("{chk}O");
        }
        return " O".to_string();
    }

    "  ".to_string()
}

fn print_bar(board: &BackgammonBoard, player_color: PlayerColor) -> String {
    format!(
        "[{}{}]",
        board.checkers_at(player_color, bar_pos(player_color)),
        if player_color == BLACK_PLAYER {
            "X"
        } else {
            "O"
        }
    )
}

fn pause() {
    let mut stdin = io::stdin();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
