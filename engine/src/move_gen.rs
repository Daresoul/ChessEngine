mod generate;
mod rook_move_gen;
mod bishop_move_gen;

pub mod move_gen {
    use std::fmt;
    use std::fmt::{Formatter, write};
    use PieceType::{BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK};
    
    
    use crate::magic::magic;
    use crate::magic::magic::Magic;

    pub enum Direction {
        South,
        West,
        North,
        East,
        NorthWest,
        NorthEast,
        SouthWest,
        SouthEast
    }

    #[derive(Debug, Clone, Copy)]
    pub enum PieceType {
        None = 0,
        PAWN = 1,
        ROOK = 2,
        KING = 3,
        KNIGHT = 4,
        BISHOP = 5,
        QUEEN = 6,
    }

    impl fmt::Display for PieceType {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                PieceType::None => write!(f, "NONE"),
                PAWN => write!(f, "P"),
                ROOK => write!(f, "R"),
                KING => write!(f, "K"),
                KNIGHT => write!(f, "N"),
                BISHOP => write!(f, "B"),
                QUEEN => write!(f, "Q")
            }
        }
    }

    impl PartialEq for PieceType {
        fn eq(&self, other: &Self) -> bool {
            match self {
                PAWN => {
                    match other {
                        PAWN => true,
                        _ => false
                    }
                },
                ROOK => {
                    match other {
                        ROOK => true,
                        _ => false
                    }
                },
                KING => {
                    match other {
                        KING => true,
                        _ => false
                    }
                },
                KNIGHT => {
                    match other {
                        KNIGHT => true,
                        _ => false
                    }
                },
                BISHOP => {
                    match other {
                        BISHOP => true,
                        _ => false
                    }
                },
                QUEEN => {
                    match other {
                        QUEEN => true,
                        _ => false
                    }
                },
                PieceType::None => {
                    match other {
                        PieceType::None => true,
                        _ => false
                    }
                }
            }
        }
    }

    #[derive(Clone)]
    pub struct MoveGen {
        pub king_position_board: Vec<u64>,//[u64; 64],
        pub knight_position_board: Vec<u64>,//[u64; 64],
        pub rook_masks: Vec<u64>,//[u64; 64],
        pub rook_magics: Vec<Magic>,//[Magic; 64],
        pub rook_table: Vec<u64>,//[u64; 102_400],
        pub bishop_masks: Vec<u64>,//[u64; 64],
        pub bishop_magics: Vec<Magic>, //[Magic; 64],
        pub bishop_table: Vec<u64>,//[u64; 5_248]
        pub white_pawn_table: Vec<u64>,
        pub black_pawn_table: Vec<u64>,
        pub white_pawn_attack_table: Vec<u64>,
        pub black_pawn_attack_table: Vec<u64>
    }

    impl MoveGen {
        pub fn init() -> MoveGen {
            let mut p = MoveGen {
                knight_position_board: vec![0; 64],
                king_position_board: vec![0; 64],
                rook_masks: vec![0; 64],
                rook_magics: vec![Magic {magic_number: 0, offset: 0, shift: 0, mask: 0}; 64],
                rook_table: vec![0; 102_400],
                bishop_masks: vec![0; 64],
                bishop_magics: vec![Magic {magic_number: 0, offset: 0, shift: 0, mask: 0}; 64],
                bishop_table: vec![0; 5_248],
                white_pawn_table: vec![0; 64],
                black_pawn_table: vec![0; 64],
                white_pawn_attack_table: vec![0; 64],
                black_pawn_attack_table: vec![0; 64],
            };

            p.calculate_knight_moves();
            p.calculate_king_moves();
            p.generate_rook_masks();
            p.generate_bishop_masks();
            p.init_pawn_moves();
            p.init_rook_magics();
            p.init_bishop_magics();
            return p
        }

        pub fn get_move(&self, p: PieceType, pos: usize, team_occupancy: u64, occupancy: u64, opponent_occupancy: u64, is_white: bool) -> u64 {
            match p {
                KNIGHT => {
                    self.knight_position_board[pos] & !team_occupancy
                },
                ROOK => {
                    self.get_rook_moves(pos, occupancy) & !team_occupancy
                },
                BISHOP => {
                    self.get_bishop_moves(pos, occupancy) & !team_occupancy
                },
                QUEEN => {
                    (self.get_bishop_moves(pos, occupancy) | self.get_rook_moves(pos, occupancy)) & !team_occupancy
                },
                KING => {
                    self.king_position_board[pos] & !team_occupancy
                }
                PAWN => {
                    let mut res = 0;

                    if is_white {
                        res |= self.white_pawn_table[pos] & !occupancy;
                        res |= self.white_pawn_attack_table[pos] & opponent_occupancy;
                    } else {
                        res |= self.black_pawn_table[pos] & !occupancy;
                        res |= self.black_pawn_attack_table[pos] & opponent_occupancy;
                    }

                    res
                }
                _ => panic!("Not supported piece moved: {:?}", p)
            }
        }

        fn white_pawn_move(pos: usize, additive: usize) -> u64 {
            if additive > pos {
                return 0
            }
            1_u64.overflowing_shl((pos - additive) as u32).0
        }

        fn black_pawn_move(pos: usize, additive: usize) -> u64 {
            if additive + pos > 63 {
                return 0
            }
            1_u64.overflowing_shl((pos + additive) as u32).0
        }
    }
}