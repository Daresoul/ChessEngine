pub mod game;
pub mod board;
mod piece;
mod debug;
pub mod debug_structs;
/*
    TODO:
    Remember to think about castling - Can do a lot with checking on the binary map
    Remember promotion - Can be done with enum types as seen below:
    struct move_type {
        move(u8) // move(square to move to)
        capture(u8) // maybe a capture type?
        castle(u8, u8, u8, u8) // castle(king start, king end, rook start, rook end)
        promotion(u8, Piece) // promotion(square to move to, piece to promote to)
    }

    How to check for checks if a move happen?
    Can introduce an option type:
    Option(u8, u8) - Option(How many pieces are in between, place to move to)
    Can also be done with Option(u8) as i dont think its worth looking at them if there is more
    than one piece in between.

    Checkmates should be done in the evaluation and can be done by checking if no valid moves
    are available.

      _____                 _
     |  __ \               | |
     | |__) |___   __ _  __| |_ __ ___   __ _ _ __
     |  _  // _ \ / _` |/ _` | '_ ` _ \ / _` | '_ \
     | | \ \ (_) | (_| | (_| | | | | | | (_| | |_) |
     |_|  \_\___/ \__,_|\__,_|_| |_| |_|\__,_| .__/
                                             | |
                                             |_|
   ___  _                    ___
  / __|| |_   ___  ___ ___  / __| __ _  _ __   ___
 | (__ | ' \ / -_)(_-<(_-< | (_ |/ _` || '  \ / -_)
  \___||_||_|\___|/__//__/  \___|\__,_||_|_|_|\___|

     develop pawns: done
     develop rook: done
     develop king: todo
     develop castling: todo
     develop en passant: todo
     develop move: todo
     develop check (as described above): todo
     develop simple board evaluation (with checkmate): todo
     develop rest of simple moves: todo

  ___              _
 | __| _ _   __ _ (_) _ _   ___
 | _| | ' \ / _` || || ' \ / -_)
 |___||_||_|\__, ||_||_||_|\___|
            |___/

    create lib for engine: todo
    create binary for sampling the libs, and testing them together: todo
    develop simple engine to go over all moves: todo
    develop alpha beta pruning: todo
    develop transposition tables: todo
    develop iterative deepening: todo


 */


// cargo test -- --nocapture
// cargo test -- --nocapture --test-threads=1

// Look into peekable: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.peekable

#[cfg(test)]
mod tests {
    use crate::piece::piece::PieceType::{King, Rook, Bishop, Knight, Pawn, Queen};
    use crate::game::game::{Game};
    use crate::board::board::{Board};
    use crate::piece::piece::Piece;
    use std::time::Instant;
    use crate::board::board::MoveType::{FutureMove, Standard};
    use crate::debug::debug;
    use crate::debug::debug::debug_board_state_with_moves_marked;
    use crate::debug_structs::debug_structs;

    #[test]
    fn test_board_value_one_piece_start() {
        let mut arr : [Option<Piece>; 64] = [None; 64];

        arr[0] = Some(Piece { piece_type: Pawn, is_white: true});

        let val = Board::get_board_value(arr);

        assert_eq!(val, 1);

        arr[0] = Some(Piece { piece_type: King, is_white: false});

        let val = Board::get_board_value(arr);

        assert_eq!(val, 1);

    }

    #[test]
    fn test_board_value_one_piece_end() {
        let mut arr : [Option<Piece>; 64] = [None; 64];

        arr[63] = Some(Piece { piece_type: Pawn, is_white: true});

        let val = Board::get_board_value(arr);

        assert_eq!(val, 1 << 63);

        arr[63] = Some(Piece { piece_type: King, is_white: false});

        let val = Board::get_board_value(arr);

        assert_eq!(val, 1 << 63);

    }

