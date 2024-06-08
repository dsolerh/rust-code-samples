use burn::{
    config::Config,
    module::Module,
    nn::{Linear, LinearConfig},
    tensor::{activation::sigmoid, backend::Backend, ElementConversion, Tensor},
};

use crate::core::{
    board::BackgammonBoard,
    dice::DiceRoll,
    moves::{generate_all_move_sequences, MoveSequence},
    players::{PlayerColor, BLACK_PLAYER, RED_PLAYER},
    positions::bar_pos,
};

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    input: Linear<B>,
    output: Linear<B>,
}

#[derive(Config, Debug)]
pub struct ModelConfig {
    input_size: usize,
    hidden_size: usize,
}

impl ModelConfig {
    /// Returns the initialized model.
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model {
            input: LinearConfig::new(self.input_size, self.hidden_size).init(device),
            output: LinearConfig::new(self.hidden_size, 1).init(device),
        }
    }
}

impl<B: Backend> Model<B> {
    /// # Shapes
    ///   - Images [batch_size, height, width]
    ///   - Output [batch_size, num_classes]
    pub fn forward(&self, features: Tensor<B, 2>) -> Tensor<B, 1> {
        let x = self.input.forward(features);
        let x = sigmoid(x);

        let x = self.output.forward(x);
        sigmoid(x).reshape([1])
    }

    pub(crate) fn get_board_features(
        &self,
        board: &BackgammonBoard,
        player_color: PlayerColor,
    ) -> Tensor<B, 2> {
        let mut features_vector = Vec::with_capacity(198);

        let mut player_feat = match player_color {
            BLACK_PLAYER => vec![0.0, 1.0],
            RED_PLAYER => vec![1.0, 0.0],
            _ => panic!("invalid player color"),
        };

        for player_color in [BLACK_PLAYER, RED_PLAYER] {
            for pos in 1..=24 {
                let mut feats = match board.points_at(pos) {
                    0 => vec![0.0, 0.0, 0.0, 0.0],
                    1 => vec![1.0, 0.0, 0.0, 0.0],
                    2 => vec![1.0, 1.0, 0.0, 0.0],
                    3 => vec![1.0, 1.0, 1.0, 0.0],
                    p => vec![1.0, 1.0, 1.0, ((p - 3) as f32) / 2.0],
                };
                features_vector.append(&mut feats)
            }

            let bar_points = board.checkers_at(player_color, bar_pos(player_color));
            let non_bar_points = 15 - bar_points;

            features_vector.push(bar_points as f32 / 2.0);
            features_vector.push(non_bar_points as f32 / 15.0);
        }

        features_vector.append(&mut player_feat);

        assert_eq!(features_vector.len(), 198);

        Tensor::from_floats(features_vector.as_slice(), &B::Device::default()).reshape([2, 198])
    }

    pub(crate) fn get_action(
        &self,
        board: &BackgammonBoard,
        player_color: PlayerColor,
        roll: &DiceRoll,
    ) -> Option<MoveSequence> {
        let move_sequences = generate_all_move_sequences(board, player_color, roll, None);

        if move_sequences.len() == 0 {
            return None;
        }

        move_sequences
            .into_iter()
            .map(|move_sequence| {
                let temp_board = board.apply_move_sequence(&move_sequence, player_color);
                let board_features = self.get_board_features(&temp_board, player_color);
                let score = self.forward(board_features);
                let score = score.sum().into_scalar().elem::<f32>();
                (score, move_sequence)
            })
            .max_by(|(x, _), (y, _)| x.partial_cmp(y).unwrap())
            .map(|val| val.1)
    }
}
