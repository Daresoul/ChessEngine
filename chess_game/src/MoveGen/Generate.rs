pub mod pieces {

    pub enum Direction {
        South,
        West,
        North,
        East,
        /*    NorthWest,
            NorthEast,
            SouthWest,
            SouthEast*/
    }

    use log::debug;
    use crate::debug;
    use crate::Magics::magics::Magics;
    use crate::Generate::pieces::Direction::{East, North, South, West};
    use crate::UTILS::utils::{bitscan_forward, bitscan_reverse};

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
                PieceType::ROOK => {
                    match other {
                        PieceType::ROOK => true,
                        _ => false
                    }
                },
                PieceType::KING => {
                    match other {
                        PieceType::KING => true,
                        _ => false
                    }
                },
                PieceType::KNIGHT => {
                    match other {
                        PieceType::KNIGHT => true,
                        _ => false
                    }
                },
                PieceType::BISHOP => {
                    match other {
                        PieceType::BISHOP => true,
                        _ => false
                    }
                },
                PieceType::QUEEN => {
                    match other {
                        PieceType::QUEEN => true,
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
        pub rook_magics: [Magics; 64],
        pub rook_table: [u64; 102_400],
    }

    /*
uint64_t magic_move_rook (int8_t square, uint64_t occupancy) {
/* Remove occupants that aren't in the blocker mask for this square. */

/* Calculate the magic move index. */
int index = (occupancy*Rook.magic [square]) >> (64-Rook.bits [square]);
/* Return the pre-calculated move board. */
return Rook.moveboard [square] [index];
}
*/


    impl MoveGen {
        pub fn init() -> MoveGen {
            let mut p = MoveGen {
                knight_position_board: [0; 64],
                king_position_board: [0; 64],
                rook_masks: [0; 64],
                rook_magics: [Default::default(); 64],
                rook_table: [0; 102_400],
            };

            p.calculate_knight_moves();
            p.calculate_king_moves();
            p.generate_rook_masks();
            p.init_magics();
            return p
        }

        pub fn calculate_king_moves(&mut self) {
            let king_moves: [(i32, i32); 8] = [
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1),           (0, 1),
                (1, -1), (1, 0),  (1, 1)
            ];

            for i in 0..64 {
                let mut moves: u64 = 0;
                let x = i % 8;
                let y = i / 8;

                for &(dx, dy) in &king_moves {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 {
                        let j = (ny * 8 + nx) as u64;
                        moves |= 1 << j;
                    }
                }

                self.king_position_board[i] = moves;
            }
        }




        pub fn calculate_knight_moves(&mut self) {
            let bitshift: [(i32, i32); 8] = [
                (-2, -1), (-2, 1), (-1, -2), (-1, 2),
                (1, -2), (1, 2), (2, -1), (2, 1)
            ];

            for i in 0..64 {
                let mut moves: u64 = 0;
                let x = i % 8;
                let y = i / 8;

                for &(dx, dy) in &bitshift {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 {
                        let j = (ny * 8 + nx) as u64;
                        moves |= 1 << j;
                    }
                }

                self.knight_position_board[i] = moves;
            }
        }

        pub fn calculate_white_pawn_move(&self, position: u64, occupancy: u64, opponent_occupancy: u64) -> u64{
            let mut result: u64 = 0;
            let forwardPosition = 1 << position - 8;
            let forwardLegal = forwardPosition & occupancy;

            if forwardLegal == 0 {
                result += forwardPosition;
            }

            if position / 8 == 6 {
                let doubleForwardPosition = 1 << position - 16;
                let doubleForwardLegal = (doubleForwardPosition + forwardPosition) & occupancy;

                if doubleForwardLegal == 0 {
                    result += doubleForwardPosition;
                }
            }

            if position % 8 != 0 {
                let diagonalLeftPosition = 1 << position - 9;
                let diagonalLeftLegal = diagonalLeftPosition & opponent_occupancy;
                if diagonalLeftLegal > 0 {
                    result += diagonalLeftPosition;
                }
            }

            if position % 8 != 7 {
                let diagonalRightPosition = 1 << position - 7;
                let diagonalRightLegal = diagonalRightPosition & opponent_occupancy;
                if diagonalRightLegal > 0 {
                    result += diagonalRightPosition;
                }
            }

            return result
        }

        pub fn calculate_black_pawn_move(&self, position: u64, occupancy: u64, opponent_occupancy: u64) -> u64{
            let mut result: u64 = 0;
            if position + 8 > 63 {
                return 0;
            }
            let forwardPosition = 1 << position + 8;
            let forwardLegal = forwardPosition & occupancy;

            if forwardLegal == 0 {
                result += forwardPosition;
            }

            if position / 8 == 1 {
                let doubleForwardPosition = 1 << position + 16;
                let doubleForwardLegal = (doubleForwardPosition + forwardPosition) & occupancy;

                if doubleForwardLegal == 0 {
                    result += doubleForwardPosition;
                }
            }

            if position % 8 != 0 {
                if position + 9 < 64 {
                    let diagonalLeftPosition = 1 << position + 9;
                    let diagonalLeftLegal = diagonalLeftPosition & opponent_occupancy;
                    if diagonalLeftLegal > 0 {
                        result += diagonalLeftPosition;
                    }
                }
            }

            if position % 8 != 7 {
                let diagonalRightPosition = 1 << position + 7;
                let diagonalRightLegal = diagonalRightPosition & opponent_occupancy;
                if diagonalRightLegal > 0 {
                    result += diagonalRightPosition;
                }
            }

            return result
        }

        pub fn generate_rook_masks(&mut self) {
            for i in 0..64 {
                let val = (Self::bb_ray(0, i, North) & !0xff00000000000000_u64) |
                    (Self::bb_ray(0, i, South) & !0xff_u64) |
                    (Self::bb_ray(0, i, East) & !0x8080808080808080_u64) |
                    (Self::bb_ray(0, i, West) & !0x101010101010101_u64);

                self.rook_masks[i] = val;

            }
        }

        // Stolen from: https://github.com/mvanthoor/rustic/blob/master/src/movegen/create.rs
        pub fn bb_ray(bb_in: u64, square: usize, direction: Direction) -> u64 {
            let mut file = square % 8;
            let mut rank = square / 8;
            let mut bb_square = 1 << square;
            let mut bb_ray = 0;
            let mut done = false;
            while !done {
                done = true;
                match direction {
                    North => {
                        if rank != 7 {
                            bb_square <<= 8;
                            bb_ray |= bb_square;
                            rank += 1;
                            done = (bb_square & bb_in) > 0;
                        }
                    }
                    East => {
                        if file != 7 {
                            bb_square <<= 1;
                            bb_ray |= bb_square;
                            file += 1;
                            done = (bb_square & bb_in) > 0;
                        }
                    }
                    South => {
                        if rank != 0 {
                            bb_square >>= 8;
                            bb_ray |= bb_square;
                            rank -= 1;
                            done = (bb_square & bb_in) > 0;
                        }
                    }
                    West => {
                        if file != 0 {
                            bb_square >>= 1;
                            bb_ray |= bb_square;
                            file -= 1;
                            done = (bb_square & bb_in) > 0;
                        }
                    }
                };
            }
            bb_ray
        }
    }
}