    #[test]
    fn test_board_value_full_board() {
        let mut arr : [Option<Piece>; 64] = [None; 64];

        for i in 0..64 {
            arr[i] = Some(Piece { piece_type: Pawn, is_white: true});
        }

        let val = Board::get_board_value(arr);

        assert_eq!(val, u64::MAX);

        for i in 0..64 {
            arr[i] = Some(Piece { piece_type: Queen, is_white: false});
        }

        let val = Board::get_board_value(arr);

        assert_eq!(val, u64::MAX);

    }

    #[test]
    fn test_board_value_normal_board() {
        let val = Board::get_board_value(debug_structs::get_normal_board());

        assert_eq!(val, 18_446_462_598_732_906_495);
    }

    #[test]
    fn test_of_new_game_from_state() {
        let arr : [Option<Piece>; 64] = debug_structs::get_normal_board();
        let game = Game::new_from_arr(arr, true);

        assert_eq!(game.board.board_state, arr);
        assert_eq!(game.board.board_value, 18_446_462_598_732_906_495);
    }

    #[test]
    fn piece_to_value_test() {

        let piece1: Option<Piece> = None; 
        assert_eq!(Piece::piece_to_u64(&piece1), 0);

        let piece2: Option<Piece> = Some(Piece { is_white: true, piece_type: Pawn});
        assert_eq!(Piece::piece_to_u64(&piece2), 1);

        let piece3: Option<Piece> = Some(Piece { is_white: true, piece_type: Rook});
        assert_eq!(Piece::piece_to_u64(&piece3), 2);

        let piece4: Option<Piece> = Some(Piece { is_white: true, piece_type: Knight});
        assert_eq!(Piece::piece_to_u64(&piece4), 3);

        let piece5: Option<Piece> = Some(Piece { is_white: true, piece_type: Bishop});
        assert_eq!(Piece::piece_to_u64(&piece5), 4);

        let piece6: Option<Piece> = Some(Piece { is_white: true, piece_type: King});
        assert_eq!(Piece::piece_to_u64(&piece6), 6);

        let piece7: Option<Piece> = Some(Piece { is_white: true, piece_type: Queen});
        assert_eq!(Piece::piece_to_u64(&piece7), 5);



        let piece8: Option<Piece> = Some(Piece { is_white: false, piece_type: Pawn});
        assert_eq!(Piece::piece_to_u64(&piece8), 7);

        let piece9: Option<Piece> = Some(Piece { is_white: false, piece_type: Rook});
        assert_eq!(Piece::piece_to_u64(&piece9), 8);

        let piece10: Option<Piece> = Some(Piece { is_white: false, piece_type: Knight});
        assert_eq!(Piece::piece_to_u64(&piece10), 9);

        let piece11: Option<Piece> = Some(Piece { is_white: false, piece_type: Bishop});
        assert_eq!(Piece::piece_to_u64(&piece11), 10);

        let piece12: Option<Piece> = Some(Piece { is_white: false, piece_type: King});
        assert_eq!(Piece::piece_to_u64(&piece12), 12);

        let piece13: Option<Piece> = Some(Piece { is_white: false, piece_type: Queen});
        assert_eq!(Piece::piece_to_u64(&piece13), 11);
    }

