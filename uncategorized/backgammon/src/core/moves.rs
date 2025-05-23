use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use super::{
    board::BackgammonBoard,
    dice::{repeat_die, DiceRoll, Die},
    players::{get_opponent, PlayerColor},
    positions::{
        bar_pos, eq_bo, gt_be, gte_be, in_hb, lt_bs, lte_bs, norm_pos, PosIter, Position, BLACK_OFF,
    },
};

#[derive(Debug, Clone)]
pub struct MoveWithDie {
    pub src: Position,
    pub dst: Position,
    #[allow(dead_code)]
    pub die: Die,
}

impl MoveWithDie {
    // creation
    pub(crate) fn from_bar_with_die(player_color: PlayerColor, die: Die) -> Self {
        let src = bar_pos(player_color);
        let dst = norm_pos(die as Position, get_opponent(player_color));

        Self { src, dst, die }
    }

    pub(crate) fn with_die(player_color: PlayerColor, src: Position, die: Die) -> Self {
        let dst = src as i8 + player_color * die as i8;
        let mut dst = if dst < 0 { 0 } else { dst as Position };

        if !gte_be(player_color, dst) {
            dst = norm_pos(BLACK_OFF, player_color);
        }

        Self { src, dst, die }
    }

    // validation
    pub(crate) fn is_valid_re_entry(
        &self,
        board: &BackgammonBoard,
        player_color: PlayerColor,
    ) -> bool {
        let chk_src = board.checkers_at(player_color, self.src);
        let chk_dst = board.checkers_at(get_opponent(player_color), self.dst);

        chk_src > 0 && chk_dst <= 1
    }

    pub(crate) fn is_valid_normal(
        &self,
        board: &BackgammonBoard,
        player_color: PlayerColor,
    ) -> bool {
        lte_bs(player_color, self.src)
            && gt_be(player_color, self.src)
            && gte_be(player_color, self.dst)
            && lt_bs(player_color, self.dst)
            && board.checkers_at(player_color, self.src) > 0
            && board.checkers_at(get_opponent(player_color), self.dst) <= 1
    }

    pub(crate) fn is_valid_boff(
        &self,
        board: &BackgammonBoard,
        player_color: PlayerColor,
        die: Die,
    ) -> bool {
        if !in_hb(player_color, self.src)
            || !eq_bo(player_color, self.dst)
            || board.checkers_at(player_color, self.src) == 0
            || !board.can_bear_off(player_color)
        {
            return false;
        }

        let boff_pos = norm_pos(self.src, player_color);

        if boff_pos == die as Position {
            return true;
        }

        if boff_pos > die as Position {
            return false;
        }

        for pos in boff_pos + 1..7 {
            if board.checkers_at(player_color, norm_pos(pos, player_color)) > 0 {
                return false;
            }
        }

        true
    }
}

impl Hash for MoveWithDie {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.src.hash(state);
        self.dst.hash(state);
    }
}

impl PartialEq for MoveWithDie {
    fn eq(&self, other: &Self) -> bool {
        self.src == other.src && self.dst == other.dst
    }
}
impl Eq for MoveWithDie {}

impl PartialOrd for MoveWithDie {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.src.partial_cmp(&other.src) {
            Some(core::cmp::Ordering::Equal) => self.dst.partial_cmp(&other.dst),
            ord => ord,
        }
    }
}
impl Ord for MoveWithDie {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.src.cmp(&other.src) {
            std::cmp::Ordering::Equal => self.dst.cmp(&other.dst),
            ord => ord,
        }
    }
}

