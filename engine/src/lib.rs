pub mod game;
pub mod board;
pub mod debug;
mod eval_board;
mod utils;
mod magic;
mod move_gen;

mod move_list;

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
    use crate::board::board::Move;
    use crate::board::board::Move::{Promotion, Standard};
    use crate::debug::debug::{print_bitboard_board, print_board};
    use crate::game::game::Game;
    use crate::move_gen::move_gen::Direction::{East, North, South, West};
    use crate::move_gen::move_gen::{MoveGen, PieceType};
    use crate::move_gen::move_gen::PieceType::{BISHOP, KNIGHT, PAWN, QUEEN, ROOK};
    use crate::utils::utils;

    #[test]
    fn pawn_move_single_double_white() {
        let mut game = Game::new_from_string("8/3p4/8/8/8/8/3P4/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(51, 35, PAWN, true),
            Standard(51, 43, PAWN, true),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);
        //assert_eq!(expected_black_moves, black_moves);
    }

    #[test]
    fn pawn_move_cant_take_allied_pieces() {
        let mut game = Game::new_from_string("8/3p4/2p1p3/8/8/2P1P3/3P4/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(42, 34, PAWN, true),
            Standard(51, 35, PAWN, true),
            Standard(44, 36, PAWN, true),
            Standard(51, 43, PAWN, true),
        ];

        moves.sort();
        expected_white_moves.sort();

        assert_eq!(expected_white_moves, moves);
    }

    #[test]
    fn pawn_move_can_take_outside_start_squares() {
        let mut game = Game::new_from_string("8/8/3p4/2P1P3/2p1p3/3P4/8/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(43, 35, PAWN, true),
            Standard(43, 34, PAWN, true),
            Standard(43, 36, PAWN, true),
            Standard(26, 18, PAWN, true),
            Standard(26, 19, PAWN, true),
            Standard(28, 20, PAWN, true),
            Standard(28, 19, PAWN, true),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);

    }

    #[test]
    fn pawn_on_edge() {
        let mut game = Game::new_from_string("1p4p1/P6P/8/8/8/8/p6p/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Promotion(8, 0, KNIGHT, PieceType::None, true),
            Promotion(8, 0, BISHOP, PieceType::None, true),
            Promotion(8, 0, QUEEN, PieceType::None, true),
            Promotion(8, 0, ROOK, PieceType::None, true),
            Promotion(8, 1, KNIGHT, PieceType::None, true),
            Promotion(8, 1, BISHOP, PieceType::None, true),
            Promotion(8, 1, QUEEN, PieceType::None, true),
            Promotion(8, 1, ROOK, PieceType::None, true),
            Promotion(15, 6, KNIGHT, PieceType::None, true),
            Promotion(15, 6, BISHOP, PieceType::None, true),
            Promotion(15, 6, QUEEN, PieceType::None, true),
            Promotion(15, 6, ROOK, PieceType::None, true),
            Promotion(15, 7, KNIGHT, PieceType::None, true),
            Promotion(15, 7, BISHOP, PieceType::None, true),
            Promotion(15, 7, QUEEN, PieceType::None, true),
            Promotion(15, 7, ROOK, PieceType::None, true),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);

    }

    #[test]
    fn pawn_move_promotion() {
        let mut game = Game::new_from_string("8/3P4/8/8/8/8/3p4/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Promotion(11, 3, KNIGHT, PieceType::None, true),
            Promotion(11, 3, BISHOP, PieceType::None, true),
            Promotion(11, 3, QUEEN, PieceType::None, true),
            Promotion(11, 3, ROOK, PieceType::None, true),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);

    }

    #[test]
    fn knight_normal_move() {
        let mut game = Game::new_from_string("8/8/3n4/8/8/3N4/8/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(43, 28, KNIGHT, true),
            Standard(43, 26, KNIGHT, true),
            Standard(43, 33, KNIGHT, true),
            Standard(43, 49, KNIGHT, true),
            Standard(43, 58, KNIGHT, true),
            Standard(43, 60, KNIGHT, true),
            Standard(43, 37, KNIGHT, true),
            Standard(43, 53, KNIGHT, true),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);
    }

    #[test]
    fn knight_take_opponent_cant_take_teamate_move() {
        let mut game = Game::new_from_string("8/8/3np3/2p1P3/2P1p3/3NP3/8/8".to_string(), true);

        let mut moves = game.get_all_moves();


        let mut expected_white_moves: Vec<Move> = vec![
            Standard(28, 19, PAWN, true),
            Standard(43, 26, KNIGHT, true),
            Standard(43, 33, KNIGHT, true),
            Standard(43, 49, KNIGHT, true),
            Standard(43, 58, KNIGHT, true),
            Standard(43, 60, KNIGHT, true),
            Standard(43, 37, KNIGHT, true),
            Standard(43, 53, KNIGHT, true),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);

    }

    #[test]
    fn knight_normal_move_corners() {
        let mut game = Game::new_from_string("N7/8/8/8/8/8/8/7N".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(0, 10, KNIGHT, true),
            Standard(0, 17, KNIGHT, true),
            Standard(63, 46, KNIGHT, true),
            Standard(63, 53, KNIGHT, true)
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);
    }

    #[test]
    fn rook_all_move_test() {
        let mut game = Game::new_from_string("8/8/8/8/3R4/8/8/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 27, ROOK, true),
            Standard(35, 19, ROOK, true),
            Standard(35, 11, ROOK, true),
            Standard(35, 3, ROOK, true),
            Standard(35, 43, ROOK, true),
            Standard(35, 51, ROOK, true),
            Standard(35, 59, ROOK, true),
            Standard(35, 34, ROOK, true),
            Standard(35, 33, ROOK, true),
            Standard(35, 32, ROOK, true),
            Standard(35, 36, ROOK, true),
            Standard(35, 37, ROOK, true),
            Standard(35, 38, ROOK, true),
            Standard(35, 39, ROOK, true),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(moves, expected_white_moves);
    }

    #[test]
    fn rook_stops_on_opponents() {
        let mut game = Game::new_from_string("8/8/3p4/8/p2Rp3/8/8/3p4".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 27, ROOK, true),
            Standard(35, 19, ROOK, true),
            Standard(35, 43, ROOK, true),
            Standard(35, 51, ROOK, true),
            Standard(35, 59, ROOK, true),
            Standard(35, 34, ROOK, true),
            Standard(35, 33, ROOK, true),
            Standard(35, 32, ROOK, true),
            Standard(35, 36, ROOK, true),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(moves, expected_white_moves);
    }

    #[test]
    fn rook_stops_on_allies() {
        let mut game = Game::new_from_string("8/8/3P4/8/P2RP3/8/8/3P4".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 27, ROOK, true),
            Standard(35, 43, ROOK, true),
            Standard(35, 51, ROOK, true),
            Standard(35, 34, ROOK, true),
            Standard(35, 33, ROOK, true),

            Standard(19, 11, PAWN, true),
            Standard(32, 24, PAWN, true),
            Standard(36, 28, PAWN, true),
            Standard(59, 51, PAWN, true)
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);
    }


    #[test]
    fn rook_top_left_corner() {
        let mut game = Game::new_from_string("R7/8/8/8/8/8/8/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(0, 1, ROOK, true),
            Standard(0, 2, ROOK, true),
            Standard(0, 3, ROOK, true),
            Standard(0, 4, ROOK, true),
            Standard(0, 5, ROOK, true),
            Standard(0, 6, ROOK, true),
            Standard(0, 7, ROOK, true),

            Standard(0, 8, ROOK, true),
            Standard(0, 16, ROOK, true),
            Standard(0, 24, ROOK, true),
            Standard(0, 32, ROOK, true),
            Standard(0, 40, ROOK, true),
            Standard(0, 48, ROOK, true),
            Standard(0, 56, ROOK, true)
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);
    }

    #[test]
    fn rook_top_right_corner() {
        let mut game = Game::new_from_string("7R/8/8/8/8/8/8/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(7, 1, ROOK, true),
            Standard(7, 2, ROOK, true),
            Standard(7, 3, ROOK, true),
            Standard(7, 4, ROOK, true),
            Standard(7, 5, ROOK, true),
            Standard(7, 6, ROOK, true),
            Standard(7, 0, ROOK, true),

            Standard(7, 15, ROOK, true),
            Standard(7, 23, ROOK, true),
            Standard(7, 31, ROOK, true),
            Standard(7, 39, ROOK, true),
            Standard(7, 47, ROOK, true),
            Standard(7, 55, ROOK, true),
            Standard(7, 63, ROOK, true)
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);
    }

    #[test]
    fn rook_bottom_left_corner() {
        let mut game = Game::new_from_string("8/8/8/8/8/8/8/R7".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(56, 57, ROOK, true),
            Standard(56, 58, ROOK, true),
            Standard(56, 59, ROOK, true),
            Standard(56, 60, ROOK, true),
            Standard(56, 61, ROOK, true),
            Standard(56, 62, ROOK, true),
            Standard(56, 63, ROOK, true),

            Standard(56, 16, ROOK, true),
            Standard(56, 24, ROOK, true),
            Standard(56, 32, ROOK, true),
            Standard(56, 40, ROOK, true),
            Standard(56, 48, ROOK, true),
            Standard(56, 0, ROOK, true),
            Standard(56, 8, ROOK, true)
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);
    }

    #[test]
    fn rook_bottom_right_corner() {
        let mut game = Game::new_from_string("8/8/8/8/8/8/8/7R".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(63, 57, ROOK, true),
            Standard(63, 58, ROOK, true),
            Standard(63, 59, ROOK, true),
            Standard(63, 60, ROOK, true),
            Standard(63, 61, ROOK, true),
            Standard(63, 62, ROOK, true),
            Standard(63, 56, ROOK, true),

            Standard(63, 7, ROOK, true),
            Standard(63, 15, ROOK, true),
            Standard(63, 23, ROOK, true),
            Standard(63, 31, ROOK, true),
            Standard(63, 39, ROOK, true),
            Standard(63, 47, ROOK, true),
            Standard(63, 55, ROOK, true)
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);
    }


    #[test]
    fn bishop_all_moves_test() {
        let mut game = Game::new_from_string("8/8/8/8/3B4/8/8/8".to_string(), true);

        let mut moves = game.get_all_moves();
        
        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 26, BISHOP, true),
            Standard(35, 17, BISHOP, true),
            Standard(35, 08, BISHOP, true),
            Standard(35, 28, BISHOP, true),
            Standard(35, 21, BISHOP, true),
            Standard(35, 14, BISHOP, true),
            Standard(35, 07, BISHOP, true),
            Standard(35, 42, BISHOP, true),
            Standard(35, 49, BISHOP, true),
            Standard(35, 56, BISHOP, true),
            Standard(35, 44, BISHOP, true),
            Standard(35, 53, BISHOP, true),
            Standard(35, 62, BISHOP, true),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);
    }

    #[test]
    fn bishop_stops_on_opponents() {
        let mut game = Game::new_from_string("8/p7/5p2/8/3B4/4p3/8/p7".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 26, BISHOP, true),
            Standard(35, 17, BISHOP, true),
            Standard(35, 08, BISHOP, true),

            Standard(35, 28, BISHOP, true),
            Standard(35, 21, BISHOP, true),

            Standard(35, 42, BISHOP, true),
            Standard(35, 49, BISHOP, true),
            Standard(35, 56, BISHOP, true),
            Standard(35, 44, BISHOP, true),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(moves, expected_white_moves);
    }

    #[test]
    fn bishop_stops_on_allies() {
        let mut game = Game::new_from_string("8/P7/5P2/8/3B4/4P3/8/P7".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 26, BISHOP, true),
            Standard(35, 17, BISHOP, true),

            Standard(35, 28, BISHOP, true),

            Standard(35, 42, BISHOP, true),
            Standard(35, 49, BISHOP, true),
            Standard(44, 36, PAWN, true),
            Standard(56, 48, PAWN, true),
            Promotion(8, 0, KNIGHT, PieceType::None, true),
            Promotion(8, 0, BISHOP, PieceType::None, true),
            Promotion(8, 0, QUEEN, PieceType::None, true),
            Promotion(8, 0, ROOK, PieceType::None, true),
            Standard(21, 13, PAWN, true)
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(moves, expected_white_moves);
    }


    #[test]
    fn queen_all_moves_test() {
        let mut game = Game::new_from_string("8/8/8/8/3Q4/8/8/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 26, QUEEN, true),
            Standard(35, 17, QUEEN, true),
            Standard(35, 08, QUEEN, true),
            Standard(35, 28, QUEEN, true),
            Standard(35, 21, QUEEN, true),
            Standard(35, 14, QUEEN, true),
            Standard(35, 07, QUEEN, true),
            Standard(35, 42, QUEEN, true),
            Standard(35, 49, QUEEN, true),
            Standard(35, 56, QUEEN, true),
            Standard(35, 44, QUEEN, true),
            Standard(35, 53, QUEEN, true),
            Standard(35, 62, QUEEN, true),

            Standard(35, 27, QUEEN, true),
            Standard(35, 19, QUEEN, true),
            Standard(35, 11, QUEEN, true),
            Standard(35, 3, QUEEN, true),
            Standard(35, 43, QUEEN, true),
            Standard(35, 51, QUEEN, true),
            Standard(35, 59, QUEEN, true),
            Standard(35, 34, QUEEN, true),
            Standard(35, 33, QUEEN, true),
            Standard(35, 32, QUEEN, true),
            Standard(35, 36, QUEEN, true),
            Standard(35, 37, QUEEN, true),
            Standard(35, 38, QUEEN, true),
            Standard(35, 39, QUEEN, true),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);
    }

    #[test]
    fn generate_rays() {
        //let pieces = Pieces::init();

        let north_ray = MoveGen::bb_ray(0, 35, North);
        assert_eq!(north_ray, 578721348210130944);
        //println!();
        //debug::debug::print_bitboard_board(&578721348210130944);

        let south_ray = MoveGen::bb_ray(0, 35, South);
        assert_eq!(south_ray, 134744072);
        //println!();
        //debug::debug::print_bitboard_board(&134744072);

        let west_ray = MoveGen::bb_ray(0, 35, East);
        assert_eq!(west_ray, 1030792151040);
        //println!();
        //debug::debug::print_bitboard_board(&1030792151040);

        let east_ray = MoveGen::bb_ray(0, 35, West);
        assert_eq!(east_ray, 30064771072);
        //println!();
        //debug::debug::print_bitboard_board(&30064771072);
    }

    #[test]
    fn check_moves_is_correct_starting_position_white() {
        let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);

        let moves = game.get_all_moves();

        assert_eq!(moves.len(), 20)
    }

    #[test]
    fn check_moves_is_correct_starting_position_black() {
        let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), false);

        let moves = game.get_all_moves();

        assert_eq!(moves.len(), 20)
    }


    #[test]
    fn check_moves_is_correct_many_positions() {
        // TODO: Add more
        let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR".to_string(), true);

        let moves = game.get_all_moves();

        assert_eq!(moves.len(), 26)
    }

    #[test]
    fn check_promotion_move() {
        let mut game = Game::new_from_string("8/2P5/8/8/8/8/5p1/8".to_string(), true);

        print_board(&game);
        let moves = game.get_all_moves();
        println!("{:?}", moves);
        game.make_move(&moves[3]);
        print_board(&game);
        game.undo_move();
        print_board(&game);
    }

    #[test]
    fn check_make_move_single_move() {
        // TODO: Add more
        let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);

        //print_board(&game);

        let m = Standard(51, 35, PAWN, true);

        game.make_move(&m);

        //print_board(&game);

        game.undo_move();

        //print_board(&game);

        //assert_eq!(game.board.get_black_occupancy() | game.board.get_white_occupancy(), )
    }


    #[test]
    fn check_make_move_double() {
        // TODO: Add more
        let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);

        print_board(&game);

        let m = Standard(51, 35, PAWN, true);

        game.make_move(&m);

        let m = Standard(11, 27, PAWN, false);



        game.make_move(&m);

        //print_board(&game);

        game.undo_move();

        //print_board(&game);

        //assert_eq!(game.board.get_black_occupancy() | game.board.get_white_occupancy(), )
    }

    #[test]
    fn check2_make_move_double() {
        // TODO: Add more
        let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);

        let moves1 = game.get_all_moves();

        //print_board(&game);

        //println!("moves1: {}", moves1.len());

        let m = Standard(51, 35, PAWN, true);

        game.make_move(&moves1[0]);

        let moves2 = game.get_all_moves();

        //println!("moves2: {}", moves1.len());


        let m = Standard(11, 27, PAWN, false);

        game.make_move(&moves2[0]);

        //print_board(&game);

        game.undo_move();

        //print_board(&game);

        //assert_eq!(game.board.get_black_occupancy() | game.board.get_white_occupancy(), )
    }

    #[test]
    pub fn test_ranks() {
        let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);
        print_board(&game);
        println!("{:?}", utils::get_file_and_rank(51));
    }

}
