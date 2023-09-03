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
     develop king: done
     develop castling: done
     develop en passant: todo
     develop move: sorta done
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
    use crate::piece::piece::{Piece};
    use crate::board::board::MoveType::{Castle, FutureMove, Standard};
    use crate::debug::debug;
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

        let mut expected_moves = vec![
            Standard(27, 18, true), Standard(27, 19, true),
            Standard(27, 20, true), Standard(27, 26, true),
            Standard(27, 28, true), Standard(27, 34, true),
            Standard(27, 35, true), Standard(27, 36, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn king_move_top_right_corner() {
        let game = Game::new_from_string("7K/8/8/8/8/8/8/8".to_string(), true);

        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(7,6, true), Standard(7,14, true), Standard(7,15, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 7);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn king_move_bottom_right_corner() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/7K".to_string(), true);

        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(63, 54, true), Standard(63, 55, true), Standard(63, 62, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 63);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn king_move_bottom_left_corner() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/K7".to_string(), true);

        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(56,48, true), Standard(56,49, true), Standard(56,57, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 56);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn king_move_top_left_corner() {
        let game = Game::new_from_string("K7/8/8/8/8/8/8/8".to_string(), true);

        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(0,1, true), Standard(0,8, true), Standard(0,9, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 0);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn king_move_cant_take() {
        let game = Game::new_from_string("8/8/2PPP3/2PKP3/2PPP3/8/8/8".to_string(), true);

        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
        ];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn king_move_take() {
        let game = Game::new_from_string("8/8/2ppp3/2pKp3/2ppp3/8/8/8".to_string(), true);

        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(27,18, true), Standard(27,19, true), Standard(27,20, true),
            Standard(27,26, true), Standard(27,28, true),
            Standard(27,34, true), Standard(27,35, true), Standard(27,36, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn pawn_move_single_double() {
        let game = Game::new_from_string("8/3p4/8/8/8/8/3P4/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let black_len = debug::get_all_from_position(&all_moves, 11).len();

        assert_eq!(black_len, 2);

        let white_len = debug::get_all_from_position(&all_moves, 51).len();

        assert_eq!(white_len, 2);
    }

    #[test]
    fn pawn_move_all_moves_available() {
        let game = Game::new_from_string("8/3p4/2P1P3/8/8/2p1p3/3P4/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let black_len = debug::get_all_from_position(&all_moves, 11).len();

        assert_eq!(black_len, 4);

        let white_len = debug::get_all_from_position(&all_moves, 51).len();

        assert_eq!(white_len, 4);
    }

    #[test]
    fn pawn_move_cant_take_allied_pieces() {
        let game = Game::new_from_string("8/3p4/2p1p3/8/8/2P1P3/3P4/8".to_string(), false);

        let all_moves = game.get_all_moves();

        let black_len = debug::get_all_from_position(&all_moves, 11).len();

        assert_eq!(black_len, 2);

        let white_len = debug::get_all_from_position(&all_moves, 51).len();

        assert_eq!(white_len, 2);
    }

    #[test]
    fn pawn_move_can_take_outside_start_squares() {
        let game = Game::new_from_string("8/8/3p4/2P1P3/2p1p3/3P4/8/8".to_string(), false);

        let all_moves = game.get_all_moves();

        let black_len = debug::get_all_from_position(&all_moves, 19).len();

        assert_eq!(black_len, 3);

        let white_len = debug::get_all_from_position(&all_moves, 43).len();

        assert_eq!(white_len, 3);
    }

    #[test]
    fn pawn_move_cant_move_double() {
        let game = Game::new_from_string("8/8/3p4/8/8/3P4/8/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let black_len = debug::get_all_from_position(&all_moves, 19).len();

        assert_eq!(black_len, 1);

        let white_len = debug::get_all_from_position(&all_moves, 43).len();

        assert_eq!(white_len, 1);
    }

    #[test]
    fn rook_move_nothing() {
        let game = Game::new_from_string("8/8/8/3R4/8/8/8/8".to_string(), true);

        let mut all_moves = game.get_all_moves();


        let mut expected_moves = vec![
            Standard(27,3, true), Standard(27,11, true), Standard(27,19, true),
            Standard(27,35, true), Standard(27,43, true), Standard(27,51, true),
            Standard(27,59, true), Standard(27,24, true), Standard(27,25, true),
            Standard(27,26, true), Standard(27,28, true), Standard(27,29, true),
            Standard(27,30, true), Standard(27,31, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn rook_move_1_piece_opponent() {
        let game = Game::new_from_string("8/8/3p4/2pR1p2/8/3p4/8/8".to_string(), true);

        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            FutureMove(27,3, true), FutureMove(27,11, true), Standard(27,19, true),
            Standard(27,35, true), Standard(27,43, true), FutureMove(27,51, true),
            FutureMove(27,59, true), FutureMove(27,24, true), FutureMove(27,25, true),
            Standard(27,26, true), Standard(27,28, true), Standard(27,29, true),
            FutureMove(27,30, true), FutureMove(27,31, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn rook_move_2_piece_opponent() {
        let game = Game::new_from_string("8/3p4/3p4/1ppR1pp1/8/3p4/3p4/8".to_string(), true);

        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            FutureMove(27,11, true), Standard(27,19, true), Standard(27,35, true),
            Standard(27,43, true), FutureMove(27,51, true), FutureMove(27,25, true),
            Standard(27,26, true), Standard(27,28, true), Standard(27,29, true),
            FutureMove(27,30, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
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

        let mut expected_moves = vec![
            Standard(60,51, true), Standard(60,52, true), Standard(60,53, true),
            Standard(60,59, true), Standard(60,61, true),
            Castle(56, 59, 60, 58, true),
            Castle(63, 61, 60, 62, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }


    #[test]
    fn test_castle_white_cant_right_pieces_own() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/R3KN1R".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(60,51, true), Standard(60,52, true), Standard(60,53, true),
            Standard(60,59, true),
            Castle(56, 59, 60, 58, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_white_cant_right_pieces_opponent() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/R3Kn1R".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(60,51, true), Standard(60,52, true), Standard(60,53, true),
            Standard(60,59, true), Standard(60,61, true),
            Castle(56, 59, 60, 58, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_white_cant_left_pieces_own() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/RB2K2R".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(60,51, true), Standard(60,52, true), Standard(60,53, true),
            Standard(60,59, true), Standard(60,61, true),
            Castle(63, 61, 60, 62, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_white_cant_left_pieces_opponent() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/Rb2K2R".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(60,51, true), Standard(60,52, true), Standard(60,53, true),
            Standard(60,59, true), Standard(60,61, true),
            Castle(63, 61, 60, 62, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_white_cant_left_square_attacked() {
        let game = Game::new_from_string("3r4/8/8/8/8/8/8/R3K2R".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(60,51, true), Standard(60,52, true), Standard(60,53, true),
            Standard(60,59, true), Standard(60,61, true),
            Castle(63, 61, 60, 62, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_white_can_castle_only_rook_moved() {
        let game = Game::new_from_string("1r6/8/8/8/8/8/8/R3K2R".to_string(), true);
        let mut all_moves = game.get_all_moves();


        let mut expected_moves = vec![
            Standard(60,51, true), Standard(60,52, true), Standard(60,53, true),
            Standard(60,59, true), Standard(60,61, true),
            Castle(63, 61, 60, 62, true),
            Castle(56, 59, 60, 58, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_white_cant_in_check() {
        let game = Game::new_from_string("8/4r3/8/8/8/8/8/R3K2R".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(60,51, true), Standard(60,52, true), Standard(60,53, true),
            Standard(60,59, true), Standard(60,61, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_black() {
        let game = Game::new_from_string("r3k2r/8/8/8/8/8/8/8".to_string(), false);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(4, 3, false), Standard(4, 5, false), Standard(4, 11, false),
            Standard(4, 12, false), Standard(4, 13, false),
            Castle(0, 3, 4, 2, false),
            Castle(7, 5, 4, 6, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }


    #[test]
    fn test_castle_black_cant_right_pieces_own() {
        let game = Game::new_from_string("r3kn1r/8/8/8/8/8/8/8".to_string(), false);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(4,3, false), Standard(4,11, false),
            Standard(4,12, false), Standard(4,13, false),
            Castle(0, 3, 4, 2, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_black_cant_right_pieces_opponent() {
        let game = Game::new_from_string("r3kN1r/8/8/8/8/8/8/8".to_string(), false);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(4,3, false), Standard(4,5, false), Standard(4,11, false),
            Standard(4,12, false), Standard(4,13, false),
            Castle(0, 3, 4, 2, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_black_cant_left_pieces_own() {
        let game = Game::new_from_string("rb2k2r/8/8/8/8/8/8/8".to_string(), false);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(4,3, false), Standard(4,5, false), Standard(4,11, false),
            Standard(4,12, false), Standard(4,13, false),
            Castle(7, 5, 4, 6, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_black_cant_left_pieces_opponent() {
        let game = Game::new_from_string("rB2k2r/8/8/8/8/8/8/8".to_string(), false);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(4,3, false), Standard(4,5, false), Standard(4,11, false),
            Standard(4,12, false), Standard(4,13, false),
            Castle(7, 5, 4, 6, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_black_cant_left_square_attacked() {
        let game = Game::new_from_string("r3k2r/8/8/8/8/8/8/3R4".to_string(), false);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(4,3, false), Standard(4,5, false), Standard(4,11, false),
            Standard(4,12, false), Standard(4,13, false),
            Castle(7, 5, 4, 6, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_black_can_castle_only_rook_moved() {
        let game = Game::new_from_string("r3k2r/8/8/8/8/8/8/1R6".to_string(), false);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(4,3, false), Standard(4,5, false), Standard(4,11, false),
            Standard(4,12, false), Standard(4,13, false),
            Castle(7, 5, 4, 6, false),
            Castle(0, 3, 4, 2, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_castle_black_cant_in_check() {
        let game = Game::new_from_string("r3k2r/8/8/8/8/8/4R3/8".to_string(), false);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(4,3, false), Standard(4,5, false), Standard(4,11, false),
            Standard(4,12, false), Standard(4,13, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_standard() {
        let mut game = Game::new_from_string("8/8/8/3r4/8/8/8/8".to_string(), false);
        let mut all_moves = game.get_all_moves();

        assert_eq!(game.board.board_value, 134_217_728);

        let moved = game.make_move(&debug::find_specific_move(&all_moves, 27, 3));

        assert_eq!(moved, true);

        assert_eq!(game.board.board_state[27], None);
        assert_eq!(game.board.board_state[3], Some(Piece {
            piece_type: Rook,
            is_white: false
        }));

        assert_eq!(game.board.board_value, 8);
    }

    #[test]
    fn test_move_castle() {
        let mut game = Game::new_from_string("8/8/8/8/8/8/8/R3K3".to_string(), true);
        let mut all_moves = game.get_all_moves();

        assert_eq!(game.board.board_value, 1_224_979_098_644_774_912);

        let moved = game.make_move(&debug::find_specific_move(&all_moves, 60, 58));

        assert_eq!(moved, true);

        assert_eq!(game.board.board_state[60], None);
        assert_eq!(game.board.board_state[58], Some(Piece {
            piece_type: King,
            is_white: true
        }));

        assert_eq!(game.board.board_state[56], None);
        assert_eq!(game.board.board_state[59], Some(Piece {
            piece_type: Rook,
            is_white: true
        }));

        assert_eq!(game.board.board_value, 864_691_128_455_135_232);
    }

    #[test]
    fn test_move_bishop() {
        let mut game = Game::new_from_string("8/8/3B4/8/8/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(19,10, true), Standard(19,1, true),
            Standard(19,12, true), Standard(19,5, true),
            Standard(19,28, true), Standard(19,37, true),
            Standard(19,46, true), Standard(19,55, true),
            Standard(19,26, true), Standard(19,33, true),
            Standard(19,40, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 19);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }


    #[test]
    fn test_move_bishop2() {
        let mut game = Game::new_from_string("8/8/8/8/6B1/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 38;

        let mut expected_moves = vec![
            Standard(indeks,31, true), Standard(indeks,47, true),
            Standard(indeks,29, true), Standard(indeks,20, true),
            Standard(indeks,11, true), Standard(indeks,2, true),
            Standard(indeks,45, true), Standard(indeks,52, true),
            Standard(indeks,59, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_bishop_top_left_corner() {
        let mut game = Game::new_from_string("B7/8/8/8/8/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(0,9, true), Standard(0,18, true),
            Standard(0,27, true), Standard(0,36, true),
            Standard(0,45, true), Standard(0,54, true),
            Standard(0,63, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 0);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_bishop_top_right_corner() {
        let mut game = Game::new_from_string("7B/8/8/8/8/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(7,14, true), Standard(7,21, true),
            Standard(7,28, true), Standard(7,35, true),
            Standard(7,42, true), Standard(7,49, true),
            Standard(7,56, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 7);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }


    #[test]
    fn test_move_bishop_low_right_corner() {
        let mut game = Game::new_from_string("8/8/8/8/8/8/8/7B".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(63,9, true), Standard(63,18, true),
            Standard(63,27, true), Standard(63,36, true),
            Standard(63,45, true), Standard(63,54, true),
            Standard(63,0, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 63);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }


    #[test]
    fn test_move_bishop_low_left_corner() {
        let mut game = Game::new_from_string("8/8/8/8/8/8/8/B7".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Standard(56,14, true), Standard(56,21, true),
            Standard(56,28, true), Standard(56,35, true),
            Standard(56,42, true), Standard(56,49, true),
            Standard(56,7, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 56);
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }


    #[test]
    fn test_move_knight() {
        let mut game = Game::new_from_string("8/8/8/8/4N3/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 36;

        let mut expected_moves = vec![
            Standard(indeks,19, true), Standard(indeks,21, true),
            Standard(indeks,26, true), Standard(indeks,30, true),
            Standard(indeks,42, true), Standard(indeks,46, true),
            Standard(indeks,51, true), Standard(indeks,53, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_knight2() {
        let mut game = Game::new_from_string("8/N7/8/8/8/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 8;

        let mut expected_moves = vec![
            Standard(indeks,2, true), Standard(indeks,18, true),
            Standard(indeks,25, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_knight3() {
        let mut game = Game::new_from_string("8/8/8/N7/8/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 24;

        let mut expected_moves = vec![
            Standard(indeks,18, true), Standard(indeks,34, true),
            Standard(indeks,41, true), Standard(indeks,9, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_knight4() {
        let mut game = Game::new_from_string("8/8/8/8/1N6/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 33;

        let mut expected_moves = vec![
            Standard(indeks,16, true), Standard(indeks,18, true),
            Standard(indeks,27, true), Standard(indeks,43, true),
            Standard(indeks,48, true), Standard(indeks,50, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_knight5() {
        let mut game = Game::new_from_string("8/8/8/7N/8/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 31;

        let mut expected_moves = vec![
            Standard(indeks,14, true), Standard(indeks,21, true),
            Standard(indeks,37, true), Standard(indeks,46, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_knight6() {
        let mut game = Game::new_from_string("8/8/8/8/6N1/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 38;

        let mut expected_moves = vec![
            Standard(indeks,21, true), Standard(indeks,23, true),
            Standard(indeks,28, true), Standard(indeks,44, true),
            Standard(indeks,53, true), Standard(indeks,55, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_knight_top_left_corner() {
        let mut game = Game::new_from_string("N7/8/8/8/8/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 0;

        let mut expected_moves = vec![
            Standard(indeks,10, true), Standard(indeks,17, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_knight_top_right_corner() {
        let mut game = Game::new_from_string("7N/8/8/8/8/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 7;

        let mut expected_moves = vec![
            Standard(indeks,13, true), Standard(indeks,22, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_knight_bottom_left_corner() {
        let mut game = Game::new_from_string("8/8/8/8/8/8/8/N7".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 56;

        let mut expected_moves = vec![
            Standard(indeks,41, true), Standard(indeks,50, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_knight_bottom_right_corner() {
        let mut game = Game::new_from_string("8/8/8/8/8/8/8/7N".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 63;

        let mut expected_moves = vec![
            Standard(indeks,46, true), Standard(indeks,53, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_queen() {
        let mut game = Game::new_from_string("8/8/8/3Q4/8/8/8/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 27;

        let mut expected_moves = vec![
            Standard(indeks,18, true), Standard(indeks,9, true),
            Standard(indeks,0, true), Standard(indeks,36, true),
            Standard(indeks,45, true), Standard(indeks,54, true),
            Standard(indeks,63, true), Standard(indeks,20, true),
            Standard(indeks,13, true), Standard(indeks,6, true),
            Standard(indeks,34, true), Standard(indeks,41, true),
            Standard(indeks,48, true),
            Standard(indeks,19, true), Standard(indeks,11, true),
            Standard(indeks,3, true), Standard(indeks,35, true),
            Standard(indeks,43, true), Standard(indeks,51, true),
            Standard(indeks,59, true), Standard(indeks,26, true),
            Standard(indeks,25, true), Standard(indeks,24, true),
            Standard(indeks,28, true), Standard(indeks,29, true),
            Standard(indeks,30, true), Standard(indeks,31, true),

        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
    }

    #[test]
    fn test_move_queen2() {
        let mut game = Game::new_from_string("8/8/8/8/8/8/Q7/8".to_string(), true);
        let mut all_moves = game.get_all_moves();

        let indeks = 48;

        let mut expected_moves = vec![
            Standard(indeks,56, true), Standard(indeks,40, true),
            Standard(indeks,32, true), Standard(indeks,24, true),
            Standard(indeks,16, true), Standard(indeks,8, true),
            Standard(indeks,0, true), Standard(indeks,49, true),
            Standard(indeks,50, true), Standard(indeks,51, true),
            Standard(indeks,52, true), Standard(indeks,53, true),
            Standard(indeks,54, true), Standard(indeks,55, true),

            Standard(indeks,57, true), Standard(indeks,41, true),
            Standard(indeks,34, true), Standard(indeks,27, true),
            Standard(indeks,20, true), Standard(indeks,13, true),
            Standard(indeks,6, true)

        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(x, expected_moves);
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
    }

    #[test]
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