pub(crate) fn get_moves(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    die: Die,
) -> Vec<MoveWithDie> {
    if board.checkers_at(player_color, bar_pos(player_color)) > 0 {
        let m = MoveWithDie::from_bar_with_die(player_color, die);
        if m.is_valid_re_entry(board, player_color) {
            vec![m]
        } else {
            Vec::new()
        }
    } else {
        PosIter::full_board(player_color, None)
            .filter(|&pos| board.checkers_at(player_color, pos) > 0)
            .map(|pos| MoveWithDie::with_die(player_color, pos, die))
            .filter(|m| {
                m.is_valid_normal(board, player_color) || m.is_valid_boff(board, player_color, die)
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone)]
pub enum MoveSequence {
    SingleMove(MoveWithDie),
    TwoMoves(MoveWithDie, MoveWithDie),
    ThreeMoves(MoveWithDie, MoveWithDie, MoveWithDie),
    FourMoves(MoveWithDie, MoveWithDie, MoveWithDie, MoveWithDie),
}

impl MoveSequence {
    pub(crate) fn to_list(&self) -> Vec<&MoveWithDie> {
        match self {
            MoveSequence::SingleMove(m1) => vec![m1],
            MoveSequence::TwoMoves(m1, m2) => vec![m1, m2],
            MoveSequence::ThreeMoves(m1, m2, m3) => vec![m1, m2, m3],
            MoveSequence::FourMoves(m1, m2, m3, m4) => vec![m1, m2, m3, m4],
        }
    }
}

impl Hash for MoveSequence {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            MoveSequence::SingleMove(move1) => {
                move1.hash(state);
            }
            MoveSequence::TwoMoves(move1, move2) => {
                if move1 > move2 {
                    move1.hash(state);
                    move2.hash(state);
                } else {
                    move2.hash(state);
                    move1.hash(state);
                }
            }
            MoveSequence::ThreeMoves(move1, move2, move3) => {
                let mut moves = [move1, move2, move3];
                moves.sort();
                for m in moves {
                    m.hash(state);
                }
            }
            MoveSequence::FourMoves(move1, move2, move3, move4) => {
                let mut moves = [move1, move2, move3, move4];
                moves.sort();
                for m in moves {
                    m.hash(state);
                }
            }
        }
    }
}

fn eq_moves(mut mvl: Vec<&MoveWithDie>, mut mvr: Vec<&MoveWithDie>) -> bool {
    mvl.sort();
    mvr.sort();

    mvl.into_iter()
        .zip(mvr.into_iter())
        .all(|(ml, mr)| ml == mr)
}

impl PartialEq for MoveSequence {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SingleMove(l0), Self::SingleMove(r0)) => l0 == r0,
            (Self::TwoMoves(l0, l1), Self::TwoMoves(r0, r1)) => {
                eq_moves(vec![l0, l1], vec![r0, r1])
            }
            (Self::ThreeMoves(l0, l1, l2), Self::ThreeMoves(r0, r1, r2)) => {
                eq_moves(vec![l0, l1, l2], vec![r0, r1, r2])
            }
            (Self::FourMoves(l0, l1, l2, l3), Self::FourMoves(r0, r1, r2, r3)) => {
                eq_moves(vec![l0, l1, l2, l3], vec![r0, r1, r2, r3])
            }
            _ => false,
        }
    }
}
impl Eq for MoveSequence {}

fn partial_cmp_moves(
    mut mvl: Vec<&MoveWithDie>,
    mut mvr: Vec<&MoveWithDie>,
) -> Option<std::cmp::Ordering> {
    mvl.sort();
    mvr.sort();

    for (ml, mr) in mvl.into_iter().zip(mvr.into_iter()) {
        match ml.partial_cmp(mr) {
            Some(std::cmp::Ordering::Equal) => {
                continue;
            }
            None => return None,
            ord => return ord,
        }
    }
    Some(std::cmp::Ordering::Equal)
}

impl PartialOrd for MoveSequence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::SingleMove(l), Self::SingleMove(r)) => l.partial_cmp(r),
            (l, r) => partial_cmp_moves(l.to_list(), r.to_list()),
        }
    }
}

fn cmp_moves(mut mvl: Vec<&MoveWithDie>, mut mvr: Vec<&MoveWithDie>) -> std::cmp::Ordering {
    mvl.sort();
    mvr.sort();

    for (ml, mr) in mvl.into_iter().zip(mvr.into_iter()) {
        match ml.cmp(mr) {
            std::cmp::Ordering::Equal => {
                continue;
            }
            ord => return ord,
        }
    }
    std::cmp::Ordering::Equal
}

impl Ord for MoveSequence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::SingleMove(l), Self::SingleMove(r)) => l.cmp(r),
            (l, r) => cmp_moves(l.to_list(), r.to_list()),
        }
    }
}

pub(crate) fn get_move_sequences(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    roll: &[Die],
) -> Vec<MoveSequence> {
    let dice_len = roll.len();
    assert!(dice_len > 0, "there must be at least 1 die");

    let mut move_sequences = Vec::new();

    for move1 in get_moves(board, player_color, roll[0]) {
        if dice_len > 1 {
            let board2 = board.apply_n_moves(&move1, player_color, 1);
            for move2 in get_moves(&board2, player_color, roll[1]) {
                if dice_len > 2 {
                    let board3 = board2.apply_n_moves(&move2, player_color, 1);
                    for move3 in get_moves(&board3, player_color, roll[2]) {
                        if dice_len > 3 {
                            let board4 = board3.apply_n_moves(&move3, player_color, 1);
                            for move4 in get_moves(&board4, player_color, roll[3]) {
                                move_sequences.push(MoveSequence::FourMoves(
                                    move1.clone(),
                                    move2.clone(),
                                    move3.clone(),
                                    move4.clone(),
                                ));
                            }
                        } else {
                            move_sequences.push(MoveSequence::ThreeMoves(
                                move1.clone(),
                                move2.clone(),
                                move3.clone(),
                            ));
                        }
                    }
                } else {
                    move_sequences.push(MoveSequence::TwoMoves(move1.clone(), move2.clone()));
                }
            }
        } else {
            move_sequences.push(MoveSequence::SingleMove(move1.clone()));
        }
    }

    move_sequences
}