    #[test]
    fn value_to_piece_test() {

        let none: u64 = 0; 
        assert_eq!(Piece::u64_to_piece(&none), None);

        let wpawn: u64 = 1; 
        assert_eq!(Piece::u64_to_piece(&wpawn), Some(Piece { is_white: true, piece_type: Pawn}));

        let wrook: u64 = 2; 
        assert_eq!(Piece::u64_to_piece(&wrook), Some(Piece { is_white: true, piece_type: Rook}));

        let wknight: u64 = 3; 
        assert_eq!(Piece::u64_to_piece(&wknight), Some(Piece { is_white: true, piece_type: Knight}));

        let wbishop: u64 = 4; 
        assert_eq!(Piece::u64_to_piece(&wbishop), Some(Piece { is_white: true, piece_type: Bishop}));

        let wqueen: u64 = 5; 
        assert_eq!(Piece::u64_to_piece(&wqueen), Some(Piece { is_white: true, piece_type: Queen}));

        let wking: u64 = 6; 
        assert_eq!(Piece::u64_to_piece(&wking), Some(Piece { is_white: true, piece_type: King}));




        let bpawn: u64 = 7; 
        assert_eq!(Piece::u64_to_piece(&bpawn), Some(Piece { is_white: false, piece_type: Pawn}));

        let brook: u64 = 8; 
        assert_eq!(Piece::u64_to_piece(&brook), Some(Piece { is_white: false, piece_type: Rook}));

        let bknight: u64 = 9; 
        assert_eq!(Piece::u64_to_piece(&bknight), Some(Piece { is_white: false, piece_type: Knight}));

        let bbishop: u64 = 10; 
        assert_eq!(Piece::u64_to_piece(&bbishop), Some(Piece { is_white: false, piece_type: Bishop}));

        let bqueen: u64 = 11; 
        assert_eq!(Piece::u64_to_piece(&bqueen), Some(Piece { is_white: false, piece_type: Queen}));

        let bking: u64 = 12; 
        assert_eq!(Piece::u64_to_piece(&bking), Some(Piece { is_white: false, piece_type: King}));

        let  over: u64 = 13; 
        assert_eq!(Piece::u64_to_piece(&over), None);
    }

