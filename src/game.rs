
pub mod game {
 
    use crate::board::board::{Board};
    use crate::piece::piece::Piece;

    pub struct Game {
        pub board: Board    
    }

    impl Game {
       pub fn new() -> Game {
            return Game { board: Board::new()}
       }

       pub fn new_from_arr(state: [Option<Piece>; 64]) -> Game {
            Game { board: Board::new_from_arr(state) }
       }

        pub fn get_all_moves(&self) -> Vec<u64> {
            let mut moves: Vec<u64> = vec![];
            for i in 0..64 {
                match self.board.board_state[i] {
                    Some(piece) => {
                        let mut piece_moves = piece.get_moves(&self.board, &0);
                        moves.append(&mut piece_moves);
                    },
                    None => {}
                }
            }
            moves
        }
    }

}