pub(crate) fn generate_all_move_sequences(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    roll: &DiceRoll,
    max_seq_len: Option<usize>,
) -> Vec<MoveSequence> {
    let max_seq_len = max_seq_len.unwrap_or(max_valid_moves(board, player_color, roll, None));

    let mut move_sequences = Vec::new();

    if max_seq_len == 0 {
        return move_sequences;
    }

    match roll {
        DiceRoll::Single(_, _) => {
            let (high_die, low_die) = roll.get_high_low_die();
            if max_seq_len == 1 {
                move_sequences.append(&mut get_move_sequences(board, player_color, &[high_die]));
                if move_sequences.len() == 0 {
                    move_sequences.append(&mut get_move_sequences(board, player_color, &[low_die]));
                }
            } else {
                move_sequences.append(&mut get_move_sequences(
                    board,
                    player_color,
                    &[high_die, low_die],
                ));
                move_sequences.append(&mut get_move_sequences(
                    board,
                    player_color,
                    &[low_die, high_die],
                ));
            }
        }
        DiceRoll::Double(die) => {
            move_sequences.append(&mut get_move_sequences(
                board,
                player_color,
                &repeat_die(*die, max_seq_len),
            ));
        }
    }

    let move_sequences = move_sequences.into_iter().collect::<HashSet<_>>();

    move_sequences.into_iter().collect::<Vec<_>>()
}

fn max_valid_moves(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    roll: &DiceRoll,
    n: Option<usize>,
) -> usize {
    let n = n.unwrap_or(4);

    match roll {
        DiceRoll::Single(die1, die2) => {
            if board.checkers_at(player_color, bar_pos(player_color)) > 0 {
                max_value(
                    board,
                    player_color,
                    &[*die1, *die2],
                    max_valid_re_entry_moves_single_roll,
                )
            } else {
                max_value(
                    board,
                    player_color,
                    &[*die1, *die2],
                    max_valid_moves_single_roll,
                )
            }
        }
        DiceRoll::Double(die) => {
            if board.checkers_at(player_color, bar_pos(player_color)) > 0 {
                max_valid_re_entry_moves_double_roll(board, player_color, *die, n)
            } else {
                max_n_valid_moves_for_double_roll(board, player_color, *die, n)
            }
        }
    }
}

fn max_value<F>(board: &BackgammonBoard, player_color: PlayerColor, roll: &[Die], func: F) -> usize
where
    F: Fn(&BackgammonBoard, PlayerColor, &[Die]) -> usize,
{
    let max_val = func(board, player_color, roll);

    if max_val == 2 {
        return max_val;
    }

    std::cmp::max(max_val, func(board, player_color, &[roll[1], roll[0]]))
}

fn max_valid_re_entry_moves_single_roll(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    roll: &[Die],
) -> usize {
    let (die_x, die_y) = (roll[0], roll[1]);
    let re_entry_move = MoveWithDie::from_bar_with_die(player_color, die_x);
    if re_entry_move.is_valid_re_entry(board, player_color) {
        let temp_board = board.apply_n_moves(&re_entry_move, player_color, 1);
        if temp_board.checkers_at(player_color, bar_pos(player_color)) == 0 {
            return 1 + max_valid_normal_or_bearing_off_moves_one_die(
                &temp_board,
                player_color,
                die_y,
            );
        }
        let re_entry_move = MoveWithDie::from_bar_with_die(player_color, die_y);
        if re_entry_move.is_valid_re_entry(&temp_board, player_color) {
            return 2;
        }
        return 1;
    }

    let re_entry_move = MoveWithDie::from_bar_with_die(player_color, die_y);
    if re_entry_move.is_valid_re_entry(board, player_color) {
        let temp_board = board.apply_n_moves(&re_entry_move, player_color, 1);

        if temp_board.checkers_at(player_color, bar_pos(player_color)) == 0 {
            return 1 + max_valid_normal_or_bearing_off_moves_one_die(
                &temp_board,
                player_color,
                die_x,
            );
        }
        return 1;
    }

    0
}

