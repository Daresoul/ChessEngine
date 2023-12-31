pub mod board {
    use std::cmp::Ordering;
    use std::{fmt, u8};
    use std::fmt::{Debug, Formatter};
    use crate::debug;
    use crate::debug_structs::debug_structs::get_debug_pawn_board;

    use crate::piece::piece::{Piece, PieceType};
    use crate::game::game::{Game};

    #[derive(Clone, Copy)]
    pub enum MoveType {
        Standard(u8, u8, bool), // move(square to move to, color of piece)
        Capture(PieceType, u8, u8, PieceType, bool), // capture(square to move to, piece captured, color of piece)
        Attack(PieceType, u8, u8, bool, bool),
        FutureMove(PieceType, u8, u8, bool), // FutureMove(square to move to, color of piece)
        Castle(u8, u8, u8, u8, bool), // castle(king start, king end, rook start, rook end, color of piece)
        Promotion(u8, u8, PieceType, bool), // promotion(square to move to, piece to promote to, color of piece)
        Defend(PieceType, u8, u8, PieceType, bool), // defend(square to move to, piece to defend, color of piece)
    }

    impl Debug for MoveType {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                MoveType::Standard(from, val, color) => {
                    write!(f, "Standard({}, {}, {})",from, val, color)
                },
                MoveType::FutureMove(p, from, val, color) => {
                    write!(f, "FutureMove({}, {}, {}, {})", p, from, val, color)
                },
                MoveType::Promotion(from, val, piece, color) => {
                    write!(f, "Promotion({}, {}, {}, {})",from, val, piece, color)
                },
                MoveType::Castle(king_start, king_end, rook_start, rook_end, color) => {
                    write!(f, "Castle({}, {}, {}, {}, {})", king_start, king_end, rook_start, rook_end, color)
                },
                MoveType::Attack(p, from, val, _can_move, color) => {
                    write!(f, "Attack({}, {}, {}, {})", p, from, val, color)
                },
                MoveType::Capture(p, from, val, cp, color) => {
                    write!(f, "Capture({}, {}, {}, {}, {})", p, from, val, cp, color)
                },
                MoveType::Defend(p, from, val, d, color) => {
                    write!(f, "Defend({}, {}, {}, {}, {})", p, from, val, d, color)
                }
            }
        }
    }

    impl fmt::Display for MoveType {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                MoveType::Standard(from, val, color) => {
                    write!(f, "Standard({}, {}, {})",from, val, color)
                },
                MoveType::FutureMove(p, from, val, color) => {
                    write!(f, "FutureMove({}, {}, {}, {})", p, from, val, color)
                },
                MoveType::Promotion(from, val, piece, color) => {
                    write!(f, "Promotion({}, {}, {}, {})",from, val, piece, color)
                },
                MoveType::Castle(king_start, king_end, rook_start, rook_end, color) => {
                    write!(f, "Castle({}, {}, {}, {}, {})", king_start, king_end, rook_start, rook_end, color)
                },
                MoveType::Attack(p, from, val, _can_move, color) => {
                    write!(f, "Attack({}, {}, {}, {})", p, from, val, color)
                },
                MoveType::Capture(p, from, val, cp, color) => {
                    write!(f, "Capture({}, {}, {}, {}, {})", p, from, val, cp, color)
                },
                MoveType::Defend(p, from, val, d, color) => {
                    write!(f, "Defend({}, {}, {}, {}, {})", p, from, val, d, color)
                }
            }
        }
    }

    impl Eq for MoveType {

    }

    impl PartialEq<Self> for MoveType {
        fn eq(&self, other: &Self) -> bool {
            match self {
                MoveType::Standard(_from, val, _) => {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return val == val2;
                        },
                        _ => {
                            return false;
                        }
                    }
                },
                MoveType::FutureMove(_p, _from, val, _) => {
                    match other {
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return val == val2;
                        },
                        _ => {
                            return false;
                        }
                    }
                },
                MoveType::Castle(king_start, king_end, rook_start, rook_end, _) => {
                    match other {
                        MoveType::Castle(king_start2, king_end2, rook_start2, rook_end2, _) => {
                            return king_start == king_start2 && king_end == king_end2 && rook_start == rook_start2 && rook_end == rook_end2;
                        },
                        _ => {
                            return false;
                        }
                    }
                },
                MoveType::Attack(p, _from, val, _can_move, _) => {
                    match other {
                        MoveType::Attack(p2, from, val2, _can_move, _) => {
                            return p == p2 && val == val2 && from == from;
                        },
                        _ => {
                            return false;
                        }
                    }
                },
                MoveType::Capture(p, _from, val, _cp, _) => {
                    match other {
                        MoveType::Capture(p2, from, val2, _cp, _) => {
                            return p == p2 && val == val2 && from == from;
                        },
                        _ => {
                            return false;
                        }
                    }
                },
                MoveType::Defend(p, _from, val, _d, _) => {
                    match other {
                        MoveType::Defend(p2, from, val2, _d, _) => {
                            return p == p2 && val == val2 && from == from;
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
                MoveType::Standard(_from, val, _) => {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Castle(rook_start, _, _, _, _) => {
                            return val.partial_cmp(rook_start);
                        },
                        MoveType::Attack(_p, _from, val2, _can_move, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Capture(_p, _from, val2, _cp, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Defend(_p, _from, val2, _d, _) => {
                            return val.partial_cmp(val2);
                        },
                        _ => {
                            return None;
                        }
                    }
                },
                MoveType::FutureMove(_p, _from, val, _) => {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Castle(rook_start, _, _, _, _) => {
                            return val.partial_cmp(rook_start);
                        },
                        MoveType::Attack(_p, _from, val2, _can_move, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Capture(_p, _from, val2, _cp, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Defend(_p, _from, val2, _d, _) => {
                            return val.partial_cmp(val2);
                        },
                        _ => {
                            return None;
                        }
                    }
                },
                MoveType::Castle(rook_start_, _, _, _, _) =>
                {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return rook_start_.partial_cmp(val2);
                        },
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return rook_start_.partial_cmp(val2);
                        },
                        MoveType::Castle(rook_start, _, _, _, _) => {
                            return rook_start_.partial_cmp(rook_start);
                        },
                        MoveType::Attack(_p, _from, val2, _can_move, _) => {
                            return rook_start_.partial_cmp(val2);
                        },
                        MoveType::Capture(_p, _from, val2, _cp, _) => {
                            return rook_start_.partial_cmp(val2);
                        },
                        MoveType::Defend(_p, _from, val2, _d, _) => {
                            return rook_start_.partial_cmp(val2);
                        },
                        _ => {
                            return None;
                        }
                    }
                },
                MoveType::Attack(_p, _from, val, _can_move, _) => {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Castle(rook_start, _, _, _, _) => {
                            return val.partial_cmp(rook_start);
                        },
                        MoveType::Attack(_p, _from, val2, _can_move, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Capture(_p, _from, val2, _cp, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Defend(_p, _from, val2, _d, _) => {
                            return val.partial_cmp(val2);
                        },
                        _ => {
                            return None;
                        }
                    }
                },
                MoveType::Capture(_p, _from, val, _cp, _) => {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Castle(rook_start, _, _, _, _) => {
                            return val.partial_cmp(rook_start);
                        },
                        MoveType::Attack(_p, _from, val2, _can_move, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Capture(_p, _from, val2, _cp, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Defend(_p, _from, val2, _d, _) => {
                            return val.partial_cmp(val2);
                        },
                        _ => {
                            return None;
                        }
                    }
                },
                MoveType::Defend(_p, _from, val, _d, _) => {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Castle(rook_start, _, _, _, _) => {
                            return val.partial_cmp(rook_start);
                        },
                        MoveType::Attack(_p, _from, val2, _can_move, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Capture(_p, _from, val2, _cp, _) => {
                            return val.partial_cmp(val2);
                        },
                        MoveType::Defend(_p, _from, val2, _d, _) => {
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
                MoveType::Standard(_from, val, _) => {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Castle(rook_start, _, _, _, _) => {
                            return val.cmp(rook_start);
                        },
                        MoveType::Attack(_p, _from, val2, _can_move, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Capture(_p, _from, val2, _cp, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Defend(_p, _from, val2, _d, _) => {
                            return val.cmp(val2);
                        },
                        _ => {
                            return Ordering::Less;
                        }
                    }
                },
                MoveType::FutureMove(_p, _from, val, _) => {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Castle(rook_start, _, _, _, _) => {
                            return val.cmp(rook_start);
                        },
                        MoveType::Attack(_p, _from, val2, _can_move, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Capture(_p, _from, val2, _cp, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Defend(_p, _from, val2, _d, _) => {
                            return val.cmp(val2);
                        },
                        _ => {
                            return Ordering::Less;
                        }
                    }
                },
                MoveType::Castle(rook_start, _, _, _, _) =>
                {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return rook_start.cmp(val2);
                        },
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return rook_start.cmp(val2);
                        },
                        MoveType::Castle(rook_start_second, _, _, _, _) => {
                            return rook_start.cmp(rook_start_second);
                        },
                        MoveType::Attack(_p, _from, val2, _can_move, _) => {
                            return rook_start.cmp(val2);
                        },
                        MoveType::Capture(_p, _from, val2, _cp, _) => {
                            return rook_start.cmp(val2);
                        },
                        MoveType::Defend(_p, _from, val2, _d, _) => {
                            return rook_start.cmp(val2);
                        },
                        _ => {
                            return Ordering::Less;
                        }
                    }
                },
                MoveType::Attack(_p, _from, val, _can_move, _) => {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Castle(rook_start, _, _, _, _) => {
                            return val.cmp(rook_start);
                        },
                        MoveType::Attack(_p, _from, val2, _can_move, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Capture(_p, _from, val2, _cp, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Defend(_p, _from, val2, _d, _) => {
                            return val.cmp(val2);
                        },
                        _ => {
                            return Ordering::Less;
                        }
                    }
                },
                MoveType::Capture(_p, _from, val, _cp, _) => {
                    match other {
                        MoveType::Standard(_from, val2, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::FutureMove(_p, _from, val2, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Castle(rook_start, _, _, _, _) => {
                            return val.cmp(rook_start);
                        },
                        MoveType::Attack(_p, _from, val2, _can_move, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Capture(_p, _from, val2, _cp, _) => {
                            return val.cmp(val2);
                        },
                        MoveType::Defend(_p, _from, val2, _d, _) => {
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

    #[derive(Debug, Clone, Copy)]
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
            /*let mut tot: u64 = 0;
            for i in 0..64 {
                let piece_value: u64 = Piece::piece_to_u64(&self.board_state[i]);
                tot += piece_value.pow(i as u32);
            }

            u64::from(tot)*/

            let mut hashAddress = 5381;
            for i in 0..64 {
                hashAddress = ((hashAddress << 5) + hashAddress) + Piece::piece_to_u64(&self.board_state[i]);
            }

            return hashAddress;
        }

        pub fn get_piece_from_position(board: &Board, position: u8) -> Option<Piece> {
            return board.board_state[position as usize];
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

        pub fn make_move(&mut self, from: usize, to: usize, m: &MoveType) -> () {
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
        }

    }
}
