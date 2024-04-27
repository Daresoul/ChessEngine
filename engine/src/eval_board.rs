mod eval_board {
    use std::cmp::min;
    use crate::board::board::{Board, BoardMove};
    use crate::game::game::Game;
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

            let psqt_white = self.psqt_bonus_white(true);
            let psqt_black = self.psqt_bonus_white(false);

            eval += psqt_white - psqt_black;

            return eval;
        }

        pub fn psqt_bonus_white(&self, is_white: bool) -> i32 {
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


        /*pub fn evaluate_board(&self, tr: &TurnResult) -> i32 {
            let mut score: i32 = 0;
            let mut white_bishop = 0;
            let mut black_bishop = 0;

            let (white_attack, black_attack) = self.attacks(&tr.gbi);

            score += i32::try_from(white_attack).unwrap() - i32::try_from(black_attack).unwrap();

            for i in 0..64 {
                match self.board.board_state[i] {
                    Some(piece) => {
                        let (val, bishop) = self.piece_value_mg(piece, true);
                        score += val;
                        score += self.psqt_mg(i, piece, true);

                        if bishop == 1 { white_bishop += 1; } else if bishop == 2 { black_bishop += 1; }

                        if piece.is_white {
                            score += self.mobility_bonus(&i, &piece, &tr.white_moves, true);
                        } else {
                            score -= self.mobility_bonus(&i, &piece, &tr.black_moves, true);
                        }
                    },
                    None => ()
                }
            }

            score += self.bishop_pair(white_bishop, black_bishop);

            score
        }

        fn mobility_bonus(&self, index: &usize, piece: &Piece, moves: &Vec<MoveType>, is_mg: bool) -> i32 {
            let knight_bonus = if is_mg {[-62,-53,-12,-4,3,13,22,28,33]} else {[-81,-56,-31,-16,5,11,17,20,25]};
            let bishop_bonus = if is_mg {[-48,-20,16,26,38,51,55,63,63,68,81,81,91,98]} else {[-59,-23,-3,13,24,42,54,57,65,73,78,86,88,97]};
            let rook_bonus = if is_mg {[-60,-20,2,3,3,11,22,31,40,40,41,48,57,57,62]} else {[-78,-17,23,39,70,99,103,121,134,139,158,164,168,169,172]};
            let queen_bonus = if is_mg {[-30,-12,-8,-9,20,23,23,35,38,53,64,65,65,66,67,67,72,72,77,79,93,108,108,108,110,114,114,116]} else {[-48,-30,-7,19,40,55,59,75,78,96,96,100,121,127,131,133,136,141,147,150,151,168,168,171,182,182,192,219]};

            let mut mobility = self.mobility(moves, index);

            match piece.piece_type {
                Knight => return if mobility >= knight_bonus.len() {knight_bonus[knight_bonus.len() - 1]} else {knight_bonus[mobility]},
                Bishop => return if mobility >= bishop_bonus.len() {bishop_bonus[bishop_bonus.len() - 1]} else {bishop_bonus[mobility]},
                Rook => return if mobility >= rook_bonus.len() {rook_bonus[rook_bonus.len() - 1]} else {rook_bonus[mobility]},
                Queen => return if mobility >= queen_bonus.len() {queen_bonus[queen_bonus.len() - 1]} else {queen_bonus[mobility]},
                _ => return 0
            };
        }

        fn mobility(&self, moves: &Vec<MoveType>, index: &usize) -> usize {
            let mut moves_count = 0;

            for m in moves.iter() {
                match m {
                    Attack(p, from, _, _, _) => {
                        if usize::from(*from) == *index {
                            moves_count += 1;
                        }
                    }
                    Capture(p, from, _, _, _) => {
                        if usize::from(*from) == *index {
                            moves_count += 1;
                        }
                    }
                    _ => ()
                }
            }

            return moves_count;
        }

        fn psqt_mg(&self, index: usize, piece: Piece, is_mg: bool) -> i32 {
            let mut v = 0;

            let bonus = if is_mg { [[[-175,-92,-74,-73],[-77,-41,-27,-15],[-61,-17,6,12],[-35,8,40,49],[-34,13,44,51],[-9,22,58,53],[-67,-27,4,37],[-201,-83,-56,-26]], [[-53,-5,-8,-23],[-15,8,19,4],[-7,21,-5,17],[-5,11,25,39],[-12,29,22,31],[-16,6,1,11],[-17,-14,5,0],[-48,1,-14,-23]], [[-31,-20,-14,-5],[-21,-13,-8,6],[-25,-11,-1,3],[-13,-5,-4,-6],[-27,-15,-4,3],[-22,-2,6,12],[-2,12,16,18],[-17,-19,-1,9]], [[3,-5,-5,4],[-3,5,8,12],[-3,6,13,7],[4,5,9,8],[0,14,12,5],[-4,10,6,8],[-5,6,10,8],[-2,-2,1,-2]], [[271,327,271,198],[278,303,234,179],[195,258,169,120],[164,190,138,98],[154,179,105,70],[123,145,81,31],[88,120,65,33],[59,89,45,-1]]]} else { [[[-96,-65,-49,-21],[-67,-54,-18,8],[-40,-27,-8,29],[-35,-2,13,28],[-45,-16,9,39],[-51,-44,-16,17],[-69,-50,-51,12],[-100,-88,-56,-17]], [[-57,-30,-37,-12],[-37,-13,-17,1],[-16,-1,-2,10],[-20,-6,0,17],[-17,-1,-14,15],[-30,6,4,6],[-31,-20,-1,1],[-46,-42,-37,-24]], [[-9,-13,-10,-9],[-12,-9,-1,-2],[6,-8,-2,-6],[-6,1,-9,7],[-5,8,7,-6],[6,1,-7,10],[4,5,20,-5],[18,0,19,13]], [[-69,-57,-47,-26],[-55,-31,-22,-4],[-39,-18,-9,3],[-23,-3,13,24],[-29,-6,9,21],[-38,-18,-12,1],[-50,-27,-24,-8],[-75,-52,-43,-36]], [[1,45,85,76],[53,100,133,135],[88,130,169,175],[103,156,172,172],[96,166,199,199],[92,172,184,191],[47,121,116,131],[11,59,73,78]]] };

            let pbonus = if is_mg { [[0,0,0,0,0,0,0,0],[3,3,10,19,16,19,7,-5],[-9,-15,11,15,32,22,5,-22],[-4,-23,6,20,40,17,4,-8],[13,0,-13,1,11,-2,-13,5], [5,-12,-7,22,-8,-5,-15,-8],[-7,7,-3,-13,5,-16,10,-8],[0,0,0,0,0,0,0,0]] } else { [[0,0,0,0,0,0,0,0],[-10,-6,10,0,14,7,-5,-19],[-10,-10,-10,4,4,3,-6,-4],[6,-2,-8,-4,-13,-12,-10,-9],[10,5,4,-5,-5,-5,14,9], [28,20,21,28,30,7,6,13],[0,-11,12,21,25,19,4,7],[0,0,0,0,0,0,0,0]] };

            let row: usize = index / 8;
            let column: usize = index % 8;
            if piece.is_white {
                v += match piece.piece_type {
                    Pawn => pbonus[7 - row][column],
                    Knight => bonus[0][7 - row][usize::min(column, 7 - column)],
                    Bishop => bonus[1][7 - row][usize::min(column, 7 - column)],
                    Rook => bonus[2][7 - row][usize::min(column, 7 - column)],
                    Queen => bonus[3][7 - row][usize::min(column, 7 - column)],
                    King => bonus[4][7 - row][usize::min(column, 7 - column)]
                }
            } else {
                v -= match piece.piece_type {
                    Pawn => pbonus[7 - row][column],
                    Knight => bonus[0][7 - row][usize::min(column, 7 - column)],
                    Bishop => bonus[1][7 - row][usize::min(column, 7 - column)],
                    Rook => bonus[2][7 - row][usize::min(column, 7 - column)],
                    Queen => bonus[3][7 - row][usize::min(column, 7 - column)],
                    King => bonus[4][7 - row][usize::min(column, 7 - column)]
                }
            }

            return v
        }

        fn piece_value_mg(&self, piece: Piece, is_mg: bool) -> (i32, i32) {
            let mut all_piece_value: i32 = 0;
            let mut piece_value: i32 = 0;
            let mut pawn_value: i32 = 0;


            if piece.is_white {
                match piece.piece_type {
                    Pawn => pawn_value += piece.get_value(is_mg),
                    _ => piece_value += piece.get_value(is_mg)
                }
                all_piece_value += piece.get_value(is_mg)
            } else {
                match piece.piece_type {
                    Pawn => pawn_value -= piece.get_value(is_mg),
                    _ => piece_value -= piece.get_value(is_mg)
                }
                all_piece_value -= piece.get_value(is_mg)
            }

            let is_bishop = if piece.piece_type == Bishop && piece.is_white {
                1
            } else if piece.piece_type == Bishop && !piece.is_white {
                2
            } else {
                0
            };

            (all_piece_value + piece_value + pawn_value, is_bishop)
        }

        pub fn attacks(&self, game_info: &GameBoardInfo) -> (usize, usize) {
            let mut white = 0;
            let mut black = 0;
            for i in 0..12 {
                if i <= 5{
                    white += game_info.move_count[i];
                } else {
                    black += game_info.move_count[i];
                }
            }

            (white, black)
        }

        /*
        function bishop_pair(pos, square) {
          if (bishop_count(pos) < 2) return 0;
          if (square == null) return 1438;
          return board(pos, square.x, square.y) == "B" ? 1 : 0;
        }
         */
        fn bishop_pair(&self, bishop_count_white: usize, bishop_count_black: usize) -> i32 {
            let mut v = 0;

            if bishop_count_white == 2 {
                v += 1438;
            }

            if bishop_count_black == 2 {
                v -= 1438;
            }

            return (v / 16) << 0;
        }*/

    }
}