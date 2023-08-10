pub mod board {
    use std::cmp::Ordering;
    use std::{fmt, u8};
    use std::fmt::{Debug, Formatter};

    use crate::piece::piece::{Piece};
    use crate::game::game::{Game};

    pub enum MoveType {
        Standard(u8), // move(square to move to)
        FutureMove(u8), // maybe a capture type?
        Castle(u8, u8, u8, u8), // castle(king start, king end, rook start, rook end)
        Promotion(u8, Piece) // promotion(square to move to, piece to promote to)
    }

    impl Debug for MoveType {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                MoveType::Standard(val) => {
                    write!(f, "Standard({})", val)
                },
                MoveType::FutureMove(val) => {
                    write!(f, "FutureMove({})", val)
                },
                MoveType::Promotion(val, piece) => {
                    write!(f, "Promotion({}, {})", val, piece)
                },
                MoveType::Castle(king_start, king_end, rook_start, rook_end) => {
                    write!(f, "Castle({}, {}, {}, {})", king_start, king_end, rook_start, rook_end)
                }
            }
        }
    }

    impl fmt::Display for MoveType {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                MoveType::Standard(val) => {
                    write!(f, "Standard({})", val)
                },
                MoveType::FutureMove(val) => {
                    write!(f, "FutureMove({})", val)
                },
                MoveType::Promotion(val, piece) => {
                    write!(f, "Promotion({}, {})", val, piece)
                },
                MoveType::Castle(king_start, king_end, rook_start, rook_end) => {
                    write!(f, "Castle({}, {}, {}, {})", king_start, king_end, rook_start, rook_end)
                }
            }
        }
    }

    impl Eq for MoveType {

    }

    impl PartialEq<Self> for MoveType {
        fn eq(&self, other: &Self) -> bool {
            match self {
                MoveType::Standard(val) => {
                    match other {
                        MoveType::Standard(val2) => {
                            return val == val2;
                        },
                        _ => {
                            return false;
                        }
                    }
                },
                MoveType::FutureMove(val) => {
                    match other {
                        MoveType::FutureMove(val2) => {
                            return val == val2;
                        },
                        _ => {
                            return false;
                        }
                    }
                },
                _ => return false
            }
        }
    }

    impl PartialOrd<Self> for MoveType {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self {
                MoveType::Standard(val) => {
                    match other {
                        MoveType::Standard(val2) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::FutureMove(val2) => {
                            return val.partial_cmp(val2);
                        },
                        _ => {
                            return None;
                        }
                    }
                },
                MoveType::FutureMove(val) => {
                    match other {
                        MoveType::Standard(val2) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::FutureMove(val2) => {
                            return val.partial_cmp(val2);
                        },
                        _ => {
                            return None;
                        }
                    }
                },
                _ => return None
            }
        }
    }

    impl Ord for MoveType {
        fn cmp(&self, other: &Self) -> Ordering {
            match self {
                MoveType::Standard(val) => {
                    match other {
                        MoveType::Standard(val2) => {
                            return val.cmp(val2);
                        },
                        MoveType::FutureMove(val2) => {
                            return val.cmp(val2);
                        },
                        _ => {
                            return Ordering::Less;
                        }
                    }
                },
                MoveType::FutureMove(val) => {
                    match other {
                        MoveType::Standard(val2) => {
                            return val.cmp(val2);
                        },
                        MoveType::FutureMove(val2) => {
                            return val.cmp(val2);
                        },
                        _ => {
                            return Ordering::Less;
                        }
                    }
                },
                _ => return Ordering::Less
            }
        }
    }

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

            return Game {
                board: game.board,
                is_white_turn: true,
                white_king_moved: false,
                black_king_moved: false,
                white_rook_left_moved: false,
                white_rook_right_moved: false,
                black_rook_left_moved: false,
                black_rook_right_moved: false,
            };
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
