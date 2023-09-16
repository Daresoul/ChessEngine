pub mod game;
pub mod board;
mod piece;
mod debug;
pub mod debug_structs;

/*
Calculate moves for pieces only that are affected by the precious move.
That means, if u move a piece, go through all moves, and see which pieces are affected.
Remove their moves from the list of moves.
Then recalculate the moves for those pieces.
Might be worse for some pieces, but for most will be better.
U know what piece made the move from the from position, and its piece type.
U know who is affected if a move from that piece, has an attack or move or capture on the piece.
Might be necessary to calculate when moves are stopped, so if hit own piece make a moves stopped move, to that
piece as otherwise that piece wont be in the move list.
 */

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
     develop rest of simple moves: Done

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
    use crate::board::board::{Board, MoveType};
    use crate::piece::piece::{Piece};
    use crate::board::board::MoveType::{Castle, FutureMove, Attack, Capture, Standard, Defend};
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

        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,27, 18, true, true), Attack(King,27, 19, true, true),
            Attack(King,27, 20, true, true), Attack(King,27, 26, true, true),
            Attack(King,27, 28, true, true), Attack(King,27, 34, true, true),
            Attack(King,27, 35, true, true), Attack(King,27, 36, true, true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn king_move_top_right_corner() {
        let game = Game::new_from_string("7K/8/8/8/8/8/8/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,7,6, true,true),
            Attack(King, 7,14, true, true),
            Attack(King,7,15, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 7);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn king_move_bottom_right_corner() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/7K".to_string(), true);

        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,63, 54, true,true), Attack(King,63, 55, true,true), Attack(King,63, 62, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 63);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn king_move_bottom_left_corner() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/K7".to_string(), true);

        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,56,48, true,true), Attack(King,56,49, true,true), Attack(King,56,57, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 56);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn king_move_top_left_corner() {
        let game = Game::new_from_string("K7/8/8/8/8/8/8/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,0,1, true,true),
            Attack(King, 0,8, true,true),
            Attack(King, 0,9, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 0);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn king_move_cant_take() {
        let game = Game::new_from_string("8/8/2PPP3/2PKP3/2PPP3/8/8/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let mut expected_moves: Vec<MoveType> = vec![];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn king_move_take() {
        let game = Game::new_from_string("8/8/2ppp3/2pKp3/2ppp3/8/8/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Capture(King,27,18, Pawn,true),
            Capture(King,27,19, Pawn,true),
            Capture(King,27,20, Pawn,true),
            Capture(King,27,26, Pawn,true),
            Capture(King,27,28, Pawn,true),
            Capture(King,27,34, Pawn,true),
            Capture(King,27,35, Pawn,true),
            Capture(King,27,36, Pawn,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn pawn_move_single_double_white() {
        let game = Game::new_from_string("8/3p4/8/8/8/8/3P4/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let index = 51;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 42, false, true),
            Attack(Pawn, index, 44, false, true),
            Standard(index, 43, false),
            Standard(index, 35, false),
        ];

        x.sort();
        expected_moves.sort();
    }

    #[test]
    fn pawn_move_single_double_black() {
        let game = Game::new_from_string("8/3p4/8/8/8/8/3P4/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let index = 11;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 18, false, true),
            Attack(Pawn, index, 20, false, true),
            Standard(index, 19, false),
            Standard(index, 27, false),
        ];

        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn pawn_move_all_moves_available() {
        let game = Game::new_from_string("8/3p4/2P1P3/8/8/2p1p3/3P4/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let index = 51;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 42, true, true),
            Attack(Pawn, index, 44, true, true),
            Standard(index, 43, false),
            Standard(index, 35, false),
        ];

        x.sort();
        expected_moves.sort();


        let index = 11;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 18, true, true),
            Attack(Pawn, index, 20, true, true),
            Standard(index, 19, false),
            Standard(index, 27, false),
        ];

        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn pawn_move_cant_take_allied_pieces() {
        let game = Game::new_from_string("8/3p4/2p1p3/8/8/2P1P3/3P4/8".to_string(), false);

        let all_moves = game.get_all_moves();

        let index = 51;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Defend(Pawn, index, 42, Pawn, true),
            Defend(Pawn, index, 44, Pawn, true),
            Standard(index, 43, false),
            Standard(index, 35, false),
        ];

        x.sort();
        expected_moves.sort();
        assert_eq!(expected_moves, x);


        let index = 11;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Defend(Pawn, index, 18, Pawn, true),
            Defend(Pawn, index, 20, Pawn, true),
            Standard(index, 19, false),
            Standard(index, 27, false),
        ];

        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn pawn_move_can_take_outside_start_squares() {
        let game = Game::new_from_string("8/8/3p4/2P1P3/2p1p3/3P4/8/8".to_string(), false);

        let all_moves = game.get_all_moves();

        let index = 43;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 34, true, true),
            Attack(Pawn, index, 36, true, true),
            Standard(index, 35, false),
        ];

        x.sort();
        expected_moves.sort();
        assert_eq!(expected_moves, x);


        let index = 19;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 26, true, true),
            Attack(Pawn, index, 28, true, true),
            Standard(index, 27, false),
        ];

        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn pawn_move_cant_move_double() {
        let game = Game::new_from_string("8/8/3p4/8/8/3P4/8/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let index = 43;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 34, false, true),
            Attack(Pawn, index, 36, false, true),
            Standard(index, 35, false),
        ];

        x.sort();
        expected_moves.sort();
        assert_eq!(expected_moves, x);


        let index = 19;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 26, false, true),
            Attack(Pawn, index, 28, false, true),
            Standard(index, 27, false),
        ];

        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn pawn_cant_attack_outside_board_white_left() {
        let game = Game::new_from_string("8/8/8/8/8/P7/8/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let index = 40;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 33, false, true),
            Standard(index, 32, false),
        ];

        x.sort();
        expected_moves.sort();
        assert_eq!(expected_moves, x);
    }

    #[test]
    fn pawn_cant_attack_outside_board_white_right() {

        let game = Game::new_from_string("8/8/8/8/8/7P/8/8".to_string(), true);

        let all_moves = game.get_all_moves();
        let index = 47;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 38, false, true),
            Standard(index, 39, false),
        ];

        x.sort();
        expected_moves.sort();
        assert_eq!(expected_moves, x);
    }


    #[test]
    fn pawn_cant_attack_outside_board_black_left() {
        let game = Game::new_from_string("8/8/p7/8/8/8/8/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let index = 16;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 25, false, true),
            Standard(index, 24, false),
        ];

        x.sort();
        expected_moves.sort();
        assert_eq!(expected_moves, x);
    }

    #[test]
    fn pawn_cant_attack_outside_board_black_right() {
        let game = Game::new_from_string("8/8/7p/8/8/8/8/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let index = 23;

        let mut x = debug::get_all_from_position(&all_moves, usize::from(index));

        let mut expected_moves = vec![
            Attack(Pawn, index, 30, false, true),
            Standard(index, 31, false),
        ];

        x.sort();
        expected_moves.sort();
        assert_eq!(expected_moves, x);
    }

    #[test]
    fn rook_move_nothing() {
        let game = Game::new_from_string("8/8/8/3R4/8/8/8/8".to_string(), true);

        let all_moves = game.get_all_moves();


        let mut expected_moves = vec![
            Attack(Rook,27,3, true, true),
            Attack(Rook,27,11, true,true),
            Attack(Rook,27,19, true,true),
            Attack(Rook,27,35, true,true),
            Attack(Rook,27,43, true,true),
            Attack(Rook,27,51, true,true),
            Attack(Rook,27,59, true,true),
            Attack(Rook,27,24, true,true),
            Attack(Rook,27,25, true,true),
            Attack(Rook,27,26, true,true),
            Attack(Rook,27,28, true,true),
            Attack(Rook,27,29, true,true),
            Attack(Rook,27,30, true,true),
            Attack(Rook,27,31, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn rook_move_1_piece_opponent() {
        let game = Game::new_from_string("8/8/3p4/2pR1p2/8/3p4/8/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            FutureMove(Rook,27,3, true),
            FutureMove(Rook,27,11, true),
            Attack(Rook,27,19, true,true),
            Attack(Rook,27,35, true,true),
            Attack(Rook,27,43, true,true),
            FutureMove(Rook,27,51, true),
            FutureMove(Rook,27,59, true),
            FutureMove(Rook,27,24, true),
            FutureMove(Rook,27,25, true),
            Attack(Rook,27,26, true,true),
            Attack(Rook,27,28, true,true),
            Attack(Rook,27,29, true,true),
            FutureMove(Rook,27,30, true), FutureMove(Rook,27,31, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn rook_move_2_piece_opponent() {
        let game = Game::new_from_string("8/3p4/3p4/1ppR1pp1/8/3p4/3p4/8".to_string(), true);

        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            FutureMove(Rook,27,11, true),
            Attack(Rook,27,19, true,true),
            Attack(Rook,27,35, true,true),
            Attack(Rook,27,43, true,true),
            FutureMove(Rook,27,51, true),
            FutureMove(Rook,27,25, true),
            Attack(Rook,27,26, true,true),
            Attack(Rook,27,28, true,true),
            Attack(Rook,27,29, true,true),
            FutureMove(Rook,27,30, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 27);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_transform_attack() {
        let state = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);
        assert_eq!(state.board.board_state, debug_structs::get_normal_board());
    }

    #[test]
    fn test_castle_white() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/R3K2R".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,60,51, true, true),
            Attack(King,60,52, true,true),
            Attack(King,60,53, true,true),
            Attack(King,60,59, true,true),
            Attack(King,60,61, true,true),
            Castle(56, 59, 60, 58, true),
            Castle(63, 61, 60, 62, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }


    #[test]
    fn test_castle_white_cant_right_pieces_own() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/R3KN1R".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,60,51, true,true),
            Attack(King,60,52, true,true),
            Attack(King,60,53, true,true),
            Attack(King,60,59, true,true),
            Castle(56, 59, 60, 58, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_white_cant_right_pieces_opponent() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/R3Kn1R".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,60,51, true,true),
            Attack(King,60,52, true,true),
            Attack(King,60,53, true,true),
            Attack(King,60,59, true,true),
            Capture(King,60,61, Knight,true),
            Castle(56, 59, 60, 58, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_white_cant_left_pieces_own() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/RB2K2R".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,60,51, true, true),
            Attack(King,60,52, true, true),
            Attack(King,60,53, true, true),
            Attack(King,60,59, true, true),
            Attack(King,60,61, true, true),
            Castle(63, 61, 60, 62, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_white_cant_left_pieces_opponent() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/Rb2K2R".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,60,51, true,true),
            Attack(King,60,52, true,true),
            Attack(King,60,53, true,true),
            Attack(King,60,59, true,true),
            Attack(King,60,61, true,true),
            Castle(63, 61, 60, 62, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_white_cant_left_square_attacked() {
        let game = Game::new_from_string("3r4/8/8/8/8/8/8/R3K2R".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,60,51, true,true),
            Attack(King,60,52, true,true),
            Attack(King,60,53, true,true),
            Attack(King,60,59, true,true),
            Attack(King,60,61, true,true),
            Castle(63, 61, 60, 62, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_white_can_castle_only_rook_moved() {
        let game = Game::new_from_string("1r6/8/8/8/8/8/8/R3K2R".to_string(), true);
        let all_moves = game.get_all_moves();


        let mut expected_moves = vec![
            Attack(King,60,51, true,true),
            Attack(King,60,52, true,true),
            Attack(King,60,53, true,true),
            Attack(King,60,59, true,true),
            Attack(King,60,61, true,true),
            Castle(63, 61, 60, 62, true),
            Castle(56, 59, 60, 58, true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_white_cant_in_check() {
        let game = Game::new_from_string("8/4r3/8/8/8/8/8/R3K2R".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,60,51, true,true),
            Attack(King,60,52, true,true),
            Attack(King,60,53, true,true),
            Attack(King,60,59, true,true),
            Attack(King,60,61, true,true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 60);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_black() {
        let game = Game::new_from_string("r3k2r/8/8/8/8/8/8/8".to_string(), false);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,4, 3, true,false),
            Attack(King,4, 5, true,false),
            Attack(King,4, 11, true,false),
            Attack(King,4, 12, true,false),
            Attack(King,4, 13, true,false),
            Castle(0, 3, 4, 2, false),
            Castle(7, 5, 4, 6, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }


    #[test]
    fn test_castle_black_cant_right_pieces_own() {
        let game = Game::new_from_string("r3kn1r/8/8/8/8/8/8/8".to_string(), false);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,4,3, true,false),
            Attack(King,4,11, true,false),
            Attack(King,4,12, true,false),
            Attack(King,4,13, true,false),
            Castle(0, 3, 4, 2, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_black_cant_right_pieces_opponent() {
        let game = Game::new_from_string("r3kN1r/8/8/8/8/8/8/8".to_string(), false);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,4,3, true,false),
            Capture(King,4,5, Knight,false),
            Attack(King,4,11, true,false),
            Attack(King,4,12, true,false),
            Attack(King,4,13, true,false),
            Castle(0, 3, 4, 2, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_black_cant_left_pieces_own() {
        let game = Game::new_from_string("rb2k2r/8/8/8/8/8/8/8".to_string(), false);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,4,3, true,false),
            Attack(King,4,5, true,false),
            Attack(King,4,11, true,false),
            Attack(King,4,12, true,false),
            Attack(King,4,13, true,false),
            Castle(7, 5, 4, 6, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_black_cant_left_pieces_opponent() {
        let game = Game::new_from_string("rB2k2r/8/8/8/8/8/8/8".to_string(), false);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,4,3, true,false),
            Attack(King,4,5, true,false),
            Attack(King,4,11, true,false),
            Attack(King,4,12, true,false),
            Attack(King,4,13, true,false),
            Castle(7, 5, 4, 6, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_black_cant_left_square_attacked() {
        let game = Game::new_from_string("r3k2r/8/8/8/8/8/8/3R4".to_string(), false);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,4,3, true,false),
            Attack(King,4,5, true,false),
            Attack(King,4,11, true,false),
            Attack(King,4,12, true,false),
            Attack(King,4,13, true,false),
            Castle(7, 5, 4, 6, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_black_can_castle_only_rook_moved() {
        let game = Game::new_from_string("r3k2r/8/8/8/8/8/8/1R6".to_string(), false);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,4,3, true,false),
            Attack(King,4,5, true,false),
            Attack(King,4,11, true,false),
            Attack(King,4,12, true,false),
            Attack(King,4,13, true,false),
            Castle(7, 5, 4, 6, false),
            Castle(0, 3, 4, 2, false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_castle_black_cant_in_check() {
        let game = Game::new_from_string("r3k2r/8/8/8/8/8/4R3/8".to_string(), false);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(King,4,3, true,false),
            Attack(King,4,5, true,false),
            Attack(King,4,11, true,false),
            Attack(King,4,12, true,false),
            Attack(King,4,13, true,false)
        ];

        let mut x = debug::get_all_from_position(&all_moves, 4);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_attack() {
        let mut game = Game::new_from_string("8/8/8/3r4/8/8/8/8".to_string(), false);
        let all_moves = game.get_all_moves();

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
        let all_moves = game.get_all_moves();

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
        let game = Game::new_from_string("8/8/3B4/8/8/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(Bishop,19,10, true,true),
            Attack(Bishop,19,1, true,true),
            Attack(Bishop,19,12, true,true),
            Attack(Bishop,19,5, true,true),
            Attack(Bishop,19,28, true,true),
            Attack(Bishop,19,37, true,true),
            Attack(Bishop,19,46, true,true),
            Attack(Bishop,19,55, true,true),
            Attack(Bishop,19,26, true,true),
            Attack(Bishop,19,33, true,true),
            Attack(Bishop,19,40, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 19);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }


    #[test]
    fn test_move_bishop2() {
        let game = Game::new_from_string("8/8/8/8/6B1/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 38;

        let mut expected_moves = vec![
            Attack(Bishop,indeks,31, true, true),
            Attack(Bishop,indeks,47, true,true),
            Attack(Bishop,indeks,29, true,true),
            Attack(Bishop,indeks,20, true,true),
            Attack(Bishop,indeks,11, true,true),
            Attack(Bishop,indeks,2, true,true),
            Attack(Bishop,indeks,45, true,true),
            Attack(Bishop,indeks,52, true,true),
            Attack(Bishop,indeks,59, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_bishop_top_left_corner() {
        let game = Game::new_from_string("B7/8/8/8/8/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(Bishop,0,9, true,true),
            Attack(Bishop,0,18, true,true),
            Attack(Bishop,0,27, true,true),
            Attack(Bishop,0,36, true,true),
            Attack(Bishop,0,45, true,true),
            Attack(Bishop,0,54, true,true),
            Attack(Bishop,0,63, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 0);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_bishop_top_right_corner() {
        let game = Game::new_from_string("7B/8/8/8/8/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(Bishop,7,14, true,true),
            Attack(Bishop,7,21, true,true),
            Attack(Bishop,7,28, true,true),
            Attack(Bishop,7,35, true,true),
            Attack(Bishop,7,42, true,true),
            Attack(Bishop,7,49, true,true),
            Attack(Bishop,7,56, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 7);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }


    #[test]
    fn test_move_bishop_low_right_corner() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/7B".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(Bishop,63,9, true,true),
            Attack(Bishop,63,18, true,true),
            Attack(Bishop,63,27, true,true),
            Attack(Bishop,63,36, true,true),
            Attack(Bishop,63,45, true,true),
            Attack(Bishop,63,54, true,true),
            Attack(Bishop,63,0, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 63);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }


    #[test]
    fn test_move_bishop_low_left_corner() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/B7".to_string(), true);
        let all_moves = game.get_all_moves();

        let mut expected_moves = vec![
            Attack(Bishop,56,14, true,true),
            Attack(Bishop,56,21, true,true),
            Attack(Bishop,56,28, true,true),
            Attack(Bishop,56,35, true,true),
            Attack(Bishop,56,42, true,true),
            Attack(Bishop,56,49, true,true),
            Attack(Bishop,56,7,  true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, 56);
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }


    #[test]
    fn test_move_knight() {
        let game = Game::new_from_string("8/8/8/8/4N3/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 36;

        let mut expected_moves = vec![
            Attack(Knight, indeks,19, true,true),
            Attack(Knight, indeks,21, true,true),
            Attack(Knight, indeks,26, true,true),
            Attack(Knight, indeks,30, true,true),
            Attack(Knight, indeks,42, true,true),
            Attack(Knight, indeks,46, true,true),
            Attack(Knight, indeks,51, true,true),
            Attack(Knight, indeks,53, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_knight2() {
        let game = Game::new_from_string("8/N7/8/8/8/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 8;

        let mut expected_moves = vec![
            Attack(Knight, indeks,2, true,true),
            Attack(Knight, indeks,18, true,true),
            Attack(Knight, indeks,25, true,true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_knight3() {
        let game = Game::new_from_string("8/8/8/N7/8/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 24;

        let mut expected_moves = vec![
            Attack(Knight, indeks,18, true,true),
            Attack(Knight, indeks,34, true,true),
            Attack(Knight, indeks,41, true,true),
            Attack(Knight, indeks,9, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_knight4() {
        let game = Game::new_from_string("8/8/8/8/1N6/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 33;

        let mut expected_moves = vec![
            Attack(Knight, indeks,16, true,true),
            Attack(Knight, indeks,18, true,true),
            Attack(Knight, indeks,27, true,true),
            Attack(Knight, indeks,43, true,true),
            Attack(Knight, indeks,48, true,true),
            Attack(Knight, indeks,50, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_knight5() {
        let game = Game::new_from_string("8/8/8/7N/8/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 31;

        let mut expected_moves = vec![
            Attack(Knight, indeks,14, true,true),
            Attack(Knight, indeks,21, true,true),
            Attack(Knight, indeks,37, true,true),
            Attack(Knight, indeks,46, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_knight6() {
        let game = Game::new_from_string("8/8/8/8/6N1/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 38;

        let mut expected_moves = vec![
            Attack(Knight, indeks,21, true,true),
            Attack(Knight, indeks,23, true,true),
            Attack(Knight, indeks,28, true,true),
            Attack(Knight, indeks,44, true,true),
            Attack(Knight, indeks,53, true,true),
            Attack(Knight, indeks,55, true,true),
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_knight_top_left_corner() {
        let game = Game::new_from_string("N7/8/8/8/8/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 0;

        let mut expected_moves = vec![
            Attack(Knight, indeks,10, true,true),
            Attack(Knight, indeks,17, true,true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_knight_top_right_corner() {
        let game = Game::new_from_string("7N/8/8/8/8/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 7;

        let mut expected_moves = vec![
            Attack(Knight, indeks,13, true,true),
            Attack(Knight, indeks,22, true,true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_knight_bottom_left_corner() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/N7".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 56;

        let mut expected_moves = vec![
            Attack(Knight, indeks,41, true,true),
            Attack(Knight, indeks,50, true,true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_knight_bottom_right_corner() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/7N".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 63;

        let mut expected_moves = vec![
            Attack(Knight, indeks,46, true,true),
            Attack(Knight, indeks,53, true,true)
        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_queen() {
        let game = Game::new_from_string("8/8/8/3Q4/8/8/8/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 27;

        let mut expected_moves = vec![
            Attack(Queen,indeks,18, true,true),
            Attack(Queen,indeks,9, true,true),
            Attack(Queen,indeks,0, true,true),
            Attack(Queen,indeks,36, true,true),
            Attack(Queen,indeks,45, true,true),
            Attack(Queen,indeks,54, true,true),
            Attack(Queen,indeks,63, true,true),
            Attack(Queen,indeks,20, true,true),
            Attack(Queen,indeks,13, true,true),
            Attack(Queen,indeks,6, true,true),
            Attack(Queen,indeks,34, true,true),
            Attack(Queen,indeks,41, true,true),
            Attack(Queen,indeks,48, true,true),
            Attack(Queen,indeks,19, true,true),
            Attack(Queen,indeks,11, true,true),
            Attack(Queen,indeks,3, true,true),
            Attack(Queen,indeks,35, true,true),
            Attack(Queen,indeks,43, true,true),
            Attack(Queen,indeks,51, true,true),
            Attack(Queen,indeks,59, true,true),
            Attack(Queen,indeks,26, true,true),
            Attack(Queen,indeks,25, true,true),
            Attack(Queen,indeks,24, true,true),
            Attack(Queen,indeks,28, true,true),
            Attack(Queen,indeks,29, true,true),
            Attack(Queen,indeks,30, true,true),
            Attack(Queen,indeks,31, true,true),

        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }

    #[test]
    fn test_move_queen2() {
        let game = Game::new_from_string("8/8/8/8/8/8/Q7/8".to_string(), true);
        let all_moves = game.get_all_moves();

        let indeks = 48;

        let mut expected_moves = vec![
            Attack(Queen,indeks,56, true,true),
            Attack(Queen,indeks,40, true,true),
            Attack(Queen,indeks,32, true,true),
            Attack(Queen,indeks,24, true,true),
            Attack(Queen,indeks,16, true,true),
            Attack(Queen,indeks,8, true,true),
            Attack(Queen,indeks,0, true,true),
            Attack(Queen,indeks,49, true,true),
            Attack(Queen,indeks,50, true,true),
            Attack(Queen,indeks,51, true,true),
            Attack(Queen,indeks,52, true,true),
            Attack(Queen,indeks,53, true,true),
            Attack(Queen,indeks,54, true,true),
            Attack(Queen,indeks,55, true,true),

            Attack(Queen,indeks,57, true,true),
            Attack(Queen,indeks,41, true,true),
            Attack(Queen,indeks,34, true,true),
            Attack(Queen,indeks,27, true,true),
            Attack(Queen,indeks,20, true,true),
            Attack(Queen,indeks,13, true,true),
            Attack(Queen,indeks,6, true,true)

        ];

        let mut x = debug::get_all_from_position(&all_moves, usize::from(indeks));
        x.sort();
        expected_moves.sort();

        assert_eq!(expected_moves, x);
    }
}
