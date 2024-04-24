
pub mod game {
    use crate::board::board::{Board, Move};
    use crate::move_gen::move_gen::MoveGen;

    #[derive(Clone, Copy)]
    pub struct Game {
        pub board: Board,
        pub is_white_turn: bool,
        pub move_gen: MoveGen
    }

    impl Game {
       pub fn new(is_white_turn: bool) -> Game {
            return Game {
                board: Board::new(),
                is_white_turn: is_white_turn,
                move_gen: MoveGen::init()
            }
       }

        pub fn new_from_string(state: String, is_white_turn: bool) -> Game {
            let board: Board = Board::from_string(state);
            Game {
                board: board,
                is_white_turn: is_white_turn,
                move_gen: MoveGen::init()
            }
        }

        pub fn get_all_moves(&self) -> Vec<Move> {
            return self.board.get_all_moves(&self.move_gen, self.is_white_turn)
        }

        pub fn make_move(&mut self, m: &Move) -> () {
            match m {
                Move::Standard(pos, to, piece_type) => {
                    self.board.make_move(&piece_type, &pos, &to, self.is_white_turn);
                },
                Move::Promotion(pos, to, piece_type) => {

                }
                _ => ()
            }

            self.is_white_turn = !self.is_white_turn
        }

    }

}