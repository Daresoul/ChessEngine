pub mod board {
    use std::cmp::Ordering;
    use PieceType::KING;
    use crate::board::board::Move::{Capture, Castle, Promotion, Standard};
    use crate::board::board::Side::{Left, Right};
    use crate::move_gen::move_gen::{MoveGen, PieceType};
    use crate::move_gen::move_gen::PieceType::{BISHOP, KNIGHT, PAWN, QUEEN, ROOK};
    use crate::move_list::move_list::{AttackMoveList, MoveList};

    // 1: pawn
    // 2: knight
    #[derive(Debug, Clone, Copy)]
    pub struct BoardMove {
        pub(crate) attack_board: u64,
        pub(crate) piece_type: PieceType,
        pub(crate) position: u8,
        pub(crate) white: bool,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Side {
        Left,
        Right
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(u8)]
    pub enum Move {
        None = 0,
        Standard(u8, u8, PieceType), // position, to, piecetype
        Capture(u8, u8, PieceType, PieceType),// from, to, moving piece, captured piece
        Promotion(u8, u8, PieceType), // from, to, piece to promote too
        Castle(u8, Side), // king position, side to castle
    }

    impl Eq for Move {}

    impl PartialOrd<Self> for Move {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    fn ordering(pos: &u8, to: &u8, opos: &u8, oto: &u8) -> Ordering {
        if pos > opos {
            return Ordering::Greater;
        } else if pos < opos {
            return Ordering::Less;
        }
        else {
            if to > oto {
                return Ordering::Greater
            } else if to < oto {
                return Ordering::Less
            }

            return Ordering::Equal
        }
    }

    impl Ord for Move {
        fn cmp(&self, other: &Self) -> Ordering {
            match self {
                Standard(from, to, _) => {
                    return match other {
                        Standard(from_, to_, _) => {
                            ordering(from, to, from_, to_)
                        },
                        Promotion(from_, to_, _) => {
                            ordering(from, to, from_, to_)
                        },
                        Capture(from_, to_, _piece_, _captured_piece) => {
                            ordering(from, to, from_, to_)
                        }
                        Castle(_from_, _side) => {
                            Ordering::Less
                        },
                        Move::None => Ordering::Greater
                    }
                },
                Promotion(from, to, _piece) => {
                    match other {
                        Standard(from_, to_, _) => {
                            return ordering(from, to, from_, to_)
                        },
                        Promotion(from_, to_, _piece_) => {
                            return ordering(from, to, from_, to_)
                        }
                        Capture(from_, to_, _piece_, _captured_piece) => {
                            return ordering(from, to, from_, to_)
                        }
                        Castle(_from_, _side) => {
                            return Ordering::Less
                        },
                        Move::None => Ordering::Greater
                    }
                }
                Capture(from, to, _piece, _captured_piece) => {
                    match other {
                        Standard(from_, to_, _) => {
                            return ordering(from, to, from_, to_)
                        },
                        Promotion(from_, to_, _piece_) => {
                            return ordering(from, to, from_, to_)
                        }
                        Capture(from_, to_, _piece_, _captured_piece) => {
                            return ordering(from, to, from_, to_)
                        }
                        Castle(_from_, _side) => {
                            return Ordering::Less
                        },
                        Move::None => Ordering::Greater
                    }
                }
                Castle(_from, _Side) => {
                    match other {
                        Standard(_from_, _to_, _) => {
                            return Ordering::Greater
                        },
                        Promotion(_from_, _to_, _piece_) => {
                            return Ordering::Greater
                        }
                        Capture(_from_, _to_, _piece_, _captured_piece) => {
                            return Ordering::Greater
                        }
                        Castle(_from_, side) => {
                            if side == &Right {
                                return Ordering::Greater
                            } else {
                                return Ordering::Less
                            }
                        },
                        Move::None => Ordering::Greater
                    }
                },
                Move::None => {
                    match other {
                        Move::None => Ordering::Equal,
                        _ => Ordering::Less
                    }
                }
            }
        }
    }

    impl PartialEq for &Side {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Right => {
                    match other {
                        Right => true,
                        _ => false
                    }
                },
                Left => {
                    match other {
                        Left => true,
                        _ => false
                    }
                }
            }
        }
    }

    impl PartialEq<Self> for Move {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Standard(from, to, piece) => {
                    match other {
                        Standard(from_, to_, piece_) => {
                            to == to_ && from == from_ && piece == piece_
                        },
                        _ => false
                    }
                }
                Promotion(from, to, piece) => {
                    match other {
                        Promotion(from_, to_, piece_) => {
                            to == to_ && from == from_ && piece == piece_
                        },
                        _ => false
                    }
                },
                Capture(from, to, piece, _captured_piece) => {
                    match other {
                        Capture(_from, _to, _piece, _captured_piece) => {
                            to == _to && from == _from && piece == _piece
                        },
                        _ => false
                    }
                },
                Castle(from, side) => {
                    match other {
                        Castle(_from, _side) => {
                            _from == from && _side == side
                        },
                        _ => false
                    }
                }
                Move::None => {
                    match other {
                        Move::None => true,
                        _ => false
                    }
                }
            }
        }
    }


    #[derive(Debug, Clone, Copy)]
    pub struct Board {
        pub white_pawn_board: u64,
        pub white_king_board: u64,
        pub white_queen_board: u64,
        pub white_rook_board: u64,
        pub white_bishop_board: u64,
        pub white_knight_board: u64,
        pub black_pawn_board: u64,
        pub black_king_board: u64,
        pub black_queen_board: u64,
        pub black_rook_board: u64,
        pub black_bishop_board: u64,
        pub black_knight_board: u64,
        pub white_king_position: u8,
        pub black_king_position: u8,
    }

    impl Board {
        pub fn new() -> Board {
            return Board {
                white_pawn_board: 65_280,
                black_pawn_board: 71_776_119_061_217_280,
                white_queen_board: 16,
                black_queen_board: 1_152_921_504_606_846_976,
                white_king_board: 8,
                black_king_board: 576_460_752_303_423_488,
                white_rook_board: 129,
                black_rook_board: 2_269_391_999_729_700,
                white_bishop_board: 36,
                black_bishop_board: 2_594_073_385_365_405_696,
                white_knight_board: 66,
                black_knight_board: 4_755_801_206_503_243_776,
                black_king_position: 4,
                white_king_position: 60,
            };
        }

        pub fn from_string(notation: String) -> Board {
            let mut board =  Board {
                white_pawn_board: 0,
                black_pawn_board: 0,
                white_queen_board: 0,
                black_queen_board: 0,
                white_king_board: 0,
                black_king_board: 0,
                white_rook_board: 0,
                black_rook_board: 0,
                white_bishop_board: 0,
                black_bishop_board: 0,
                white_knight_board: 0,
                black_knight_board: 0,
                black_king_position: 0,
                white_king_position: 0
            };

            Board::transform_string_to_state(&mut board, notation);

            return board
        }

        pub fn get_board_value(&self) -> u64 {
            return self.white_king_board |
                self.white_queen_board |
                self.white_knight_board |
                self.white_rook_board |
                self.white_bishop_board |

                self.black_king_board |
                self.black_queen_board |
                self.black_knight_board |
                self.black_rook_board |
                self.black_bishop_board |

                self.black_pawn_board |
                self.white_pawn_board;
        }

        pub fn get_black_occupancy(&self) -> u64 {
            return self.black_king_board |
                self.black_queen_board |
                self.black_knight_board |
                self.black_rook_board |
                self.black_bishop_board |
                self.black_pawn_board;
        }
        pub fn get_white_occupancy(&self) -> u64 {
            return self.white_king_board |
                self.white_queen_board |
                self.white_knight_board |
                self.white_rook_board |
                self.white_bishop_board |
                self.white_pawn_board;
        }

        // Forsyth–Edwards Notation
        pub fn transform_string_to_state(board: &mut Board, state: String) -> () {
            let mut board_index: u8 = 0;

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
                    'P' => {board.white_pawn_board = board.white_pawn_board | 1 << board_index; board_index += 1},
                    'p' => {board.black_pawn_board = board.black_pawn_board | 1 << board_index; board_index += 1},
                    'R' => {board.white_rook_board = board.white_rook_board | 1 << board_index; board_index += 1},
                    'r' => {board.black_rook_board = board.black_rook_board | 1 << board_index; board_index += 1},
                    'N' => {board.white_knight_board = board.white_knight_board | 1 << board_index; board_index += 1},
                    'n' => {board.black_knight_board = board.black_knight_board | 1 << board_index; board_index += 1},
                    'B' => {board.white_bishop_board = board.white_bishop_board | 1 << board_index; board_index += 1},
                    'b' => {board.black_bishop_board = board.black_bishop_board | 1 << board_index; board_index += 1},
                    'Q' => {board.white_queen_board = board.white_queen_board | 1 << board_index; board_index += 1},
                    'q' => {board.black_queen_board = board.black_queen_board | 1 << board_index; board_index += 1},
                    'K' => {
                        board.white_king_board = board.white_king_board | 1 << board_index;
                        board.white_king_position = board_index;
                        board_index += 1
                    },
                    'k' => {
                        board.black_king_board = board.black_king_board | 1 << board_index;
                        board.black_king_position = board_index;
                        board_index += 1
                    },
                    _ => continue
                };
            }
        }

        fn get_attack_board(boards: &Vec<BoardMove>) -> u64 {
            let mut attackBoard = 0;
            for b in boards {
                attackBoard |= b.attack_board;
            }

            return attackBoard;
        }

        pub fn attack_boards_to_moves(&self, move_boards: &AttackMoveList, moves_arr: &mut MoveList, is_white: bool) {
            for x in move_boards.iter() {
                let bits = x.attack_board.count_ones();
                let mut attack_board = x.attack_board;
                for _ in 0..bits {
                    let index = Self::pop_lsb(&mut attack_board);

                    if x.piece_type == PAWN {
                        let promotion_rank = if is_white { index < 8} else {index > 56};

                        if promotion_rank {
                            moves_arr.add(Promotion(x.position, index as u8, KNIGHT));
                            moves_arr.add(Promotion(x.position, index as u8, BISHOP));
                            moves_arr.add(Promotion(x.position, index as u8, QUEEN));
                            moves_arr.add(Promotion(x.position, index as u8, ROOK));
                        } else {
                            moves_arr.add(Standard(x.position, index as u8, PAWN));
                        }
                    } else {
                        moves_arr.add(Standard(x.position, index as u8, PAWN));
                    }
                }
            }
        }

        pub fn get_captured_board(&self, pos: &u8, is_white: bool) -> Option<PieceType> {
            let position: u64 = 1 << pos;
            match is_white {
                false => {
                    if self.white_pawn_board & position > 0 {
                        return Some(PAWN);
                    }

                    if self.white_queen_board & position > 0 {
                        return Some(QUEEN);
                    }

                    if self.white_knight_board & position > 0 {
                        return Some(KNIGHT);
                    }

                    if self.white_bishop_board & position > 0 {
                        return Some(BISHOP);
                    }

                    if self.white_rook_board & position > 0 {
                        return Some(ROOK);
                    }

                    return None
                },
                true => {
                    if self.black_pawn_board & position > 0 {
                        return Some(PAWN);
                    }

                    if self.black_queen_board & position > 0 {
                        return Some(QUEEN);
                    }

                    if self.black_knight_board & position > 0 {
                        return Some(KNIGHT);
                    }

                    if self.black_bishop_board & position > 0 {
                        return Some(BISHOP);
                    }

                    if self.black_rook_board & position > 0 {
                        return Some(ROOK);
                    }

                    return None
                }
            }
        }

        pub fn movePiece(board: &mut u64, from: &u8, to: &u8) -> () {
            let to_position: u64 = 1 << to;
            let from_position: u64 = 1 << from;
            *board = *board | to_position;
            *board ^= from_position;
        }

        pub fn remove_piece_if_taken(&mut self, to: &u8, is_white: bool) -> () {
            let to_position: u64 = 1 << to;
            match self.get_captured_board(to, is_white) {
                Some(piece) => {
                    match piece {
                        PAWN => { if is_white { self.black_pawn_board = self.black_pawn_board ^ to_position }
                        else { self.white_pawn_board = self.white_pawn_board ^ to_position } }

                        ROOK => { if is_white { self.black_rook_board = self.black_rook_board ^ to_position }
                        else { self.white_rook_board = self.white_rook_board ^ to_position } }

                        QUEEN => { if is_white { self.black_queen_board = self.black_queen_board ^ to_position }
                        else { self.white_queen_board = self.white_queen_board ^ to_position } }

                        BISHOP => { if is_white { self.black_bishop_board = self.black_bishop_board ^ to_position }
                        else { self.white_bishop_board = self.white_bishop_board ^ to_position } }

                        KNIGHT => { if is_white { self.black_knight_board = self.black_knight_board ^ to_position }
                        else { self.white_knight_board = self.white_knight_board ^ to_position } }

                        _ => panic!("Umm, a captured king somehow????")
                    }
                },
                None => ()
            }
        }

        pub fn make_move(&mut self, p: &PieceType, from: &u8, to: &u8, is_white: bool) -> () {
            self.remove_piece_if_taken(to, is_white);
            match p {
                PAWN => {
                    let board = if is_white {&mut self.white_pawn_board} else {&mut self.black_pawn_board};
                    Self::movePiece(board, from, to);
                },
                KING => {
                    let board = if is_white {&mut self.white_king_board} else {&mut self.black_king_board};
                    Self::movePiece(board, from, to);
                },
                QUEEN => {
                    let board = if is_white {&mut self.white_queen_board} else {&mut self.black_queen_board};
                    Self::movePiece(board, from, to);
                },
                ROOK => {
                    let board = if is_white {&mut self.white_rook_board} else {&mut self.black_rook_board};
                    Self::movePiece(board, from, to);
                },
                BISHOP => {
                    let board = if is_white {&mut self.white_bishop_board} else {&mut self.black_bishop_board};
                    Self::movePiece(board, from, to);
                },
                KNIGHT => {
                    let board = if is_white {&mut self.white_knight_board} else {&mut self.black_knight_board};
                    Self::movePiece(board, from, to);
                },
                PieceType::None => panic!("Cant make a move on a None")
            };
        }


        pub fn pop_lsb(mask: &mut u64) -> usize {
            let bit_pos = mask.trailing_zeros();
            *mask &= *mask - 1;
            bit_pos as usize
        }

        pub fn get_moves(&self, team_occupancy: u64, opponent_occupancy: u64, occupancy: u64, is_white: bool, moves_array: &mut AttackMoveList, move_gen: &MoveGen) {
            let mut white_knight_board = if is_white {self.white_knight_board} else {self.black_knight_board};
            let mut white_rook_board = if is_white {self.white_rook_board} else {self.black_rook_board};
            let mut white_bishop_board = if is_white {self.white_bishop_board} else {self.black_bishop_board};
            let mut white_queen_board = if is_white {self.white_queen_board} else {self.black_queen_board};
            let mut white_pawn_board = if is_white {self.white_pawn_board} else {self.black_pawn_board};

            for _ in 0..(white_knight_board.count_ones() as usize) {
                let lsb = Self::pop_lsb(&mut white_knight_board);
                let b = BoardMove {
                    attack_board: move_gen.get_move(KNIGHT, lsb, team_occupancy, occupancy),
                    piece_type: KNIGHT,
                    position: u8::try_from(lsb).unwrap(),
                    white: true,
                };
                moves_array.add(b);
            }

            for _ in 0..(white_rook_board.count_ones() as usize) {
                let lsb = Self::pop_lsb(&mut white_rook_board);
                let b = BoardMove {
                    attack_board: move_gen.get_move(ROOK, lsb, team_occupancy, occupancy),
                    piece_type: ROOK,
                    position: u8::try_from(lsb).unwrap(),
                    white: true,
                };
                moves_array.add(b);
            }

            for _ in 0..(white_bishop_board.count_ones() as usize) {
                let lsb = Self::pop_lsb(&mut white_bishop_board);
                let b = BoardMove {
                    attack_board: move_gen.get_move(BISHOP, lsb, team_occupancy, occupancy),
                    piece_type: BISHOP,
                    position: u8::try_from(lsb).unwrap(),
                    white: true,
                };
                moves_array.add(b);
            }

            for _ in 0..(white_queen_board.count_ones() as usize) {
                let lsb = Self::pop_lsb(&mut white_queen_board);
                let b = BoardMove {
                    attack_board: move_gen.get_move(QUEEN, lsb, team_occupancy, occupancy),
                    piece_type: QUEEN,
                    position: u8::try_from(lsb).unwrap(),
                    white: true,
                };
                moves_array.add(b);
            }

            for _ in 0..(white_pawn_board.count_ones() as usize) {
                let lsb = Self::pop_lsb(&mut white_pawn_board);
                let b = BoardMove {
                    attack_board: if is_white {move_gen.calculate_white_pawn_move(lsb, occupancy, opponent_occupancy)} else {move_gen.calculate_black_pawn_move(lsb, occupancy, opponent_occupancy)},
                    piece_type: PAWN,
                    position: u8::try_from(lsb).unwrap(),
                    white: true,
                };
                moves_array.add(b)
            }
        }

        // TODO: Recreate
        pub fn compute_hash(&self) -> u64 {
            return 0
        }

        // TODO: recreate
        /*pub fn make_move(&mut self, from: usize, to: usize, m: &MoveType) -> () {
            let current_option_piece = self.board_state[from];
            match current_option_piece {
                Some(piece) => {
                    self.board_value = self.board_value ^ 1 << from;
                    self.board_value = self.board_value | 1 << to;
                    self.board_state[from] = None;
                    self.board_state[to] = Some(piece);
                },
                None => {
                    debug::debug::print_board_board(self);
                    panic!("No piece at position {}, {}, {}", from, to, m);
                }
            }
        }*/

    }
}
