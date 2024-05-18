use super::players::{PlayerColor, BLACK_PLAYER, RED_PLAYER};

const BOARD_START: Position = 24;
const BOARD_END: Position = 1;

pub(crate) const BLACK_BAR: Position = 26;
pub(crate) const RED_BAR: Position = 27;

pub(crate) const BLACK_OFF: Position = 0;
pub(crate) const RED_OFF: Position = 25;

pub(crate) type Position = usize;

pub(crate) fn norm_pos(pos: Position, player_color: PlayerColor) -> Position {
    assert!(pos <= 25, "the maximun normalizable position is 25");
    if player_color == BLACK_PLAYER {
        pos
    } else {
        25 - pos
    }
}

pub(crate) fn bar_pos(player_color: PlayerColor) -> Position {
    if player_color == BLACK_PLAYER {
        BLACK_BAR
    } else {
        RED_BAR
    }
}

pub(crate) struct PosIter {
    start: Position,
    end: Position,
    step: i8,
}

impl PosIter {
    pub(crate) fn full_board(player_color: PlayerColor, start: Option<Position>) -> Self {
        let start = start.unwrap_or(norm_pos(BOARD_START, player_color));
        let end = norm_pos(BOARD_END, player_color);

        Self {
            start,
            end,
            step: player_color,
        }
    }

    pub(crate) fn home_board(player_color: PlayerColor) -> Self {
        if player_color == BLACK_PLAYER {
            Self {
                start: 6,
                end: 1,
                step: BLACK_PLAYER,
            }
        } else {
            Self {
                start: 19,
                end: 24,
                step: RED_PLAYER,
            }
        }
    }
}

impl Iterator for PosIter {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        let start = self.start as i8;
        let end = self.end as i8;
        let next_val = start + self.step;
        if start != end + self.step {
            self.start = next_val as usize;
            Some(start as Position)
        } else {
            None
        }
    }
}

pub(crate) fn lte_bs(player_color: PlayerColor, pos: Position) -> bool {
    if player_color == BLACK_PLAYER {
        pos <= BOARD_START
    } else {
        pos >= BOARD_END
    }
}

pub(crate) fn lt_bs(player_color: PlayerColor, pos: Position) -> bool {
    if player_color == BLACK_PLAYER {
        pos < BOARD_START
    } else {
        pos > BOARD_END
    }
}

pub(crate) fn gte_be(player_color: PlayerColor, pos: Position) -> bool {
    if player_color == BLACK_PLAYER {
        pos >= BOARD_END
    } else {
        pos <= BOARD_START
    }
}

pub(crate) fn gt_be(player_color: PlayerColor, pos: Position) -> bool {
    if player_color == BLACK_PLAYER {
        pos > BOARD_END
    } else {
        pos < BOARD_START
    }
}

pub(crate) fn in_hb(player_color: PlayerColor, pos: Position) -> bool {
    if player_color == BLACK_PLAYER {
        pos >= 1 && pos <= 6
    } else {
        pos <= 24 && pos >= 19
    }
}

pub(crate) fn eq_bo(player_color: PlayerColor, pos: Position) -> bool {
    pos == norm_pos(BLACK_OFF, player_color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_iter_board() {
        let positions = PosIter::full_board(BLACK_PLAYER, None).collect::<Vec<_>>();
        assert_eq!(positions, (1..=24).rev().collect::<Vec<_>>());

        let positions = PosIter::full_board(BLACK_PLAYER, Some(6)).collect::<Vec<_>>();
        assert_eq!(positions, (1..=6).rev().collect::<Vec<_>>());

        let positions = PosIter::full_board(RED_PLAYER, None).collect::<Vec<_>>();
        assert_eq!(positions, (1..=24).collect::<Vec<_>>());

        let positions = PosIter::full_board(RED_PLAYER, Some(20)).collect::<Vec<_>>();
        assert_eq!(positions, (20..=24).collect::<Vec<_>>());
    }

    #[test]
    fn test_pos_iter_home_board() {
        let positions = PosIter::home_board(BLACK_PLAYER).collect::<Vec<_>>();
        assert_eq!(positions, (1..=6).rev().collect::<Vec<_>>());

        let positions = PosIter::home_board(RED_PLAYER).collect::<Vec<_>>();
        assert_eq!(positions, (19..=24).collect::<Vec<_>>());
    }
}

// def pos_iter(player, start=None):
//     start = start if start is not None else normalize_pos(24, player)
//     return range(start, normalize_pos(0, player), player)
