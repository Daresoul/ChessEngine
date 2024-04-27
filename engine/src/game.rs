pub mod game {
    use crate::board::board::{Board, BoardMove, Move};
    use crate::debug::debug::print_board_from_board;
    use crate::move_gen::move_gen::MoveGen;
    use crate::move_list::move_list::{AttackMoveList, MoveList};

    #[derive(Clone)]
    pub struct Game {
        pub board: Board,
        pub is_white_turn: bool,
        pub move_gen: MoveGen,
        pub white_attack_boards: Vec<BoardMove>,
        pub black_attack_boards: Vec<BoardMove>,
        pub move_log: Vec<Move>
    }

    impl Game {
       pub fn new(is_white_turn: bool) -> Game {
            return Game {
                board: Board::new(),
                is_white_turn: is_white_turn,
                move_gen: MoveGen::init(),
                white_attack_boards: Vec::with_capacity(16),
                black_attack_boards: Vec::with_capacity(16),
                move_log: Vec::with_capacity(2000)
            }
       }

        pub fn new_from_string(state: String, is_white_turn: bool) -> Game {
            let board: Board = Board::from_string(state);
            Game {
                board: board,
                is_white_turn: is_white_turn,
                move_gen: MoveGen::init(),
                white_attack_boards: Vec::with_capacity(16),
                black_attack_boards: Vec::with_capacity(16),
                move_log: Vec::with_capacity(2000)
            }
        }

        pub fn get_all_moves(&mut self) -> Vec<Move> {

            let mut moves = Vec::with_capacity(250);

            let occupancy = self.board.get_board_value();
            let white_occupancy = self.board.get_white_occupancy();
            let black_occupancy = self.board.get_black_occupancy();

            self.black_attack_boards.clear();
            self.white_attack_boards.clear();


            self.board.get_moves(white_occupancy, black_occupancy, occupancy, true, &mut self.white_attack_boards, &self.move_gen);

            self.board.get_moves(black_occupancy, white_occupancy, occupancy, false, &mut self.black_attack_boards, &self.move_gen);

            if self.is_white_turn {
                self.board.attack_boards_to_moves(&self.white_attack_boards, &mut moves, true);
            } else {
                self.board.attack_boards_to_moves(&self.black_attack_boards, &mut moves,false);
            }

            return moves
        }

        pub fn make_move(&mut self, m: &Move) -> () {
            let Actual_move: Move = match m {
                Move::Standard(pos, to, piece_type, is_white) => {
                    let x = self.board.make_move(&piece_type, &pos, &to, self.is_white_turn, false);
                    if self.board.check_for_multiple_pieces(*pos as usize) >= 2 {
                        print_board_from_board(&self.board);
                        panic!("The move {:?} resultet in {:?} and crashed the program at the from position.", m, x);
                    }
                    if self.board.check_for_multiple_pieces(*to as usize) >= 2 {
                        print_board_from_board(&self.board);
                        panic!("The move {:?} resultet in {:?} and crashed the program at the to position.", m, x);
                    }
                    x
                },
                Move::Promotion(pos, to, piece_type, cp, is_white) => {
                    let x = self.board.make_move(&piece_type, &pos, &to, self.is_white_turn, true);
                    if self.board.check_for_multiple_pieces(*pos as usize) >= 2 {
                        print_board_from_board(&self.board);
                        panic!("The move {:?} resultet in {:?} and crashed the program.", m, x);
                    }
                    if self.board.check_for_multiple_pieces(*to as usize) >= 2 {
                        print_board_from_board(&self.board);
                        panic!("The move {:?} resultet in {:?} and crashed the program.", m, x);
                    }
                    x
                }
                _ => Move::None
            };

            if Actual_move == Move::None {
                print!("{:?}", m)
            }

            self.move_log.push(Actual_move);

            self.is_white_turn = !self.is_white_turn
        }

        pub fn undo_move(&mut self) -> () {
            let m_option = self.move_log.pop();
            match m_option {
                Some(m) => {
                    self.board.undo_move(m);
                },
                None => panic!("Went for undo move too many times")
            }

            self.is_white_turn = !self.is_white_turn;
        }

    }

}