use log::debug;
use Direction::{NorthEast, NorthWest};
use crate::debug;
use crate::debug::debug::print_bitboard_board;
use crate::move_gen::move_gen::Direction::{East, North, South, SouthEast, SouthWest, West};
use crate::move_gen::move_gen::{Direction, MoveGen};
use crate::utils::utils;

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

    pub fn init_pawn_moves(&mut self) -> () {
        for pos in 0..64 {
            let mut white_move: u64 = 0;
            let mut black_move: u64 = 0;
            let mut white_attack: u64 = 0;
            let mut black_attack: u64 = 0;
            let position = utils::POSITIONS[pos];

            if utils::RANK_8 & position == 0 {
                white_move |= utils::POSITIONS[pos - 8];
                if utils::RANK_2 & position > 0 {
                    white_move |= utils::POSITIONS[pos - 16];
                }
            }

            if utils::RANK_1 & position == 0 {
                black_move |= utils::POSITIONS[pos + 8];
                if utils::RANK_7 & position > 0 {
                    black_move |= utils::POSITIONS[pos + 16];
                }
            }

            if utils::FILE_A & position == 0 {
                if utils::RANK_8 & position == 0 {
                    white_attack |= utils::POSITIONS[pos - 9];
                }

                if utils::RANK_1 & position == 0 {
                    black_attack |= utils::POSITIONS[pos + 7]
                }
            }

            if utils::FILE_H & position == 0 {
                if utils::RANK_8 & position == 0 {
                    white_attack |= utils::POSITIONS[pos - 7];
                }

                if utils::RANK_1 & position == 0 {
                    black_attack |= utils::POSITIONS[pos + 9]
                }
            }

            self.white_pawn_table[pos] = white_move;
            self.black_pawn_table[pos] = black_move;
            self.white_pawn_attack_table[pos] = white_attack;
            self.black_pawn_attack_table[pos] = black_attack;
        }
    }

    pub fn calculate_white_pawn_move(&self, pos: usize, occupancy: u64, opponent_occupancy: u64) -> u64{
        let mut result: u64 = 0;

        if pos < 8 {
            return result;
        }

        let position = 1 << pos;

        result |= 1 << pos - 8 & !occupancy;

        if result > 0 && 0xff000000000000_u64 & position > 0 {
            result |= 1 << pos - 16 & !occupancy;
        }

        if 0x101010101010101_u64 & position == 0 {
            result |= 1 << pos - 9 & opponent_occupancy;
        }

        if 0x8080808080808080_u64 & position == 0 {
            result |= 1 << pos - 7 & opponent_occupancy;
        }

        return result
    }

    pub fn calculate_black_pawn_move(&self, pos: usize, occupancy: u64, opponent_occupancy: u64) -> u64{

        let mut result: u64 = 0;

        if pos > 55 {
            return result;
        }

        let position = 1 << pos;

        result |= 1 << pos + 8 & !occupancy;

        if result > 0 && 0xff00_u64 & position > 0 {
            result |= 1 << pos + 16 & !occupancy;
        }

        if 0x101010101010101_u64 & position == 0 {
            result |= 1 << pos + 7 & opponent_occupancy;
        }

        if 0x8080808080808080_u64 & position == 0 {
            result |= 1 << pos + 9 & opponent_occupancy;
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