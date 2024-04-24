mod generate;
mod rook_move_gen;
mod bishop_move_gen;

pub mod move_gen {
    
    use PieceType::{BISHOP, KING, KNIGHT, QUEEN, ROOK};
    
    
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

    #[derive(Clone)]
    pub struct MoveGen {
        pub king_position_board: Vec<u64>,//[u64; 64],
        pub knight_position_board: Vec<u64>,//[u64; 64],
        pub rook_masks: Vec<u64>,//[u64; 64],
        pub rook_magics: Vec<Magic>,//[Magic; 64],
        pub rook_table: Vec<u64>,//[u64; 102_400],
        pub bishop_masks: Vec<u64>,//[u64; 64],
        pub bishop_magics: Vec<Magic>, //[Magic; 64],
        pub bishop_table: Vec<u64>//[u64; 5_248]
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