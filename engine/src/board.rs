pub mod board {
    use std::cmp::Ordering;
    use PieceType::KING;
    use crate::board::board::Move::{Capture, Castle, Promotion, Standard};
    use crate::board::board::Side::{Left, Right};
    use crate::move_gen::move_gen::{MoveGen, PieceType};
    use crate::move_gen::move_gen::PieceType::{BISHOP, KNIGHT, PAWN, QUEEN, ROOK};

    // 1: pawn
    // 2: knight
    #[derive(Debug, Clone, Copy)]
    pub struct BoardMove {
        attack_board: u64,
        piece_type: PieceType,
        position: u8,
        white: bool,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Side {
        Left,
        Right
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Move {
        Standard(u8, u8, PieceType), // position, to, piecetype
        Capture(u8, u8, PieceType, PieceType),// from, to, moving piece, captured piece
        Promotion(u8, u8, PieceType), // from, to, piece to promote too
        Castle(u8, Side) // king position, side to castle
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
                        Capture(from_, to_, piece_, captured_piece) => {
                            ordering(from, to, from_, to_)
                        }
                        Castle(from_, side) => {
                            Ordering::Less
                        }
                    }
                },
                Promotion(from, to, piece) => {
                    match other {
                        Standard(from_, to_, _) => {
                            return ordering(from, to, from_, to_)
                        },
                        Promotion(from_, to_, piece_) => {
                            return ordering(from, to, from_, to_)
                        }
                        Capture(from_, to_, piece_, captured_piece) => {
                            return ordering(from, to, from_, to_)
                        }
                        Castle(from_, side) => {
                            return Ordering::Less
                        }
                    }
                }
                Capture(from, to, piece, captured_piece) => {
                    match other {
                        Standard(from_, to_, _) => {
                            return ordering(from, to, from_, to_)
                        },
                        Promotion(from_, to_, piece_) => {
                            return ordering(from, to, from_, to_)
                        }
                        Capture(from_, to_, piece_, captured_piece) => {
                            return ordering(from, to, from_, to_)
                        }
                        Castle(from_, side) => {
                            return Ordering::Less
                        }
                    }
                }
                Castle(from, Side) => {
                    match other {
                        Standard(from_, to_, _) => {
                            return Ordering::Greater
                        },
                        Promotion(from_, to_, piece_) => {
                            return Ordering::Greater
                        }
                        Capture(from_, to_, piece_, captured_piece) => {
                            return Ordering::Greater
                        }
                        Castle(from_, side) => {
                            if side == &Right {
                                return Ordering::Greater
                            } else {
                                return Ordering::Less
                            }
                        }
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
                Capture(from, to, piece, captured_piece) => {
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

        pub fn attack_boards_to_moves(&self, move_boards: &Vec<BoardMove>, is_white: bool) -> Vec<Move> {
            let mut moves: Vec<Move> = Vec::with_capacity(50);

            for i in 0..64 {
                for x in move_boards {
                    let position = 1 << i;

                    let promotion_rank = if is_white { i < 8} else {i > 56};

                    if x.piece_type == PAWN && promotion_rank && (position & x.attack_board) > 0 {
                        moves.push(Promotion(x.position, i, KNIGHT));
                        moves.push(Promotion(x.position, i, BISHOP));
                        moves.push(Promotion(x.position, i, QUEEN));
                        moves.push(Promotion(x.position, i, ROOK))
                    }
                    else if x.piece_type == KNIGHT {
                        if (position & x.attack_board) > 0 {
                            let m = Standard(x.position, i, x.piece_type);
                            moves.push(m);
                        }
                    } else {
                        if (position & x.attack_board) > 0 {
                            let m = Standard(x.position, i, x.piece_type);
                            moves.push(m);
                        }
                    }
                }
            }

            return moves;
        }

        pub fn get_all_moves(&self, pieces: &MoveGen, is_white: bool) -> Vec<Move> {
            let occupancy = self.get_board_value();
            let white_occupancy = self.get_white_occupancy();
            let black_occupancy = self.get_black_occupancy();

            let mut white_moves_boards: Vec<BoardMove> = self.get_white_moves(white_occupancy, black_occupancy, occupancy, pieces);

            //let white_attack_board = Self::get_attack_board(&white_moves_boards);

            let mut black_moves_boards: Vec<BoardMove> = self.get_black_moves(white_occupancy, black_occupancy, occupancy, pieces);

            //let black_attack_board = Self::get_attack_board(&black_moves_boards);

            let moves;

            if is_white {
                moves = self.attack_boards_to_moves(&white_moves_boards, true);
            } else {
                moves = self.attack_boards_to_moves(&black_moves_boards, false);
            }


            return moves
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
                }
            };
        }


        pub fn get_white_moves(&self, white_occupancy: u64, black_occupancy: u64, occupancy: u64, pieces: &MoveGen) -> Vec<BoardMove> {
            let mut moves: Vec<BoardMove> = Vec::with_capacity(16);

            for i in 0..64 {
                if (self.white_knight_board & 1 << i) > 0 {
                    let reverse_board: u64 = pieces.knight_position_board[i] & !white_occupancy;
                    let b = BoardMove {
                        attack_board: reverse_board,
                        piece_type: KNIGHT,
                        position: u8::try_from(i).unwrap(),
                        white: true,
                    };
                    moves.push(b);
                    continue
                }

                if (self.white_rook_board & 1 << i) > 0 {
                    let b = BoardMove {
                        attack_board: pieces.get_move(ROOK, i, white_occupancy, occupancy),
                        piece_type: ROOK,
                        position: u8::try_from(i).unwrap(),
                        white: true,
                    };
                    moves.push(b);
                    continue
                }

                if (self.white_bishop_board & 1 << i) > 0 {
                    let b = BoardMove {
                        attack_board: pieces.get_move(BISHOP, i, white_occupancy, occupancy),
                        piece_type: BISHOP,
                        position: u8::try_from(i).unwrap(),
                        white: true,
                    };
                    moves.push(b);
                    continue
                }

                if (self.white_queen_board & 1 << i) > 0 {
                    let b = BoardMove {
                        attack_board: pieces.get_move(QUEEN, i, white_occupancy, occupancy),
                        piece_type: QUEEN,
                        position: u8::try_from(i).unwrap(),
                        white: true,
                    };
                    moves.push(b);
                    continue
                }

                if (self.white_pawn_board & 1 << i) > 0 {
                    let b = BoardMove {
                        attack_board: pieces.calculate_white_pawn_move(u64::try_from(i).unwrap(), occupancy, black_occupancy),
                        piece_type: PAWN,
                        position: u8::try_from(i).unwrap(),
                        white: true,
                    };
                    moves.push(b);
                    continue
                }
            }

            return moves;
        }

        pub fn get_black_moves(&self, white_occupancy: u64, black_occupancy: u64, occupancy: u64, pieces: &MoveGen) -> Vec<BoardMove> {
            let mut moves: Vec<BoardMove> = Vec::with_capacity(16);
            for i in 0..64 {
                if (self.black_knight_board & 1 << i) > 0 {
                    let reverse_board: u64 = (0xFFFF_FFFF_FFFF_FFFF ^ black_occupancy) & pieces.knight_position_board[i];
                    let b = BoardMove {
                        attack_board: reverse_board,
                        piece_type: KNIGHT,
                        position: u8::try_from(i).unwrap(),
                        white: false,
                    };
                    moves.push(b);
                    continue
                }

                if (self.black_rook_board & 1 << i) > 0 {
                    let b = BoardMove {
                        attack_board: pieces.get_move(ROOK, i, black_occupancy, occupancy),
                        piece_type: ROOK,
                        position: u8::try_from(i).unwrap(),
                        white: false,
                    };
                    moves.push(b);
                    continue
                }

                if (self.black_bishop_board & 1 << i) > 0 {
                    let b = BoardMove {
                        attack_board: pieces.get_move(BISHOP, i, black_occupancy, occupancy),
                        piece_type: BISHOP,
                        position: u8::try_from(i).unwrap(),
                        white: false,
                    };
                    moves.push(b);
                    continue
                }

                if (self.black_queen_board & 1 << i) > 0 {
                    let b = BoardMove {
                        attack_board: pieces.get_move(QUEEN, i, black_occupancy, occupancy),
                        piece_type: QUEEN,
                        position: u8::try_from(i).unwrap(),
                        white: false,
                    };
                    moves.push(b);
                    continue
                }

                if (self.black_pawn_board & 1 << i) > 0 {
                    let b = BoardMove {
                        attack_board: pieces.calculate_black_pawn_move(u64::try_from(i).unwrap(), occupancy, white_occupancy),
                        piece_type: PAWN,
                        position: u8::try_from(i).unwrap(),
                        white: false,
                    };
                    moves.push(b);
                    continue
                }
            }

            return moves;
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
