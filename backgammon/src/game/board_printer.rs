use std::io::{self, Read};

use crate::core::{
    board::BackgammonBoard,
    dice::DiceRoll,
    moves::MoveSequence,
    players::{PlayerColor, BLACK_PLAYER, RED_PLAYER},
    positions::{bar_pos, Position},
};

const UPER_POS: [Position; 13] = [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
const LOWER_POS: [Position; 13] = [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

pub struct BoardPrinter;

impl BoardPrinter {
    pub(crate) fn print_board(
        &self,
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

// pauses the execution of the program until the user press a key on the terminal
fn pause() {
    let mut stdin = io::stdin();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
