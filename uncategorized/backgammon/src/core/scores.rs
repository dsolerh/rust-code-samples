use super::{
    board::BackgammonBoard,
    players::{get_opponent, PlayerColor, BLACK_PLAYER, RED_PLAYER},
    positions::{bar_pos, norm_pos, PosIter, Position, BLACK_BAR, RED_BAR},
};

pub(crate) enum GamePhase {
    Initial,
    BearOff,
    NormalRace,
    BearOffRace,
}

pub(crate) fn get_game_phase(board: &BackgammonBoard, player_color: PlayerColor) -> GamePhase {
    match (is_race(board), board.can_bear_off(player_color)) {
        (true, true) => GamePhase::BearOffRace,
        (true, false) => GamePhase::NormalRace,
        (false, true) => GamePhase::BearOff,
        (false, false) => GamePhase::Initial,
    }
}

fn is_race(board: &BackgammonBoard) -> bool {
    if board.checkers_at(BLACK_PLAYER, BLACK_BAR) > 0 || board.checkers_at(RED_PLAYER, RED_BAR) > 0
    {
        return false;
    }

    let (black_start, _) = get_checkers_boundries(board, BLACK_PLAYER);
    let (red_start, _) = get_checkers_boundries(board, RED_PLAYER);

    black_start < red_start
}

fn get_checkers_boundries(
    board: &BackgammonBoard,
    player_color: PlayerColor,
) -> (Position, Position) {
    let mut start = None;
    let mut end = None;

    for pos in PosIter::full_board(player_color, None) {
        if board.checkers_at(player_color, pos) > 0 {
            if start.is_none() {
                start = Some(pos)
            }

            end = Some(pos)
        }
    }

    (start.unwrap_or_default(), end.unwrap_or_default())
}

pub(crate) fn get_score<F>(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    max_val: f32,
    weight: f32,
    max_weight: f32,
    metric_fn: F,
) -> f32
where
    F: Fn(&BackgammonBoard, PlayerColor) -> f32,
{
    if weight == 0.0 || max_weight == 0.0 {
        0.0
    } else {
        metric_fn(board, player_color) / max_val * (weight / max_weight)
    }
}

pub(crate) fn get_pip_count_diff(board: &BackgammonBoard, player_color: PlayerColor) -> f32 {
    get_pip_count(board, get_opponent(player_color)) - get_pip_count(board, player_color)
}

fn get_pip_count(board: &BackgammonBoard, player_color: PlayerColor) -> f32 {
    25.0 * board.checkers_at(player_color, bar_pos(player_color)) as f32
        + PosIter::full_board(player_color, None)
            .map(|pos| {
                board.checkers_at(player_color, pos) as f32 * norm_pos(pos, player_color) as f32
            })
            .sum::<f32>()
}

pub(crate) fn get_prime_length(board: &BackgammonBoard, player_color: PlayerColor) -> f32 {
    let mut current_longest_prime = 0;
    let mut longest_prime = 0;
    let mut prev_prime_pos = -1;

    for pos in PosIter::full_board(player_color, None) {
        if board.checkers_at(player_color, pos) > 0 {
            if current_longest_prime == 0 {
                // if this is the firts prime then give the lenght 1
                current_longest_prime = 1;
            } else if prev_prime_pos != -1 && (pos as i32 - prev_prime_pos).abs() == 1 {
                // if the previous prime pos has been set and
                // the absolute difference beteen the current prime position and the prevoius is 1
                current_longest_prime += 1;
            } else if longest_prime == 0 {
                // if there's no recorded longest prime
                longest_prime = current_longest_prime;
                current_longest_prime = 1;
            } else if current_longest_prime > longest_prime {
                longest_prime = current_longest_prime;
                current_longest_prime = 1;
            } else {
                current_longest_prime = 1;
            }

            prev_prime_pos = pos as i32;
        }
    }

    if current_longest_prime > longest_prime {
        current_longest_prime as f32
    } else {
        longest_prime as f32
    }
}

pub(crate) fn get_anchors_count(board: &BackgammonBoard, player_color: PlayerColor) -> f32 {
    PosIter::home_board(get_opponent(player_color))
        .filter(|&pos| board.checkers_at(player_color, pos) > 1)
        .count() as f32
}

pub(crate) fn get_prime_in_home_board_count(
    board: &BackgammonBoard,
    player_color: PlayerColor,
) -> f32 {
    PosIter::home_board(player_color)
        .filter(|&pos| board.checkers_at(player_color, pos) > 1)
        .count() as f32
}

pub(crate) fn get_checkers_in_opponent_hb_count(
    board: &BackgammonBoard,
    player_color: PlayerColor,
) -> f32 {
    PosIter::home_board(get_opponent(player_color))
        .map(|pos| board.checkers_at(player_color, pos))
        .sum::<usize>() as f32
}

pub(crate) fn get_blot_count(board: &BackgammonBoard, player_color: PlayerColor) -> f32 {
    PosIter::full_board(player_color, None)
        .filter(|&pos| board.checkers_at(player_color, pos) == 1)
        .count() as f32
}

pub(crate) fn get_blots_in_home_board_count(
    board: &BackgammonBoard,
    player_color: PlayerColor,
) -> f32 {
    PosIter::home_board(player_color)
        .filter(|&pos| board.checkers_at(player_color, pos) == 1)
        .count() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pip_count() {
        let board = BackgammonBoard::default_board();
        assert_eq!(get_pip_count(&board, BLACK_PLAYER), 167.0);
        assert_eq!(get_pip_count(&board, RED_PLAYER), 167.0);
    }

    #[test]
    fn test_pip_count_diff() {
        let board = BackgammonBoard::default_board();
        assert_eq!(get_pip_count_diff(&board, BLACK_PLAYER), 0.0);
        assert_eq!(get_pip_count_diff(&board, RED_PLAYER), 0.0);
    }
}