pub mod board {

    use std::u8;

    use crate::piece::piece::{Piece};
    use crate::game::game::{Game};

    #[derive(Debug, Clone)]
    pub struct Board {
        pub board_value: u64,
        pub board_state: [Option<Piece>; 64],
    }

    impl Board {
        pub fn new() -> Board {
            return Board {
                board_value: 0,
                board_state: [None; 64]
            };
        }

        pub fn new_from_arr(state: [Option<Piece>; 64]) -> Board {
            return Board {
                board_value: Self::get_board_value(state),
                board_state: state
            };
        }

        pub fn get_board_value(state: [Option<Piece>; 64]) -> u64 {
            let mut board_value: u64 = 0;
            for x in state.iter().rev() {
                match x {
                    None => { board_value = board_value << 1},
                    Some(_) => {
                       board_value = (board_value << 1) + 1; 
                    }
                }
            }

            return board_value;
        }

        pub fn compute_hash2(&self) -> u64 {
            let mut tot = 0;
            for i in 0..64 {
                let piece_value: u64 = Piece::piece_to_u64(&self.board_state[i]);
                tot += piece_value * (i as u64);
            }

            tot
        }

        pub fn compute_hash(&self) -> u64 { 
            let mut tot = 0;
            for i in 0..64 {
                let piece_value: u64 = Piece::piece_to_u64(&self.board_state[i]);
                tot += piece_value * (i as u64);
            }

            tot
        }

        pub fn get_piece_from_position(board: &Board, position: u8) -> Option<Piece> {
            return board.board_state[position as usize];
        }

        pub fn move_piece(game: Game, from: u8, _to: u8) -> Game {
            if Board::get_board_state_from_position(&game.board, &from) {
                let _piece: Option<Piece> = Board::get_piece_from_position(&game.board, from);
            }

            return Game { board: game.board };
        }

        pub fn get_board_state_from_position(board: &Board, position: &u8) -> bool {
            if *position > 63 {
                return false;
            }

            if (board.board_value & (1 << position)) > 0 {
                return true; 
            }
            return false;
        }

    }
}
