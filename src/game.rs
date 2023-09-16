
pub mod game {
    use MoveType::{Defend, FutureMove, Promotion};
    use crate::board::board::{Board, MoveType};
    use crate::board::board::MoveType::{Attack, Capture, Castle, Standard};
    use crate::piece::piece::{Piece};
    use crate::piece::piece::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};

    #[derive(Clone, Copy)]
    pub struct Game {
        pub board: Board,
        pub is_white_turn: bool,
        pub white_king_moved: bool,
        pub black_king_moved: bool,
        pub white_rook_left_moved: bool,
        pub white_rook_right_moved: bool,
        pub black_rook_left_moved: bool,
        pub black_rook_right_moved: bool,
    }

    impl Game {
       pub fn new(is_white_turn: bool) -> Game {
            return Game {
                board: Board::new(),
                is_white_turn: is_white_turn,
                white_king_moved: false,
                black_king_moved: false,
                white_rook_left_moved: false,
                white_rook_right_moved: false,
                black_rook_left_moved: false,
                black_rook_right_moved: false,
            }
       }

       pub fn new_from_arr(state: [Option<Piece>; 64], is_white_turn: bool) -> Game {
            Game {
                board: Board::new_from_arr(state),
                is_white_turn: is_white_turn,
                white_king_moved: false,
                black_king_moved: false,
                white_rook_left_moved: false,
                white_rook_right_moved: false,
                black_rook_left_moved: false,
                black_rook_right_moved: false,
            }
       }

        pub fn new_from_string(state: String, is_white_turn: bool) -> Game {
            Game {
                board: Board::new_from_arr(Game::transform_string_to_state(state)),
                is_white_turn: is_white_turn,
                white_king_moved: false,
                black_king_moved: false,
                white_rook_left_moved: false,
                white_rook_right_moved: false,
                black_rook_left_moved: false,
                black_rook_right_moved: false,
            }
        }

        // Forsyth–Edwards Notation
        pub fn transform_string_to_state(state: String) -> [Option<Piece>; 64] {
            let mut board: [Option<Piece>; 64] = [None; 64];
            let mut board_index = 0;

            for c in state.chars() {
                if board_index > 63 {
                    break;
                }
                match c {
                    '1' => board_index += 1,
                    '2' => board_index += 2,
                    '3' => board_index += 3,
                    '4' => board_index += 4,
                    '5' => board_index += 5,
                    '6' => board_index += 6,
                    '7' => board_index += 7,
                    '8' => board_index += 8,
                    'P' => {board[board_index] = Some(Piece { piece_type: Pawn, is_white: true}); board_index += 1},
                    'p' => {board[board_index] = Some(Piece { piece_type: Pawn, is_white: false}); board_index += 1},
                    'R' => {board[board_index] = Some(Piece { piece_type: Rook, is_white: true}); board_index += 1},
                    'r' => {board[board_index] = Some(Piece { piece_type: Rook, is_white: false}); board_index += 1},
                    'N' => {board[board_index] = Some(Piece { piece_type: Knight, is_white: true}); board_index += 1},
                    'n' => {board[board_index] = Some(Piece { piece_type: Knight, is_white: false}); board_index += 1},
                    'B' => {board[board_index] = Some(Piece { piece_type: Bishop, is_white: true}); board_index += 1},
                    'b' => {board[board_index] = Some(Piece { piece_type: Bishop, is_white: false}); board_index += 1},
                    'Q' => {board[board_index] = Some(Piece { piece_type: Queen, is_white: true}); board_index += 1},
                    'q' => {board[board_index] = Some(Piece { piece_type: Queen, is_white: false}); board_index += 1},
                    'K' => {board[board_index] = Some(Piece { piece_type: King, is_white: true}); board_index += 1},
                    'k' => {board[board_index] = Some(Piece { piece_type: King, is_white: false}); board_index += 1},
                    _ => continue
                };
            }
            board
        }

        pub fn check_attack_castle_white(&self, moves: &Vec<MoveType>) -> (bool, bool) {
            let mut can_castle_left = true;
            let mut can_castle_right = true;


            for single_move in moves.iter() {
                match single_move {
                    Attack(_, _from, to, can_move, color) => {
                        if !*color && *can_move {
                            if *to == 60 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 58 || *to == 59 {
                                can_castle_left = false;
                            } else if *to == 61 || *to == 62 {
                                can_castle_right = false;
                            }
                        }
                    },
                    Promotion(_from, to, _, color) => {
                        if !*color {
                            if *to == 60 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 58 || *to == 59 {
                                can_castle_left = false;
                            } else if *to == 61 || *to == 62 {
                                can_castle_right = false;
                            }
                        }
                    },
                    Capture(_, from, to, cp, color) => {
                        if !*color {
                            if *to == 60 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 58 || *to == 59 {
                                can_castle_left = false;
                            } else if *to == 61 || *to == 62 {
                                can_castle_right = false;
                            }
                        }
                    }
                    _ => continue
                }
            }

            (can_castle_left,can_castle_right)
        }

        pub fn castle_white(&self, all_moves: &Vec<MoveType>) -> Vec<MoveType> {
            let mut moves = vec![];


            let (can_left, can_right) = self.check_attack_castle_white(all_moves);

            // Check spaces between king and rook left are empty
            if can_left {
                if !Board::get_board_state_from_position(&self.board, &57) &&
                    !Board::get_board_state_from_position(&self.board, &58) &&
                    !Board::get_board_state_from_position(&self.board, &59) {
                    moves.push(Castle(56, 59, 60, 58, true));
                }
            }

            // Check spaces between king and rook right are empty
            if can_right {
                if !Board::get_board_state_from_position(&self.board, &61) &&
                    !Board::get_board_state_from_position(&self.board, &62) {
                    moves.push(Castle(63, 61, 60, 62, true));
                }
            }
            moves
        }

        pub fn check_attack_castle_black(&self, moves: &Vec<MoveType>) -> (bool, bool) {
            let mut can_castle_left = true;
            let mut can_castle_right = true;

            for single_move in moves.iter() {
                match single_move {
                    Attack(_, _from, to, can_move, color) => {
                        if *color && *can_move {
                            if *to == 4 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 2 || *to == 3 {
                                can_castle_left = false;
                            } else if *to == 5 || *to == 6 {
                                can_castle_right = false;
                            }
                        }
                    },
                    Promotion(_from, to, _, color) => {
                        if *color {
                            if *to == 4 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 2 || *to == 3 {
                                can_castle_left = false;
                            } else if *to == 5 || *to == 6 {
                                can_castle_right = false;
                            }
                        }
                    },
                    Capture(_, from, to, cp, color) => {
                        if *color {
                            if *to == 4 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 2 || *to == 3 {
                                can_castle_left = false;
                            } else if *to == 5 || *to == 6 {
                                can_castle_right = false;
                            }
                        }
                    }
                    _ => continue
                }
            }

            (can_castle_left,can_castle_right)
        }

        pub fn  castle_black(&self, all_moves: &Vec<MoveType>) -> Vec<MoveType> {
            let mut moves = vec![];


            let (can_left, can_right) = self.check_attack_castle_black(all_moves);

            // Check spaces between king and rook left are empty
            if can_left {
                if !Board::get_board_state_from_position(&self.board, &1) &&
                    !Board::get_board_state_from_position(&self.board, &2) &&
                    !Board::get_board_state_from_position(&self.board, &3) {
                    moves.push(Castle(0, 3, 4, 2, false));
                }
            }

            // Check spaces between king and rook right are empty
            if can_right {
                if !Board::get_board_state_from_position(&self.board, &5) &&
                    !Board::get_board_state_from_position(&self.board, &6) {
                    moves.push(Castle(7, 5, 4, 6, false));
                }
            }
            moves
        }

        pub fn get_all_moves(&self) -> (Vec<MoveType>, Vec<MoveType>) {
            let mut moves: Vec<MoveType> = vec![];
            let mut defence: Vec<MoveType> = vec![];
            moves.reserve(500);
            defence.reserve(100);


            for i in 0..64 {
                match self.board.board_state[i] {
                    Some(piece) => {
                        let (mut piece_moves, mut piece_defense_moves) =
                            piece.get_moves(&self.board, &(i as u8));
                        moves.append( &mut piece_moves);
                        defence.append( &mut piece_defense_moves);
                    },
                    None => ()
                }
            }

            // check for castelling
            if self.is_white_turn {
                match self.board.board_state[60] {
                    Some(piece) => {
                        if piece.piece_type == King && piece.is_white == true {
                            let mut white_castle = self.castle_white(&moves);
                            if white_castle.len() > 0 {
                                moves.append(&mut white_castle);
                            }
                        }
                    },
                    None => ()
                }
            } else {
                match self.board.board_state[4] {
                    Some(piece) => {
                        if piece.piece_type == King && piece.is_white == false {
                            let mut black_castle = self.castle_black(&moves);
                            if black_castle.len() > 0 {
                                moves.append(&mut black_castle);
                            }
                        }
                    },
                    None => ()
                }
            }
            (moves, defence)
        }

        pub fn evaluate_board(&self, moves: &Vec<MoveType>, defence: &Vec<MoveType>) -> i32 {
            let mut score: i32 = 0;
            for i in 0..64 {
                match self.board.board_state[i] {
                    Some(piece) => {
                        if piece.is_white {
                            score += i32::from(piece.get_value());
                        } else {
                            score -= i32::from(piece.get_value());
                        }
                    },
                    None => ()
                }
            }

            for m in moves {
                match m {
                    Promotion(_from, _to, _, color) => {
                        if *color {
                            score += 50;
                        } else {
                            score -= 50;
                        }
                    },
                    FutureMove(_p, _from, _to, color) => {
                        if *color {
                            score += 25;
                        } else {
                            score -= 25;
                        }
                    },
                    Attack(_p, _from, _to, can_move, color) => {
                        if *can_move {
                            if *color {
                                score += 5;
                            } else {
                                score -= 5;
                            }
                        }
                    },
                    Capture(p, cfrom, cto, cp, color) => {
                        let mut count = 0;
                        for d in defence {
                            match d {
                                Defend(_p, dfrom, dto, _d, _color) => {
                                    if cto == dfrom {
                                        count += 1;
                                    }
                                },
                                _ => ()

                            }
                        }

                        if count > 0 {
                            if *color {
                                score += i32::abs(i32::from((*p).get_value()) - i32::from((*cp).get_value()));
                            } else {
                                score -= i32::abs(i32::from((*p).get_value()) - i32::from((*cp).get_value()));
                            }
                        }
                    }
                    _ => ()
                }
            }

            score
        }

        pub fn make_move(&mut self, chosen_move: &MoveType) -> bool {
            //println!("movetype: {}", *chosen_move);
            match chosen_move {
                Standard(from, to, _color) => {
                    //let val = usize::from(*from);
                    //println!("board: {}", self.board.board_state.get(val).unwrap().unwrap());
                    self.board.make_move(usize::from(*from), usize::from(*to));
                    self.is_white_turn = !self.is_white_turn;
                    return true;
                },
                FutureMove(_p, _from, _to, _color) => {
                    panic!("Dont do future movedvdslkfgsdæljfkgædsj");
                },
                Promotion(from, to, piece, color) => {
                    self.board.make_move(usize::from(*from), usize::from(*to));
                    self.board.board_state[usize::from(*to)] = Some(Piece {piece_type: *piece, is_white: *color});
                    self.is_white_turn = !self.is_white_turn;
                    return true;
                },
                Castle(king_from, king_to, rook_from, rook_to, _color) => {
                    self.board.make_move(usize::from(*king_from), usize::from(*king_to));
                    self.board.make_move(usize::from(*rook_from), usize::from(*rook_to));
                    self.is_white_turn = !self.is_white_turn;
                    return true;
                }
                Attack(_p, from, to, can_move, _color) => {
                    if *can_move {
                        self.board.make_move(usize::from(*from), usize::from(*to));
                        self.is_white_turn = !self.is_white_turn;
                        return true;
                    }
                    panic!("Piece cannot move there.")
                },
                Capture(_p, from, to, _cp, _color) => {
                    self.board.make_move(usize::from(*from), usize::from(*to));
                    self.is_white_turn = !self.is_white_turn;
                    return true;
                },
                Defend(_p, _from, _to, _d, _color) => {
                    panic!("Dont do defend movedvdslkfgsdæljfkgædsj");
                },
            }
        }

    }

}