fn max_valid_normal_or_bearing_off_moves_one_die(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    die: Die,
) -> usize {
    for pos in PosIter::full_board(player_color, None) {
        if board.checkers_at(player_color, pos) == 0 {
            continue;
        }

        let m = MoveWithDie::with_die(player_color, pos, die);

        if m.is_valid_normal(board, player_color) || m.is_valid_boff(board, player_color, die) {
            return 1;
        }
    }

    0
}

fn max_valid_moves_single_roll(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    roll: &[Die],
) -> usize {
    let (die_x, die_y) = (roll[0], roll[1]);

    let mut moves = 0;

    for ipos in PosIter::full_board(player_color, None) {
        if board.checkers_at(player_color, ipos) == 0 {
            continue;
        }

        let m = MoveWithDie::with_die(player_color, ipos, die_x);

        if m.is_valid_normal(board, player_color) || m.is_valid_boff(board, player_color, die_x) {
            moves = 1;

            let temp_board = board.apply_n_moves(&m, player_color, 1);

            for jpos in PosIter::full_board(player_color, Some(ipos)) {
                if temp_board.checkers_at(player_color, jpos) == 0 {
                    continue;
                }

                let m = MoveWithDie::with_die(player_color, jpos, die_y);
                if m.is_valid_normal(&temp_board, player_color)
                    || m.is_valid_boff(&temp_board, player_color, die_y)
                {
                    return 2;
                }
            }
        }
    }

    moves
}

fn max_valid_re_entry_moves_double_roll(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    die: Die,
    n: usize,
) -> usize {
    let re_entry_move = MoveWithDie::from_bar_with_die(player_color, die);
    if re_entry_move.is_valid_re_entry(board, player_color) {
        let bar_chk = board.checkers_at(player_color, bar_pos(player_color));

        if bar_chk > n {
            return n;
        }

        let temp_board = board.apply_n_moves(&re_entry_move, player_color, 1);

        return bar_chk as usize
            + max_n_valid_moves_for_double_roll(&temp_board, player_color, die, n - bar_chk);
    }

    0
}

fn max_n_valid_moves_for_double_roll(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    die: Die,
    n: usize,
) -> usize {
    let mut board = board.clone();

    let mut moves = 0;

    for pos in PosIter::full_board(player_color, None) {
        let chk = board.checkers_at(player_color, pos);
        if chk == 0 {
            continue;
        }

        let m = MoveWithDie::with_die(player_color, pos, die);
        if m.is_valid_normal(&board, player_color) {
            if moves + chk >= n {
                return n;
            }

            let max_jumps = ((n - moves) as f32 / chk as f32).ceil() as usize;
            let jumps = max_n_jumps_with_one_die(&board, player_color, pos, die, max_jumps);

            moves += chk * jumps;
            if moves >= n {
                return n;
            }

            board = board.apply_n_moves(
                &MoveWithDie::with_die(player_color, pos, die * jumps as u8),
                player_color,
                chk,
            );
        } else if m.is_valid_boff(&board, player_color, die) {
            moves += chk;
            if moves >= n {
                return n;
            }

            board = board.apply_n_moves(&m, player_color, chk)
        }
    }

    moves
}

