mod eval_board {
    use std::cmp::min;
    use std::ops::Sub;
    use crate::board::board::{Board, BoardMove};
    use crate::game::game::Game;
    use crate::move_gen::move_gen::PieceType;
    use crate::move_gen::move_gen::PieceType::{BISHOP, KNIGHT, PAWN, QUEEN, ROOK};
    use crate::utils::utils;

    impl Game {

        const PSQT_PAWN_BONUS: [[i32; 8]; 8] = [
            [0,0,0,0,0,0,0,0],
            [3,3,10,19,16,19,7,-5],
            [-9,-15,11,15,32,22,5,-22],
            [-4,-23,6,20,40,17,4,-8],
            [13,0,-13,1,11,-2,-13,5],
            [5,-12,-7,22,-8,-5,-15,-8],
            [-7,7,-3,-13,5,-16,10,-8],
            [0,0,0,0,0,0,0,0]
        ];

        const PSQT_KNIGHT_BONUS: [[i32; 8]; 8] = [
            [-175,-92,-74,-73,-73,-74,-92,-175],
            [-77,-41,-27,-15,-15,-27,-41,-77],
            [-61,-17,6,12, 12, 6, -17, -61],
            [-35,8,40,49, 49, 40, 8, -35],
            [-34,13,44,51, 51, 44, 13, -34],
            [-9,22,58,53, 53, 58, 22, -9],
            [-67,-27,4,37, 37, 4, -27, -67],
            [-201,-83,-56,-26, -26, -56, -83, -201]
        ];

        const PSQT_BISHOP_BONUS: [[i32; 8]; 8] = [
                [-53,-5,-8,-23,-23,-8,-5,-53],
                [-15,8,19,4,4,19,8,-15],
                [-7,21,-5,17,17,-5,21,-7],
                [-5,11,25,39,39,25,11,-5],
                [-12,29,22,31,31,22,29,-12],
                [-16,6,1,11,11,1,6,-16],
                [-17,-14,5,0,0,5,-14,-17],
                [-48,1,-14,-23,-23,-14,1,-48]
        ];

        const PSQT_ROOK_BONUS: [[i32; 8]; 8] = [
            [-53,-5,-8,-23,-23,-8,-5,-53],
            [-15,8,19,4,4,19,8,-15],
            [-7,21,-5,17,17,-5,21,-7],
            [-5,11,25,39,39,25,11,-5],
            [-12,29,22,31,31,22,29,-12],
            [-16,6,1,11,11,1,6,-16],
            [-17,-14,5,0,0,5,-14,-17],
            [-48,1,-14,-23,-23,-14,1,-48]
        ];

        const PSQT_QUEEN_BONUS: [[i32; 8]; 8] = [
            [-31,-20,-14,-5, -5, -14, -20, -31],
            [-21,-13,-8,6,6,-8,-13,-21],
            [-25,-11,-1,3,3,-1,-11,-25],
            [-13,-5,-4,-6,-6,-4,-5,-13],
            [-27,-15,-4,3,3,-4,-15,-27],
            [-22,-2,6,12,12,6,-2,-22],
            [-2,12,16,18,18,16,12,-2],
            [-17,-19,-1,9,9,-1,-19,-17]
        ];

        const PSQT_KING_BONUS: [[i32; 8]; 8] = [
            [271,327,271,198, 198, 271, 327, 271],
            [278,303,234,179, 179, 234, 303, 278],
            [195,258,169,120,120,169,258,195],
            [164,190,138,98,98,138,190,164],
            [154,179,105,70,70,105,179,154],
            [123,145,81,31,31,81,145,123],
            [88,120,65,33,33,65,120,88],
            [59,89,45,-1,-1,45,89,59]
        ];

        pub fn count_pieces_white(&self) -> i32 {
            let mut sum: i32 = 0;

            sum += utils::count_ones_i32(self.board.white_pawn_board) * 124;
            sum += utils::count_ones_i32(self.board.white_queen_board) * 2538;
            sum += utils::count_ones_i32(self.board.white_rook_board) * 1276;
            sum += utils::count_ones_i32(self.board.white_bishop_board) * 825;
            sum += utils::count_ones_i32(self.board.white_knight_board) * 781;

            return sum;
        }


        pub fn count_pieces_black(&self) -> i32 {
            let mut sum = 0;

            sum += utils::count_ones_i32(self.board.black_pawn_board) * 124;
            sum += utils::count_ones_i32(self.board.black_queen_board) * 2538;
            sum += utils::count_ones_i32(self.board.black_rook_board) * 1276;
            sum += utils::count_ones_i32(self.board.black_bishop_board) * 825;
            sum += utils::count_ones_i32(self.board.black_knight_board) * 781;

            return sum;
        }


