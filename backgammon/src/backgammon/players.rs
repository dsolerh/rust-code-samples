use crate::backgammon::board::Point;
use rand::seq::SliceRandom;

pub(crate) type PlayerColor = i8;

pub(crate) const BLACK_PLAYER: PlayerColor = -1;
pub(crate) const RED_PLAYER: PlayerColor = 1;

pub(crate) fn get_random_player() -> PlayerColor {
    [BLACK_PLAYER, RED_PLAYER]
        .choose(&mut rand::thread_rng())
        .cloned()
        .unwrap()
}

pub(crate) fn get_opponent(player_color: PlayerColor) -> PlayerColor {
    if player_color == BLACK_PLAYER {
        RED_PLAYER
    } else {
        BLACK_PLAYER
    }
}

pub(crate) fn r(n: u8) -> Point {
    RED_PLAYER * n as i8
}

pub(crate) fn b(n: u8) -> Point {
    BLACK_PLAYER * n as i8
}
