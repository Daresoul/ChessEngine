mod eval_board {
    use crate::board::board::MoveType;
    use crate::board::board::MoveType::{Attack, Capture};
    use crate::game::game::{Game, TurnResult};
    use crate::game_board_info::game_board_info::GameBoardInfo;
    use crate::piece::piece::Piece;
    use crate::piece::piece::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};

    impl Game {
        pub fn evaluate_board(&self, tr: &TurnResult) -> i32 {
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
                Knight => return knight_bonus[mobility],
                Bishop => return bishop_bonus[mobility],
                Rook => return rook_bonus[mobility],
                Queen => return queen_bonus[mobility],
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
        }

    }
}