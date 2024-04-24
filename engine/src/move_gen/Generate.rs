use Direction::{NorthEast, NorthWest};
use crate::move_gen::move_gen::Direction::{East, North, South, SouthEast, SouthWest, West};
use crate::move_gen::move_gen::{Direction, MoveGen};


impl MoveGen {

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

    pub fn generate_bishop_masks(&mut self) {
        let edge_squares = 0xff00000000000000_u64 | 0xff_u64 | 0x8080808080808080_u64 | 0x101010101010101_u64;
        for i in 0..64 {
            let val = (Self::bb_ray(0, i, NorthEast) |
                Self::bb_ray(0, i, NorthWest) | Self::bb_ray(0, i, SouthWest) |
                Self::bb_ray(0, i, SouthEast)) & !edge_squares;

            self.bishop_masks[i] = val;

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
                NorthWest => {
                    if rank != 7 && file != 0 {
                        bb_square <<= 7;
                        bb_ray |= bb_square;
                        file -= 1;
                        rank += 1;
                        done = (bb_square & bb_in) > 0
                    }
                },
                NorthEast => {
                    if (rank != 7) && (file != 7) {
                        bb_square <<= 9;
                        bb_ray |= bb_square;
                        rank += 1;
                        file += 1;
                        done = (bb_square & bb_in) > 0;
                    }
                }
                Direction::SouthEast => {
                    if (rank != 0) && (file != 7) {
                        bb_square >>= 7;
                        bb_ray |= bb_square;
                        rank -= 1;
                        file += 1;
                        done = (bb_square & bb_in) > 0;
                    }
                }
                Direction::SouthWest => {
                    if (rank != 0) && (file != 0) {
                        bb_square >>= 9;
                        bb_ray |= bb_square;
                        rank -= 1;
                        file -= 1;
                        done = (bb_square & bb_in) > 0;
                    }
                }
            };
        }
        bb_ray
    }
}