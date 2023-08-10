
pub mod debug
{
    use crate::game::game::Game;
    use crate::board::board::{Board, MoveType};
    use crate::board::board::MoveType::{FutureMove, Standard};
    use crate::piece::piece::Piece;

    pub fn debug_board(game: &Game) {
       print!("board_value: {}", game.board.board_value);
       for i in 0..64 {
           if i % 8 == 0 {
               print!("\n");
           }
           print!("{} ", Board::get_board_state_from_position(&game.board, &(i as u8)));
        }
    }

    pub fn debug_board_state(game: &Game) {
        for i in 0..64 {
            if i % 8 == 0 {
                print!("\n");
            }
            print!("{} ", match game.board.board_state[i] {
                Some(piece) => piece.to_string(),
                None => "NN".to_string()
            });
        }
        print!("\n");
    }

    pub fn debug_board_state_with_index_marked(game: &Game, index: usize) {
        for i in 0..64 {
            if i % 8 == 0 {
                print!("\n");
            }
            if i == index {
                print!("*");
            }
            print!("{} ", match game.board.board_state[i] {
                Some(piece) => piece.to_string(),
                None => "NN".to_string()
            });
        }
        print!("\n");
    }

    pub fn debug_board_state_with_moves_marked_for_index(game: &Game, index: usize, move_list: &[Option<Vec<MoveType>>; 64]) {
        for i in 0..64 {
            if i % 8 == 0 {
                print!("\n");
            }

            if i == index {
                print!("*");
            }

            print!("{} ", match game.board.board_state[i] {
                Some(piece) => piece.to_string(),
                None =>
                    {
                        match &move_list[index] {
                            Some(moves) => {
                                let mut string = "NN";
                                for single_move in moves.iter() {
                                    match single_move {
                                        Standard(index) => {
                                            if *index == i as u8 {
                                                string = "KN";
                                            }
                                        },
                                        FutureMove(index) => {
                                            if *index == i as u8 {
                                                string = "ON";
                                            }
                                        },
                                        MoveType::Castle(a,b,c,d) => {
                                            if *d == i as u8 {
                                                string = "CN";
                                            }
                                        },
                                        _ => ()
                                    }
                                }
                                string.to_string()
                            },
                            None => "NN".to_string()
                        }
                    }
            });
        }

        print!("\n");
    }

    pub fn debug_board_state_with_moves_marked(game: &Game, index: usize, move_list: &[Option<Vec<MoveType>>; 64]) {
        for i in 0..64 {
            if i % 8 == 0{
                print!("\n");
            }
            if i == index {
                print!("*");
            }

            print!("{} ", match game.board.board_state[i] {
                Some(piece) => piece.to_string(),
                None => {
                    // Checking if there is a move on this square
                    let mut b: u8 = 0;
                    let mut p: Option<Piece> = None;
                    // Going over each position and finding the moves vector
                    for (j, moves) in move_list.iter().enumerate() {
                        match moves {
                            Some(moves) => {
                                // Going over each move (just an index)
                                for move_type in moves.iter() {
                                    match move_type {
                                        // if the square we are on is in the moves vector
                                        Standard(index) => {
                                            if *index == i as u8 {
                                                b = 1;
                                                p = game.board.board_state[j];
                                                break;
                                            }
                                        },
                                        FutureMove(index) => {
                                            if *index == i as u8 {
                                                b = 2;
                                                p = game.board.board_state[j];
                                                break;
                                            }
                                        },
                                        _ => ()
                                    }
                                }
                            },
                            None => ()
                        }
                    }
                    if b == 1 {
                        format!("{}{}", match p { Some(piece) => piece.to_string(), None => "X".to_string()}, "X".to_string())
                    } else if b == 2 {
                        format!("{}{}", match p { Some(piece) => piece.to_string(), None => "X".to_string()}, "O".to_string())
                    } else {
                        "NN".to_string()
                    }
                }
            });

        }
        print!("\n");
    }
}

