pub mod game {
    use std::cmp::PartialEq;
    use std::ptr::copy;
    use Move::{Capture, Promotion, Standard};
    use PieceType::{BISHOP, KNIGHT, PAWN, QUEEN};
    use crate::board::board::{Board, BoardMove, Move};
    use crate::board::board::Move::Castle;
    use crate::board::board::Side::{Left, Right};
    use crate::debug::debug::print_board_from_board;
    use crate::move_gen::move_gen::{MoveGen, PieceType};
    use crate::move_gen::move_gen::PieceType::{KING, ROOK};
    use crate::move_list::move_list::{AttackMoveList, MoveList};
    use crate::utils::utils;

    #[derive(Clone)]
    pub struct Game {
        pub board: Board,
        pub is_white_turn: bool,
        pub move_gen: MoveGen,
        pub white_attack_boards: Vec<BoardMove>,
        pub black_attack_boards: Vec<BoardMove>,
        pub move_log: Vec<Move>
    }

    impl PartialEq<PieceType> for &PieceType {
        fn eq(&self, other: &PieceType) -> bool {
            match self {
                PieceType::None => {
                    match other {
                        PieceType::None => return true,
                        _ => return false
                    }
                }
                PAWN => {
                    match other {
                        PAWN => return true,
                        _ => return false
                    }
                }
                ROOK => {
                    match other {
                        ROOK => return true,
                        _ => return false
                    }
                }
                KING => {
                    match other {
                        KING => return true,
                        _ => return false
                    }
                }
                KNIGHT => {
                    match other {
                        KNIGHT => return true,
                        _ => return false
                    }
                }
                BISHOP => {
                    match other {
                        BISHOP => return true,
                        _ => return false
                    }
                }
                QUEEN => {
                    match other {
                        QUEEN => return true,
                        _ => return false
                    }
                }
            }
        }
    }

    impl Game {
       pub fn new(is_white_turn: bool) -> Game {
            return Game {
                board: Board::new(),
                is_white_turn: is_white_turn,
                move_gen: MoveGen::init(),
                white_attack_boards: Vec::with_capacity(16),
                black_attack_boards: Vec::with_capacity(16),
                move_log: Vec::with_capacity(2000),
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
                move_log: Vec::with_capacity(2000),
            }
        }

        pub fn get_all_moves(&mut self) -> (Vec<Move>, u64, u64) {

            let mut moves = Vec::with_capacity(250);

            let occupancy = self.board.get_board_value();
            let white_occupancy = self.board.get_white_occupancy();
            let black_occupancy = self.board.get_black_occupancy();

            self.black_attack_boards.clear();
            self.white_attack_boards.clear();

            self.board.get_moves(white_occupancy, black_occupancy, occupancy, true, &mut self.white_attack_boards, &self.move_gen);

            self.board.get_moves(black_occupancy, white_occupancy, occupancy, false, &mut self.black_attack_boards, &self.move_gen);

            let complete_attack_board_white = Self::attack_boards_to_attacked_squares(&self.white_attack_boards);

            let complete_attack_board_black = Self::attack_boards_to_attacked_squares(&self.black_attack_boards);

            let mut white_king_board = self.board.white_king_board;
            let mut black_king_board = self.board.black_king_board;

            for _ in 0..(white_king_board.count_ones() as usize) {
                let lsb = utils::pop_lsb(&mut white_king_board);
                let b = BoardMove {
                    attack_board: self.move_gen.get_move(KING, lsb, white_occupancy, occupancy, black_occupancy, true) & !complete_attack_board_black,
                    piece_type: KING,
                    position: u8::try_from(lsb).unwrap(),
                    white: true,
                };
                self.white_attack_boards.push(b)
            }

            for _ in 0..(black_king_board.count_ones() as usize) {
                let lsb = utils::pop_lsb(&mut black_king_board);
                let b = BoardMove {
                    attack_board: self.move_gen.get_move(KING, lsb, black_occupancy, occupancy, white_occupancy, true) & !complete_attack_board_white,
                    piece_type: KING,
                    position: u8::try_from(lsb).unwrap(),
                    white: true,
                };
                self.black_attack_boards.push(b)
            }


            if self.is_white_turn {
                self.board.attack_boards_to_moves(&self.white_attack_boards, &mut moves, true, black_occupancy);
            } else {
                self.board.attack_boards_to_moves(&self.black_attack_boards, &mut moves,false, white_occupancy);
            }

            return (moves, complete_attack_board_white, complete_attack_board_black)
        }

