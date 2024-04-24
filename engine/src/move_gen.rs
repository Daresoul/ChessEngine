mod generate;
mod rook_move_gen;
mod bishop_move_gen;

pub mod move_gen {
    use log::debug;
    use PieceType::{BISHOP, KING, KNIGHT, QUEEN, ROOK};
    use crate::debug::debug;
    use crate::debug::debug::print_bitboard_board;
    use crate::magic::magic;

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
        PAWN,
        ROOK,
        KING,
        KNIGHT,
        BISHOP,
        QUEEN,
    }

    impl PartialEq for PieceType {
        fn eq(&self, other: &Self) -> bool {
            match self {
                PieceType::PAWN => {
                    match other {
                        PieceType::PAWN => true,
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
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct MoveGen {
        pub king_position_board: [u64; 64],
        pub knight_position_board: [u64; 64],
        pub rook_masks: [u64; 64],
        pub rook_magics: [magic::Magic; 64],
        pub rook_table: [u64; 102_400],
        pub bishop_masks: [u64; 64],
        pub bishop_magics: [magic::Magic; 64],
        pub bishop_table: [u64; 5_248]
    }

    impl MoveGen {
        pub fn init() -> MoveGen {
            let mut p = MoveGen {
                knight_position_board: [0; 64],
                king_position_board: [0; 64],
                rook_masks: [0; 64],
                rook_magics: [Default::default(); 64],
                rook_table: [0; 102_400],
                bishop_masks: [0; 64],
                bishop_magics: [Default::default(); 64],
                bishop_table: [0; 5_248],
            };

            p.calculate_knight_moves();
            p.calculate_king_moves();
            p.generate_rook_masks();
            p.generate_bishop_masks();
            p.init_rook_magics();
            p.init_bishop_magics();
            return p
        }

        pub fn get_move(&self, p: PieceType, pos: usize, team_occupancy: u64, occupancy: u64) -> u64 {
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
                _ => panic!("Not supported piece moved: {:?}", p)
            }
        }
    }
}