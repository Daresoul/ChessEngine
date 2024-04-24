
pub mod game {
    use crate::board::board::{Board, BoardMove, Move};
    use crate::move_gen::move_gen::MoveGen;
    use crate::move_gen::move_gen::PieceType::PAWN;

    #[derive(Clone)]
    pub struct Game {
        pub board: Board,
        pub is_white_turn: bool,
        pub move_gen: MoveGen,
        pub white_attack_boards: [BoardMove; 16],
        pub white_attack_boards_len: usize,
        pub white_moves: [Move; 250],
        pub white_moves_len: usize,
        pub black_attack_boards: [BoardMove; 16],
        pub black_attack_boards_len: usize,
        pub black_moves: [Move; 250],
        pub black_moves_len: usize,
    }

    impl Game {
       pub fn new(is_white_turn: bool) -> Game {
            return Game {
                board: Board::new(),
                is_white_turn: is_white_turn,
                move_gen: MoveGen::init(),
                white_attack_boards: [BoardMove {
                    attack_board: 0,
                    piece_type: PAWN,
                    position: 0,
                    white: true
                }; 16],
                white_attack_boards_len: 0,
                white_moves: [Move::None; 250],
                white_moves_len: 0,
                black_attack_boards: [BoardMove {
                    attack_board: 0,
                    piece_type: PAWN,
                    position: 0,
                    white: true
                }; 16],
                black_attack_boards_len: 0,
                black_moves: [Move::None; 250],
                black_moves_len: 0
            }
       }

        pub fn new_from_string(state: String, is_white_turn: bool) -> Game {
            let board: Board = Board::from_string(state);
            Game {
                board: board,
                is_white_turn: is_white_turn,
                move_gen: MoveGen::init(),
                white_attack_boards: [BoardMove {
                    attack_board: 0,
                    piece_type: PAWN,
                    position: 0,
                    white: true
                }; 16],
                white_attack_boards_len: 0,
                white_moves: [Move::None; 250],
                white_moves_len: 0,
                black_attack_boards: [BoardMove {
                    attack_board: 0,
                    piece_type: PAWN,
                    position: 0,
                    white: true
                }; 16],
                black_attack_boards_len: 0,
                black_moves: [Move::None; 250],
                black_moves_len: 0
            }
        }

        pub fn reset_white_moves(&mut self) -> () {
            unsafe {
                std::ptr::write_bytes(self.white_moves.as_mut_ptr(), 0, self.white_moves.len());
            }
        }

        pub fn reset_white_attack_boards(&mut self) -> () {
            unsafe {
                std::ptr::write_bytes(self.white_attack_boards.as_mut_ptr(), 0, self.white_attack_boards.len());
            }
        }

        pub fn reset_black_moves(&mut self) -> () {
            unsafe {
                std::ptr::write_bytes(self.black_moves.as_mut_ptr(), 0, self.black_moves.len());
            }
        }

        pub fn reset_black_attack_boards(&mut self) -> () {
            unsafe {
                std::ptr::write_bytes(self.black_attack_boards.as_mut_ptr(), 0, self.black_attack_boards.len());
            }
        }

        pub fn get_all_moves(&mut self) -> () {

            let occupancy = self.board.get_board_value();
            let white_occupancy = self.board.get_white_occupancy();
            let black_occupancy = self.board.get_black_occupancy();

            self.reset_black_attack_boards();
            self.reset_white_attack_boards();

            if self.is_white_turn {self.reset_white_moves()} else {self.reset_black_moves()}

            self.white_attack_boards_len = self.board.get_moves(white_occupancy, black_occupancy, occupancy, true, &mut self.white_attack_boards, &self.move_gen);

            self.black_attack_boards_len = self.board.get_moves(black_occupancy, white_occupancy, occupancy, false, &mut self.black_attack_boards, &self.move_gen);

            if self.is_white_turn {
                self.white_moves_len = self.board.attack_boards_to_moves(&self.white_attack_boards, &mut self.white_moves, true);
            } else {
                self.black_moves_len = self.board.attack_boards_to_moves(&self.black_attack_boards, &mut self.white_moves,false);
            }
        }

        pub fn make_move(&mut self, m: &Move) -> () {
            match m {
                Move::Standard(pos, to, piece_type) => {
                    self.board.make_move(&piece_type, &pos, &to, self.is_white_turn);
                },
                Move::Promotion(_pos, _to, _piece_type) => {

                }
                _ => ()
            }

            self.is_white_turn = !self.is_white_turn
        }

    }

}