        pub fn get_castling_rights_white(&self, occupancy: &u64, opponent_attacked_squares: &u64) -> (bool, bool) {
            let mut left_possible = true;
            let mut right_possible = true;
            let check_left: u64 = 6_917_529_027_641_081_856;
            let check_right: u64 = 864_691_128_455_135_232;
            let king_position: u64 = 1_152_921_504_606_846_976;
            //let rook_right_position: u64 = 9_223_372_036_854_775_808;
            //let rook_left_position: u64 = 72_057_594_037_927_936;

            if self.board.white_king_board == 0 {
                return (false, false)
            }

            if self.board.white_king_board & king_position == 0 {
                return (false, false)
            }

            if opponent_attacked_squares & check_left > 0 || occupancy & check_left > 0  {
                left_possible = false;
            }

            if opponent_attacked_squares & check_right > 0 || occupancy & check_right > 0 {
                right_possible = false;
            }


            for x in self.move_log.iter() {
                match *x {
                    Standard(_, _, KING, true) => {
                        left_possible = false;
                        right_possible = false;
                    }
                    Capture(_, _, KING, _, true) => {
                        left_possible = false;
                        right_possible = false;
                    }
                    Castle(_, _, true) => {
                        left_possible = false;
                        right_possible = false;
                    }
                    Standard(56, _, ROOK, true) => {
                        left_possible = false;
                    },
                    Capture(56, _, ROOK, _, true) => {
                        left_possible = false;
                    }
                    Standard(63, _, ROOK, true) => {
                        right_possible = false;
                    },
                    Capture(63, _, ROOK, _, true) => {
                        right_possible = false;
                    }
                    _ => ()
                }
            }

            return (left_possible, right_possible)
        }

        pub fn get_castling_rights_black(&self, occupancy: &u64, opponent_attacked_squares: &u64) -> (bool, bool) {
            let mut left_possible = true;
            let mut right_possible = true;
            let check_left: u64 = 12;
            let check_right: u64 = 96;
            let king_position: u64 = 16;

            if self.board.black_king_board == 0 {
                return (false, false)
            }

            if self.board.white_king_board & king_position == 0 {
                return (false, false)
            }

            if opponent_attacked_squares & check_left > 0 || occupancy & check_left > 0 {
                left_possible = false;
            }

            if opponent_attacked_squares & check_right > 0 || occupancy & check_right > 0 {
                right_possible = false;
            }


            for x in self.move_log.iter() {
                match *x {
                    Standard(_, _, KING, false) => {
                        left_possible = false;
                        right_possible = false;
                    }
                    Capture(_, _, KING, _, false) => {
                        left_possible = false;
                        right_possible = false;
                    }
                    Castle(_, _, false) => {
                        left_possible = false;
                        right_possible = false;
                    }
                    Standard(0, _, ROOK, false) => {
                        left_possible = false;
                    },
                    Capture(0, _, ROOK, _, false) => {
                        left_possible = false;
                    }
                    Standard(7, _, ROOK, false) => {
                        right_possible = false;
                    },
                    Capture(7, _, ROOK, _, false) => {
                        right_possible = false;
                    }
                    _ => ()
                }
            }

            return (left_possible, right_possible)
        }

        pub fn attack_boards_to_attacked_squares(list: &Vec<BoardMove>) -> u64 {
            let mut sum = 0;
            for i in list.iter() {
                sum |= i.attack_board;
            }

            return sum
        }

        pub fn make_move(&mut self, m: &Move) -> () {
            //println!("{:?}", m);

            let x = self.board.make_move(&m, self.is_white_turn);

            let mut from = 0;
            let mut to = 0;

            match m {
                Move::None => {}
                Standard(_from, _to, _, _) => {
                    from = *_from;
                    to = *_to;
                }
                Capture(_from, _to, _, _, _) => {
                    from = *_from;
                    to = *_to;
                }
                Promotion(_from, _to, _, _, _) => {
                    from = *_from;
                    to = *_to;
                }
                Castle(_from, _, c) => {
                    from = *_from;
                }
            }

            self.move_log.push(*m);

            if self.board.check_for_multiple_pieces(from as usize) >= 2 {
                self.panic_with_trail()
            }
            if self.board.check_for_multiple_pieces(to as usize) >= 2 {
                self.panic_with_trail()
            }

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

        pub fn panic_with_trail(&self) {

            println!("--------------------------------------------------------------------\n--------------------------------------------------------------------");

            print_board_from_board(&self.board);

            println!("move log len: {}", self.move_log.len());
            println!("{:?}", self.move_log);


            for (i, x) in self.move_log.iter().enumerate() {
                println!("{}: {}", i, x.to_printable())
            }

            println!("--------------------------------------------------------------------\n--------------------------------------------------------------------");

            panic!("Crashed")

        }

    }

}