        pub fn evaluate_board(&self) -> i32 {
            let mut eval = 0;
            let white_pieces_eval = self.count_pieces_white();
            let black_pieces_eval = self.count_pieces_black();

            eval += white_pieces_eval - black_pieces_eval;

            let psqt_white = self.psqt_bonus(true);
            let psqt_black = self.psqt_bonus(false);

            eval += psqt_white - psqt_black;

            eval += self.mobility();

            return eval;
        }

        pub fn mobility(&self) -> i32 {
            let mut sum = 0;

            let knight_bonus = [-62, -53, -12, -4, 3, 13, 22, 28, 33];
            let bishop_bonus = [-48, -20, -16, -26, 38, 51, 55, 63, 63, 68, 81, 81, 91, 98];
            let rook_bonus = [-60, -20, 2, 3, 3, 11, 22, 31, 40, 40, 41, 48, 57, 57, 62];
            let queen_bonus = [-30, -12, -8, -9, 20, 23, 23, 35, 38, 53, 64, 65, 65, 66, 67, 67, 72, 72, 77, 79, 93, 108, 108, 108, 110, 114, 114, 116];


            let mut moves = Vec::with_capacity(32);


            let occupancy = self.board.get_board_value();
            let white_occupancy = self.board.get_white_occupancy();
            let black_occupancy = self.board.get_black_occupancy();

            self.board.get_eval_moves(white_occupancy, black_occupancy, occupancy, &mut moves, true, &self.move_gen);

            self.board.get_eval_moves(black_occupancy, white_occupancy, occupancy,  &mut moves, false, &self.move_gen);


            for m in moves {
                let bonus = match m.piece_type {
                    ROOK => rook_bonus[m.attack_board.count_ones() as usize],
                    KNIGHT => knight_bonus[m.attack_board.count_ones() as usize],
                    BISHOP => bishop_bonus[m.attack_board.count_ones() as usize],
                    QUEEN => queen_bonus[m.attack_board.count_ones() as usize],
                    _ => panic!("Shouldnt get any other type moves here.")
                };

                if m.white {
                    sum += bonus;
                } else {
                    sum -= bonus;
                }
            }

            return sum
        }

        pub fn psqt_bonus(&self, is_white: bool) -> i32 {
            let mut eval = 0;

            let mut knight_board = if is_white {self.board.white_knight_board} else {self.board.black_knight_board};
            let mut rook_board = if is_white {self.board.white_rook_board} else {self.board.black_rook_board};
            let mut bishop_board = if is_white {self.board.white_bishop_board} else {self.board.black_bishop_board};
            let mut queen_board = if is_white {self.board.white_queen_board} else {self.board.black_queen_board};
            let mut king_board = if is_white {self.board.white_king_board} else {self.board.black_king_board};
            let mut pawn_board = if is_white {self.board.white_pawn_board} else {self.board.black_pawn_board};

            for _ in 0..(knight_board.count_ones() as usize) {
                let lsb = Board::pop_lsb(&mut knight_board);
                let (file, rank) = utils::get_file_and_rank(lsb);

                eval += if is_white {Self::PSQT_KNIGHT_BONUS[rank][file]} else {Self::PSQT_KNIGHT_BONUS[7-rank][file]}
            }

            for _ in 0..(rook_board.count_ones() as usize) {
                let lsb = Board::pop_lsb(&mut rook_board);
                let (file, rank) = utils::get_file_and_rank(lsb);

                eval += if is_white {Self::PSQT_ROOK_BONUS[rank][file]} else {Self::PSQT_ROOK_BONUS[7-rank][file]}
            }

            for _ in 0..(bishop_board.count_ones() as usize) {
                let lsb = Board::pop_lsb(&mut bishop_board);
                let (file, rank) = utils::get_file_and_rank(lsb);

                eval += if is_white {Self::PSQT_BISHOP_BONUS[rank][file]} else {Self::PSQT_BISHOP_BONUS[7-rank][file]}
            }

            for _ in 0..(queen_board.count_ones() as usize) {
                let lsb = Board::pop_lsb(&mut queen_board);
                let (file, rank) = utils::get_file_and_rank(lsb);

                eval += if is_white {Self::PSQT_QUEEN_BONUS[rank][file]} else {Self::PSQT_QUEEN_BONUS[7-rank][file]}
            }

            for _ in 0..(king_board.count_ones() as usize) {
                let lsb = Board::pop_lsb(&mut king_board);
                let (file, rank) = utils::get_file_and_rank(lsb);

                eval += if is_white {Self::PSQT_KING_BONUS[rank][file]} else {Self::PSQT_KING_BONUS[7-rank][file]}
            }



            for _ in 0..(pawn_board.count_ones() as usize) {
                let lsb = Board::pop_lsb(&mut pawn_board);
                let (file, rank) = utils::get_file_and_rank(lsb);

                eval += if is_white {Self::PSQT_PAWN_BONUS[rank][file]} else {Self::PSQT_PAWN_BONUS[7-rank][file]}
            }

            return eval
        }

    }
}