fn max_n_jumps_with_one_die(
    board: &BackgammonBoard,
    player_color: PlayerColor,
    pos: Position,
    die: Die,
    n: usize,
) -> usize {
    let mut jumps = 1;

    if n == 1 {
        return n;
    }

    let mut last_valid_dst = (pos as i8 + (player_color * die as i8)) as Position;

    loop {
        let m = MoveWithDie::with_die(player_color, pos, die * (jumps + 1));

        if m.is_valid_normal(board, player_color) {
            jumps += 1;

            if jumps >= n as u8 {
                return jumps as usize;
            }

            last_valid_dst = m.dst;
        } else {
            let temp_board = board.apply_n_moves(
                &MoveWithDie {
                    src: pos,
                    dst: last_valid_dst,
                    die: 0,
                },
                player_color,
                1,
            );
            let m = MoveWithDie::with_die(player_color, last_valid_dst, die);

            if m.is_valid_boff(&temp_board, player_color, die) {
                return (jumps + 1) as usize;
            }
            break;
        }
    }

    jumps as usize
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;

    use crate::backgammon::players::{b, r, BLACK_PLAYER};

    use super::*;

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    macro_rules! ms {
        (($src:literal, $dst:literal, $die:literal)) => {
            MoveSequence::SingleMove(MoveWithDie {
                src: $src,
                dst: $dst,
                die: $die,
            })
        };
        (($src1:literal, $dst1:literal, $die1:literal), ($src2:literal, $dst2:literal, $die2:literal)) => {
            MoveSequence::TwoMoves(
                MoveWithDie {
                    src: $src1,
                    dst: $dst1,
                    die: $die1,
                },
                MoveWithDie {
                    src: $src2,
                    dst: $dst2,
                    die: $die2,
                },
            )
        };
    }

    #[test]
    fn test_move_with_die_hash() {
        let m1 = MoveWithDie::with_die(BLACK_PLAYER, 10, 3);
        let m2 = MoveWithDie::with_die(BLACK_PLAYER, 10, 3);

        assert_eq!(m1, m2);
        assert_eq!(calculate_hash(&m1), calculate_hash(&m2));

        let m1 = MoveWithDie::with_die(BLACK_PLAYER, 3, 3);
        let m2 = MoveWithDie::with_die(BLACK_PLAYER, 3, 6);

        assert_eq!(m1, m2);
        assert_eq!(calculate_hash(&m1), calculate_hash(&m2));
    }

    #[test]
    fn test_hash_set() {
        let mut set = HashSet::new();

        set.insert(ms!((24, 23, 1), (13, 11, 2)));
        set.insert(ms!((13, 11, 2), (24, 23, 1)));

        assert!(set.len() == 1);
    }

    #[test]
    fn test_move_sequence_hash() {
        let mvs1 = ms!((24, 23, 1), (13, 11, 2));
        let mvs2 = ms!((13, 11, 2), (24, 23, 1));

        assert_eq!(mvs1, mvs2);
        assert_eq!(calculate_hash(&mvs1), calculate_hash(&mvs2));

        let mvs1 = ms!((23, 25, 5), (21, 23, 2));
        let mvs2 = ms!((21, 23, 2), (23, 25, 6));

        assert_eq!(mvs1, mvs2);
        assert_eq!(calculate_hash(&mvs1), calculate_hash(&mvs2));
    }

    #[test]
    fn test_max_valid_moves() {
        let board = BackgammonBoard::default_board();
        let max_moves = max_valid_moves(&board, BLACK_PLAYER, &DiceRoll::Single(2, 1), None);
        assert_eq!(max_moves, 2);

        let max_moves = max_valid_moves(&board, BLACK_PLAYER, &DiceRoll::Double(1), None);
        assert_eq!(max_moves, 4);
    }

    #[test]
    fn test_generate_all_move_sequences() {
        let board = BackgammonBoard::default_board();
        let mut sequence =
            generate_all_move_sequences(&board, BLACK_PLAYER, &DiceRoll::Single(1, 2), None);
        sequence.sort();
        let mut expected_sequence = vec![
            ms!((24, 23, 1), (24, 22, 2)),
            ms!((24, 23, 1), (23, 21, 2)),
            ms!((24, 23, 1), (13, 11, 2)),
            ms!((24, 23, 1), (8, 6, 2)),
            ms!((24, 23, 1), (6, 4, 2)),
            ms!((8, 7, 1), (24, 22, 2)),
            ms!((8, 7, 1), (13, 11, 2)),
            ms!((8, 7, 1), (8, 6, 2)),
            ms!((8, 7, 1), (7, 5, 2)),
            ms!((8, 7, 1), (6, 4, 2)),
            ms!((6, 5, 1), (24, 22, 2)),
            ms!((6, 5, 1), (13, 11, 2)),
            ms!((6, 5, 1), (8, 6, 2)),
            ms!((6, 5, 1), (6, 4, 2)),
            ms!((6, 5, 1), (5, 3, 2)),
            ms!((24, 22, 2), (22, 21, 1)),
            ms!((13, 11, 2), (11, 10, 1)),
            ms!((6, 4, 2), (4, 3, 1)),
        ];
        expected_sequence.sort();
        assert_eq!(sequence, expected_sequence);
    }

    #[test]
    fn test_generate_all_move_sequences_2() {
        let board = BackgammonBoard::from_pairs(&[
            (18, r(15)),
            (19, b(3)),
            (20, b(2)),
            (21, b(2)),
            (22, b(2)),
            (23, b(2)),
            (24, b(2)),
            (26, b(2)),
        ]);
        let mut sequence =
            generate_all_move_sequences(&board, BLACK_PLAYER, &DiceRoll::Single(1, 2), None);
        sequence.sort();
        assert_ne!(sequence, vec![]);
    }
}
