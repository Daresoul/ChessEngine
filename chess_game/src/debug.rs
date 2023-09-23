
pub mod debug
{
    use MoveType::Castle;
    
    use crate::board::board::{Board, MoveType};
    use crate::board::board::MoveType::{Attack, Capture, Defend, FutureMove, Standard};
    use crate::game::game::Game;
    use crate::piece::piece::Piece;


    pub fn print_board(game: &Game) {
        println!("Board:");
        for i in 0..8 {
            for j in 0..8 {
                print!("┌{:─^width$}┐", match game.board.board_state[i * 8 + j] {
                    Some(piece) => piece.to_string(),
                    None => " ".to_string()
                }, width = 4);
            }
            println!();
        }
    }

    pub fn print_board_board(board: &Board) {
        println!("Board:");
        for i in 0..8 {
            for j in 0..8 {
                print!("┌{:─^width$}┐", match board.board_state[i * 8 + j] {
                    Some(piece) => piece.to_string(),
                    None => " ".to_string()
                }, width = 4);
            }
            println!();
        }
    }

    pub fn get_all_from_position(moves: &Vec<MoveType>, starting_square: usize) -> Vec<MoveType> {
        let mut selected_moves: Vec<MoveType> = vec![];

        for single_move in moves.iter() {
            match single_move {
                Standard(from, _to, _) => {
                    if *from == starting_square as u8 {
                        selected_moves.push(*single_move);
                    }
                },
                FutureMove(_, from, _to, _) => {
                    if *from == starting_square as u8 {
                        selected_moves.push(*single_move);
                    }
                },
                Attack(_, from, _to, _, _) => {
                    if *from == starting_square as u8 {
                        selected_moves.push(*single_move);
                    }
                },
                Capture(_, from, _to, _, _) => {
                    if *from == starting_square as u8 {
                        selected_moves.push(*single_move);
                    }
                },
                Defend(_, from, _to, _, _) => {
                    if *from == starting_square as u8 {
                        selected_moves.push(*single_move);
                    }
                },
                Castle(rook_from, _, king_from, _, _) => {
                    if *rook_from == starting_square as u8 {
                        selected_moves.push(*single_move);
                    }

                    if *king_from == starting_square as u8 {
                        selected_moves.push(*single_move);
                    }
                },
                _ => ()
            }
        }

        return selected_moves;
    }

    pub fn count_color(all_moves: &Vec<MoveType>, is_white: bool) -> usize {
        let mut count = 0;
        for single_move in all_moves.iter() {
            match single_move {
                Standard(_, _, color) => {
                    if *color == is_white {
                        count += 1;
                    }
                }
                FutureMove(_, _, _, color) => {
                    if *color == is_white {
                        count += 1;
                    }
                },
                Capture(_, _, _, _, color) => {
                    if *color == is_white {
                        count += 1;
                    }
                },
                Attack(_, _, _, _, color) => {
                    if *color == is_white {
                        count += 1;
                    }
                },
                Defend(_, _, _, _, color) => {
                    if *color == is_white {
                        count += 1;
                    }
                },
                Castle(_, _, _, _, color) => {
                    if *color == is_white {
                        count += 1;
                    }
                },
                _ => ()
            }
        }
        return count;
    }

    pub fn find_specific_move(moves: &Vec<MoveType>, from: u8, to: u8) -> MoveType {
        for single_move in moves.iter() {
            match single_move {
                Standard(f, t, _) => {
                    if *f == from && *t == to {
                        return *single_move;
                    }
                },
                FutureMove(_, f, t, _) => {
                    if *f == from && *t == to {
                        return *single_move;
                    }
                },
                Defend(_pt, f, t, _, _) => {
                    if *f == from && *t == to {
                        return *single_move;
                    }
                },
                Attack(_, f, t, _, _) => {
                    if *f == from && *t == to {
                        return *single_move;
                    }
                },
                Capture(_, f, t, _, _) => {
                    if *f == from && *t == to {
                        return *single_move;
                    }
                },
                Castle(rf, rt, kf, kt, _) => {
                    if *rf == from && *rt == to {
                        return *single_move;
                    }

                    if *kf == from && *kt == to {
                        return *single_move;
                    }
                },
                _ => ()
            }
        }

        return Standard(0,0,false)

    }
}

