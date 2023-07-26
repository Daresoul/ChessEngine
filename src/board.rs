pub mod board {

    use std::u8;

    use md5::{Md5, Digest};

    use crate::piece::piece::{Piece, PieceType};
    use crate::game::game::{Game};

    #[derive(Debug, Clone)]
    pub struct Board {
        pub board_value: u64,
        pub board_state: [Option<Piece>; 64],
        pub vec_questionMark: Vec<u8>
    }

    impl Board {
        pub fn new() -> Board {
            return Board {
                board_value: 0,
                board_state: [None; 64],
                vec_questionMark: vec![]
            };
        }

        pub fn set_board_bin(n: u64) -> Board {
            let board: Board = Board {
                board_value: n,
                board_state: [None; 64],
                vec_questionMark: vec![]
            };

            return board;
        }

        pub fn get_evaluation(&self) -> [u8; 64] {
            return self.board_state.map(|piece: Option<Piece>| Board::get_value(&piece));
        }

        pub fn set_bytes(&mut self) {

            for i in 0..64 {
                let piece: Option<Piece> = Board::get_piece_from_position(&self, i as u8);
                self.vec_questionMark.append(&mut vec![Board::get_value(&piece)]);
            }
        }
 
        pub fn compute_hash_working(&self) -> u64 { 
            self.board_state.iter().enumerate().rfold(0, |acc, (i, piece): (usize, &Option<Piece>)| {
                let piece_value: u8 = Board::get_value(&piece);
                acc + (piece_value as u64) * (i as u64)
            })
        }

        pub fn compute_hash(&self) -> usize { 
            self.board_state.iter().enumerate().rfold(0, |acc, (i, piece): (usize, &Option<Piece>)| {
                let piece_value: usize = Board::get_value_2(&piece);
                acc + piece_value * i
            })
        }

        pub fn get_piece(val: u8) -> Option<Piece> {
            match val {
                0 => None,
                1 => Some(Piece { is_white: true, piece_type: PieceType::Pawn}),
                2 => Some(Piece { is_white: true, piece_type: PieceType::Rook}),
                3 => Some(Piece { is_white: true, piece_type: PieceType::Knight}),
                4 => Some(Piece { is_white: true, piece_type: PieceType::Bishop}),
                5 => Some(Piece { is_white: true, piece_type: PieceType::Queen}),
                6 => Some(Piece { is_white: true, piece_type: PieceType::King}),
                7 => Some(Piece { is_white: false, piece_type: PieceType::Pawn}),
                8 => Some(Piece { is_white: false, piece_type: PieceType::Rook}),
                9 => Some(Piece { is_white: false, piece_type: PieceType::Knight}),
                10 => Some(Piece { is_white: false, piece_type: PieceType::Bishop}),
                11 => Some(Piece { is_white: false, piece_type: PieceType::Queen}),
                12 => Some(Piece { is_white: false, piece_type: PieceType::King}),
                _ => None
            }
        }


        pub fn get_value_2(piece: &Option<Piece>) -> usize {
            match piece {
                Some(piece) => {
                    match piece.is_white {
                        true => {
                            match piece.piece_type {
                                PieceType::Pawn => {
                                    return 1;
                                },
                                PieceType::Rook => {
                                    return 2;
                                },
                                PieceType::Knight => {
                                    return 3;
                                },
                                PieceType::Bishop => {
                                    return 4;
                                },
                                PieceType::Queen => {
                                    return 5;
                                },
                                PieceType::King => {
                                    return 6;
                                }
                            }
                        },
                        false => {
                            match piece.piece_type {
                                PieceType::Pawn => {
                                    return 7;
                                },
                                PieceType::Rook => {
                                    return 8;
                                },
                                PieceType::Knight => {
                                    return 9;
                                },
                                PieceType::Bishop => {
                                    return 10;
                                },
                                PieceType::Queen => {
                                    return 11;
                                },
                                PieceType::King => {
                                    return 12;
                                }
                            }
                        },                        
                    } 
                },
                None => {
                    return 0;
                }
            }
        }


        pub fn get_value(piece: &Option<Piece>) -> u8 {
            match piece {
                Some(piece) => {
                    match piece.is_white {
                        true => {
                            match piece.piece_type {
                                PieceType::Pawn => {
                                    return 1;
                                },
                                PieceType::Rook => {
                                    return 2;
                                },
                                PieceType::Knight => {
                                    return 3;
                                },
                                PieceType::Bishop => {
                                    return 4;
                                },
                                PieceType::Queen => {
                                    return 5;
                                },
                                PieceType::King => {
                                    return 6;
                                }
                            }
                        },
                        false => {
                            match piece.piece_type {
                                PieceType::Pawn => {
                                    return 7;
                                },
                                PieceType::Rook => {
                                    return 8;
                                },
                                PieceType::Knight => {
                                    return 9;
                                },
                                PieceType::Bishop => {
                                    return 10;
                                },
                                PieceType::Queen => {
                                    return 11;
                                },
                                PieceType::King => {
                                    return 12;
                                }
                            }
                        },                        
                    } 
                },
                None => {
                    return 0;
                }
            }
        }

        pub fn get_piece_from_position(board: &Board, position: u8) -> Option<Piece> {
            return board.board_state[position as usize];
        }

        pub fn move_piece(game: Game, from: u8, to: u8) -> Game {
            if Board::get_board_state_from_position(&game.board, from) {
                let piece: Option<Piece> = Board::get_piece_from_position(&game.board, from);
            }

            return Game { board: game.board };
        }

        pub fn get_board_state_from_position(board: &Board, position: u8) -> bool {
            if position > 63 {
                return false;
            }

            if (board.board_value & (1 << position)) > 0 {
                return true; 
            }
            return false;
        }

        pub fn set_board_state_from_position(board: &Board, position: u8) -> Board {
            let val: u64 = board.board_value ^ (1 << position);
            return Board {
                board_value: val,
                board_state: board.board_state,
                vec_questionMark: vec![]
            };
        }
    }
}
