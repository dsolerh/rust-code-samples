// pub(crate) struct LogicAgent {
//     config: LogicConfig,
// }
//
// #[derive(Debug, Clone, Copy)]
// struct LogicConfig {
//     initial_phase: PhaseConfig,
//     bear_off: PhaseConfig,
//     bear_off_race: PhaseConfig,
//     normal_race: PhaseConfig,
// }
//
// impl LogicConfig {
//     fn get_phase_config(&self, game_phase: GamePhase) -> PhaseConfig {
//         match game_phase {
//             GamePhase::Initial => self.initial_phase,
//             GamePhase::BearOff => self.bear_off,
//             GamePhase::NormalRace => self.normal_race,
//             GamePhase::BearOffRace => self.bear_off_race,
//         }
//     }
// }
//
// #[derive(Debug, Clone, Copy)]
// struct PhaseConfig {
//     pip_count_weight: f32,
//     anchor_count_wieght: f32,
//     blot_count_weight: f32,
//     blots_in_home_board_weight: f32,
//     checker_in_opponent_home_board_weight: f32,
//     prime_in_home_board_weight: f32,
//     prime_lenght_weight: f32,
// }
//
// impl PhaseConfig {
//     fn get_max_positive(&self) -> f32 {
//         self.pip_count_weight.abs()
//             + self.prime_lenght_weight.abs()
//             + self.anchor_count_wieght.abs()
//             + self.prime_in_home_board_weight.abs()
//     }
//
//     fn get_max_negative(&self) -> f32 {
//         self.checker_in_opponent_home_board_weight.abs()
//             + self.blot_count_weight.abs()
//             + self.blots_in_home_board_weight.abs()
//     }
// }
//
// const MAX_PIP_COUNT_DIFF_IDEAL: f32 = 15.0 * 25.0; // 15 chk in the bar - 0 (all checkers are bear off) this limit is ideal
// const MAX_PIP_COUNT_DIFF: f32 = MAX_PIP_COUNT_DIFF_IDEAL / 3.0; // have a more reasonable value
// const MAX_PRIME_LENGHT: f32 = 7.0;
// const MAX_BLOTS_COUNT: f32 = 15.0;
// const MAX_ANCHORS_COUNT: f32 = 6.0;
// const MAX_CHECKERS: f32 = 15.0;
// const MAX_PRIME_HB: f32 = 6.0;
// const MAX_BLOTS_HB: f32 = 6.0;
//
// const HARD_LOGIC_CONFIG: LogicConfig = LogicConfig {
//     initial_phase: PhaseConfig {
//         pip_count_weight: 9.0,
//         anchor_count_wieght: 5.0,
//         blot_count_weight: -4.0,
//         blots_in_home_board_weight: -7.0,
//         checker_in_opponent_home_board_weight: -9.0,
//         prime_in_home_board_weight: 8.0,
//         prime_lenght_weight: 8.0,
//     },
//     bear_off: PhaseConfig {
//         pip_count_weight: 2.0,
//         anchor_count_wieght: 8.0,
//         blot_count_weight: -9.0,
//         blots_in_home_board_weight: -5.0,
//         checker_in_opponent_home_board_weight: -5.0,
//         prime_in_home_board_weight: 5.0,
//         prime_lenght_weight: 5.0,
//     },
//     bear_off_race: PhaseConfig {
//         pip_count_weight: 10.0,
//         anchor_count_wieght: 0.0,
//         blot_count_weight: 0.0,
//         blots_in_home_board_weight: 0.0,
//         checker_in_opponent_home_board_weight: 0.0,
//         prime_in_home_board_weight: 0.0,
//         prime_lenght_weight: 0.0,
//     },
//     normal_race: PhaseConfig {
//         pip_count_weight: 10.0,
//         anchor_count_wieght: 0.0,
//         blot_count_weight: 0.0,
//         blots_in_home_board_weight: 0.0,
//         checker_in_opponent_home_board_weight: 0.0,
//         prime_in_home_board_weight: 0.0,
//         prime_lenght_weight: 0.0,
//     },
// };
//
// impl LogicAgent {
//     pub(crate) fn new_hard() -> Self {
//         Self {
//             config: HARD_LOGIC_CONFIG,
//         }
//     }
//
//     fn score_board(&self, board: &BackgammonBoard, player_color: PlayerColor) -> f32 {
//         let game_phase = get_game_phase(board, player_color);
//         let phase_config = self.config.get_phase_config(game_phase);
//
//         let max_positive_weight = phase_config.get_max_positive();
//         let max_negative_weight = phase_config.get_max_negative();
//
//         let mut score = 0.0;
//
//         // positive
//         score += get_score(
//             board,
//             player_color,
//             MAX_PIP_COUNT_DIFF,
//             phase_config.pip_count_weight,
//             max_positive_weight,
//             get_pip_count_diff,
//         );
//         score += get_score(
//             board,
//             player_color,
//             MAX_PRIME_LENGHT,
//             phase_config.prime_lenght_weight,
//             max_positive_weight,
//             get_prime_length,
//         );
//         score += get_score(
//             board,
//             player_color,
//             MAX_ANCHORS_COUNT,
//             phase_config.anchor_count_wieght,
//             max_positive_weight,
//             get_anchors_count,
//         );
//         score += get_score(
//             board,
//             player_color,
//             MAX_PRIME_HB,
//             phase_config.prime_in_home_board_weight,
//             max_positive_weight,
//             get_prime_in_home_board_count,
//         );
//
//         // negative
//         score += get_score(
//             board,
//             player_color,
//             MAX_CHECKERS,
//             phase_config.checker_in_opponent_home_board_weight,
//             -max_negative_weight,
//             get_checkers_in_opponent_hb_count,
//         );
//         score += get_score(
//             board,
//             player_color,
//             MAX_BLOTS_COUNT,
//             phase_config.blot_count_weight,
//             -max_negative_weight,
//             get_blot_count,
//         );
//         score += get_score(
//             board,
//             player_color,
//             MAX_BLOTS_HB,
//             phase_config.blots_in_home_board_weight,
//             -max_negative_weight,
//             get_blots_in_home_board_count,
//         );
//
//         score
//     }
// }
//
// impl Agent for LogicAgent {
//     fn get_action(
//         &self,
//         board: &BackgammonBoard,
//         player_color: PlayerColor,
//         roll: &DiceRoll,
//     ) -> Option<MoveSequence> {
//         let move_sequences = generate_all_move_sequences(board, player_color, roll, None);
//
//         if move_sequences.len() == 0 {
//             return None;
//         }
//
//         move_sequences
//             .into_iter()
//             .map(|move_sequence| {
//                 let temp_board = board.apply_move_sequence(&move_sequence, player_color);
//                 let score = self.score_board(&temp_board, player_color);
//                 (score, move_sequence)
//             })
//             .max_by(|(x, _), (y, _)| x.partial_cmp(y).unwrap())
//             .map(|val| val.1)
//     }
// }