    #[test]
    fn king_move() {
        let game = Game::new_from_string("8/8/8/3K4/8/8/8/8".to_string(), true);

        let mut all_moves = game.get_all_moves();
        let mut x = all_moves[27].take().unwrap();

        let mut expected_moves = vec![
            Standard(18), Standard(19), Standard(20),
            Standard(26), Standard(28),
            Standard(34), Standard(35), Standard(36),
        ];

        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn king_move_cant_take() {
        let game = Game::new_from_string("8/8/2PPP3/2PKP3/2PPP3/8/8/8".to_string(), true);

        let mut all_moves = game.get_all_moves();
        let mut x = all_moves[27].take().unwrap();

        let mut expected_moves = vec![
        ];

        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn king_move_take() {
        let game = Game::new_from_string("8/8/2ppp3/2pKp3/2ppp3/8/8/8".to_string(), true);

        let mut all_moves = game.get_all_moves();
        let mut x = all_moves[27].take().unwrap();

        let mut expected_moves = vec![
            Standard(18), Standard(19), Standard(20),
            Standard(26), Standard(28),
            Standard(34), Standard(35), Standard(36),
        ];

        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn pawn_move_single_double() {
        let game = Game::new_from_string("8/3p4/8/8/8/8/3P4/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let black_len = match &all_moves[11] {
            Some(x) => x.len(),
            None => 0
        };

        assert_eq!(black_len, 0);

        let white_len = match &all_moves[51] {
            Some(x) => x.len(),
            None => 0
        };

        assert_eq!(white_len, 2);
    }

    #[test]
    fn pawn_move_all_moves_available() {
        let game = Game::new_from_string("8/3p4/2P1P3/8/8/2p1p3/3P4/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let black_len = match &all_moves[11] {
            Some(x) => x.len(),
            None => 0
        };

        assert_eq!(black_len, 0);

        let white_len = match &all_moves[51] {
            Some(x) => x.len(),
            None => 0
        };

        assert_eq!(white_len, 4);
    }

    #[test]
    fn pawn_move_cant_take_allied_pieces() {
        let game = Game::new_from_string("8/3p4/2p1p3/8/8/2P1P3/3P4/8".to_string(), false);

        let all_moves = game.get_all_moves();

        let black_len = match &all_moves[11] {
            Some(x) => x.len(),
            None => 0
        };

        assert_eq!(black_len, 2);

        let white_len = match &all_moves[51] {
            Some(x) => x.len(),
            None => 0
        };

        assert_eq!(white_len, 0);
    }

    #[test]
    fn pawn_move_can_take_outside_start_squares() {
        let game = Game::new_from_string("8/8/3p4/2P1P3/2p1p3/3P4/8/8".to_string(), false);

        let all_moves = game.get_all_moves();

        let black_len = match &all_moves[19] {
            Some(x) => x.len(),
            None => 0
        };

        assert_eq!(black_len, 3);

        let white_len = match &all_moves[43] {
            Some(x) => x.len(),
            None => 0
        };

        assert_eq!(white_len, 0);
    }

    #[test]
    fn pawn_move_cant_move_double() {
        let game = Game::new_from_string("8/8/3p4/8/8/3P4/8/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let black_len = match &all_moves[19] {
            Some(x) => x.len(),
            None => 0
        };

        assert_eq!(black_len, 0);

        let white_len = match &all_moves[43] {
            Some(x) => x.len(),
            None => 0
        };

        assert_eq!(white_len, 1);
    }

    #[test]
    fn rook_move_nothing() {
        let game = Game::new_from_string("8/8/8/3R4/8/8/8/8".to_string(), true);

        let mut all_moves = game.get_all_moves();

        let mut x = all_moves[27].take().unwrap();
        x.sort();

        let mut expected_moves = vec![
            Standard(3), Standard(11), Standard(19), Standard(35), Standard(43), Standard(51), Standard(59),
            Standard(24), Standard(25), Standard(26), Standard(28), Standard(29), Standard(30), Standard(31),
        ];
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn rook_move_1_piece_opponent() {
        let game = Game::new_from_string("8/8/3p4/2pR1p2/8/3p4/8/8".to_string(), true);

        let mut all_moves = game.get_all_moves();

        let mut x = all_moves[27].take().unwrap();
        x.sort();

        let mut expected_moves = vec![
            FutureMove(3), FutureMove(11), Standard(19), Standard(35), Standard(43), FutureMove(51), FutureMove(59),
            FutureMove(24), FutureMove(25), Standard(26), Standard(28), Standard(29), FutureMove(30), FutureMove(31)
        ];
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn rook_move_2_piece_opponent() {
        let game = Game::new_from_string("8/3p4/3p4/1ppR1pp1/8/3p4/3p4/8".to_string(), true);

        let mut all_moves = game.get_all_moves();

        let mut x = all_moves[27].take().unwrap();
        x.sort();

        let mut expected_moves = vec![
            FutureMove(11), Standard(19), Standard(35), Standard(43), FutureMove(51),
            FutureMove(25), Standard(26), Standard(28), Standard(29), FutureMove(30)
        ];
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_transform_standard() {
        let state = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);
        assert_eq!(state.board.board_state, debug_structs::get_normal_board());
    }

    #[test]
    fn test_castle_white() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/R3K2R".to_string(), true);
        let mut all_moves = game.get_all_moves();
        debug::debug_board_state_with_moves_marked_for_index(&game, 60, &all_moves);
    }

    //#[test]
    /*fn hash_performance_test() {

        let boards = debug_structs::generate_boards(1_000_000);

        let mut hash: Vec<u64> = vec![];
        let cloned_boards2 = boards.clone();
        let now = Instant::now();

        for board in cloned_boards2 {
            hash.push(board.compute_hash());
        }
        let elapsed = now.elapsed();


        println!("time: {:.2?}", elapsed);
        let mut hash2: Vec<u64> = vec![];
        let cloned_boards = boards.clone();
        let now = Instant::now();

        for board in cloned_boards {
            hash2.push(board.compute_hash2());
        }
        let elapsed = now.elapsed();


        println!("time: {:.2?}", elapsed);

        for (i, x) in hash.iter().enumerate() {
            assert_eq!(*x, hash2[i])
        }
    }*/

    /*#[test]
    fn make_move_performance_test() {

        let games = debug_structs::generate_games(1_000_000);

        let now = Instant::now();

        for game in games.iter() {
            let all_moves = game.get_all_moves();
        }

        let elapsed = now.elapsed();


        /*let current_index = 11;
        debug::debug_board_state_with_moves_marked(&game, current_index, &all_moves);
        println!("moves_count: {}", match &all_moves[current_index] {
            Some(x) => x.len(),
            None => 0
        });*/
        println!("time: {:.2?}", elapsed);
    }*/



}
