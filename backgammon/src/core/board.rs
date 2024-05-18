use super::moves::{MoveSequence, MoveWithDie};
use super::players::{b, get_opponent, r, PlayerColor, BLACK_PLAYER, RED_PLAYER};
use super::positions::{bar_pos, norm_pos, Position};

use lazy_static::lazy_static;

pub(crate) type Point = i8;

/**
 * Board representation
 *
 * |-----------------------------||||||--------- Red Home Board ---------|
 * | 13 | 14 | 15 | 16 | 17 | 18 |||||| 19 | 20 | 21 | 22 | 23 | 24 | 25 |  <- Black (x) starting point
 * |  x | -- | -- | -- |  o | -- ||||||  o | -- | -- | -- | -- |  x | -- |
 * |  x | -- | -- | -- |  o | -- |[27]|  o | -- | -- | -- | -- |  x | -- |
 * |  x | -- | -- | -- |  o | -- ||||||  o | -- | -- | -- | -- | -- | -- |
 * |  x | -- | -- | -- | -- | -- ||||||  o | -- | -- | -- | -- | -- | -- |
 * |  x | -- | -- | -- | -- | -- ||||||  o | -- | -- | -- | -- | -- | -- |
 * | -- | -- | -- | -- | -- | -- |||||| -- | -- | -- | -- | -- | -- | -- |
 * | -- | -- | -- | -- | -- | -- |||||| -- | -- | -- | -- | -- | -- | -- |
 * | -- | -- | -- | -- | -- | -- |||||| -- | -- | -- | -- | -- | -- | -- |
 * |  o | -- | -- | -- | -- | -- ||||||  x | -- | -- | -- | -- | -- | -- |
 * |  o | -- | -- | -- | -- | -- ||||||  x | -- | -- | -- | -- | -- | -- |
 * |  o | -- | -- | -- |  x | -- ||||||  x | -- | -- | -- | -- | -- | -- |
 * |  o | -- | -- | -- |  x | -- |[26]|  x | -- | -- | -- | -- |  o | -- |
 * |  o | -- | -- | -- |  x | -- ||||||  x | -- | -- | -- | -- |  o | -- |
 * | 12 | 11 | 10 |  9 |  8 |  7 ||||||  6 |  5 |  4 |  3 |  2 |  1 |  0 |  <- Red (o) starting point
 * |-----------------------------||||||-------- Black Home Board --------|
 */
type Board = [Point; 28];

#[derive(Debug, Clone)]
pub struct BackgammonBoard {
    board: Board,
}

lazy_static! {
    static ref DEFAULT_BOARD: Board = {
        let mut m = [0 as Point; 28];
        m[1] = r(2);
        m[6] = b(5);
        m[8] = b(3);
        m[12] = r(5);
        m[13] = b(5);
        m[17] = r(3);
        m[19] = r(5);
        m[24] = b(2);
        m
    };
}

impl BackgammonBoard {
    pub(crate) fn default_board() -> Self {
        Self {
            board: *DEFAULT_BOARD,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_pairs(pairs: &[(Position, Point)]) -> Self {
        let mut board: Board = [0; 28];

        for &(pos, point) in pairs {
            board[pos] = point
        }

        Self { board }
    }

    pub(crate) fn points(&self) -> (Point, Point) {
        let (mut black_points, mut red_points) = (0, 0);

        for point in self.board {
            if point < 0 {
                black_points -= point
            }
            if point > 0 {
                red_points += point
            }
        }

        (black_points, red_points)
    }

    pub(crate) fn points_at(&self, pos: Position) -> usize {
        self.board[pos].abs() as usize
    }

    pub(crate) fn checkers_at(&self, player_color: PlayerColor, pos: Position) -> usize {
        match player_color {
            BLACK_PLAYER => {
                if self.board[pos] < 0 {
                    -self.board[pos] as usize
                } else {
                    0
                }
            }
            RED_PLAYER => {
                if self.board[pos] > 0 {
                    self.board[pos] as usize
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub(crate) fn can_bear_off(&self, player_color: PlayerColor) -> bool {
        let chk = self.checkers_at(player_color, norm_pos(0, player_color))
            + self.checkers_at(player_color, norm_pos(1, player_color))
            + self.checkers_at(player_color, norm_pos(2, player_color))
            + self.checkers_at(player_color, norm_pos(3, player_color))
            + self.checkers_at(player_color, norm_pos(4, player_color))
            + self.checkers_at(player_color, norm_pos(5, player_color))
            + self.checkers_at(player_color, norm_pos(6, player_color));

        chk == 15
    }

    pub(crate) fn apply_n_moves(
        &self,
        m: &MoveWithDie,
        player_color: PlayerColor,
        n: usize,
    ) -> Self {
        let points = player_color * n as i8;

        let mut new_board = self.board.clone();
        new_board[m.src] -= points;

        let opponent = get_opponent(player_color);
        let opp_chk = self.checkers_at(opponent, m.dst);

        if opp_chk != 0 {
            new_board[m.dst] = points;
        } else {
            new_board[m.dst] += points;
        }

        new_board[bar_pos(opponent)] += opp_chk as i8 * opponent;

        Self { board: new_board }
    }

    pub(crate) fn apply_move_sequence(
        &self,
        move_sequence: &MoveSequence,
        player_color: PlayerColor,
    ) -> Self {
        let mut new_board = self.clone();

        for m in move_sequence.to_list() {
            new_board = new_board.apply_n_moves(m, player_color, 1)
        }

        new_board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backgammon_board_points() {
        let board = BackgammonBoard::default_board();
        assert_eq!(board.points(), (-15, 15));
    }

    #[test]
    fn test_backgammon_board_checkers_at() {
        let board = BackgammonBoard::default_board();
        assert_eq!(board.checkers_at(RED_PLAYER, 1), 2);
        assert_eq!(board.checkers_at(BLACK_PLAYER, 1), 0);
        assert_eq!(board.checkers_at(RED_PLAYER, 6), 0);
        assert_eq!(board.checkers_at(BLACK_PLAYER, 6), 5);
    }

    #[test]
    fn test_backgammon_board_can_bear_off() {
        let board = BackgammonBoard::default_board();
        assert!(!board.can_bear_off(BLACK_PLAYER));
        assert!(!board.can_bear_off(RED_PLAYER));

        let board = BackgammonBoard::from_pairs(&[(6, b(3)), (4, b(6)), (3, b(4)), (0, b(2))]);
        assert!(board.can_bear_off(BLACK_PLAYER));

        let board = BackgammonBoard::from_pairs(&[(24, r(1)), (23, r(6)), (20, r(2)), (25, r(6))]);
        assert!(board.can_bear_off(RED_PLAYER));
    }

    #[test]
    fn test_backgammon_board_apply_move_sequence() {
        let board = BackgammonBoard::default_board();
        let resulting_board = board
            .apply_move_sequence(
                &MoveSequence::TwoMoves(
                    MoveWithDie {
                        src: 24,
                        dst: 22,
                        die: 2,
                    },
                    MoveWithDie {
                        src: 13,
                        dst: 10,
                        die: 3,
                    },
                ),
                BLACK_PLAYER,
            )
            .board;
        let mut expected_board = DEFAULT_BOARD.clone();
        expected_board[24] = b(1);
        expected_board[22] = b(1);
        expected_board[13] = b(4);
        expected_board[10] = b(1);

        assert_eq!(resulting_board, expected_board)
    }
}
