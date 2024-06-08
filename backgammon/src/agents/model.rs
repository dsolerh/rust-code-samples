//
// pub(crate) struct ModelAgent<B: Backend> {
//     model: Model<B>,
// }
//
// impl<B: Backend> ModelAgent<B> {
//     pub(crate) fn new(device: &B::Device, input_size: usize, hidden_size: usize) -> Self {
//         Self {
//             model: ModelConfig::new(input_size, hidden_size).init(device),
//         }
//     }
//     pub(crate) fn with_model(model: Model<B>) -> Self {
//         Self { model }
//     }
// }
//
// impl<B: Backend> Agent for ModelAgent<B> {
//     fn get_action(
//         &self,
//         board: &BackgammonBoard,
//         player_color: PlayerColor,
//         roll: &DiceRoll,
//     ) -> Option<MoveSequence> {
//         self.model.get_action(board, player_color, roll)
//     }
// }
//
