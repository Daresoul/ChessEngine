pub mod game;
pub mod board;
pub mod debug;
mod eval_board;
mod UTILS;
mod Constants;
mod magic;
mod move_gen;

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
    use crate::move_gen::move_gen::MoveGen;
    use crate::move_gen::move_gen::PieceType::{BISHOP, KNIGHT, PAWN, QUEEN, ROOK};

/*    #[test]
    fn pawn_move_single_double_white() {
        let game = Game::new_from_string("8/3p4/8/8/8/8/3P4/8".to_string(), true);

        let mut moves  = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(51, 35, PAWN),
            Standard(51, 43, PAWN),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);
        //assert_eq!(expected_black_moves, black_moves);
    }

    #[test]
    fn pawn_move_cant_take_allied_pieces() {
        let game = Game::new_from_string("8/3p4/2p1p3/8/8/2P1P3/3P4/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(42, 34, PAWN),
            Standard(51, 35, PAWN),
            Standard(44, 36, PAWN),
            Standard(51, 43, PAWN),
        ];

        moves.sort();
        expected_white_moves.sort();

        assert_eq!(expected_white_moves, moves);
    }

    #[test]
    fn pawn_move_can_take_outside_start_squares() {
        let game = Game::new_from_string("8/8/3p4/2P1P3/2p1p3/3P4/8/8".to_string(), true);

        let mut moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(43, 35, PAWN),
            Standard(43, 34, PAWN),
            Standard(43, 36, PAWN),
            Standard(26, 18, PAWN),
            Standard(26, 19, PAWN),
            Standard(28, 20, PAWN),
            Standard(28, 19, PAWN),
        ];

        expected_white_moves.sort();
        moves.sort();

        assert_eq!(expected_white_moves, moves);

    }

    #[test]
    fn pawn_on_edge() {
        let game = Game::new_from_string("1p4p1/P6P/8/8/8/8/p6p/8".to_string(), true);

        let mut white_moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Promotion(8, 0, KNIGHT),
            Promotion(8, 0, BISHOP),
            Promotion(8, 0, QUEEN),
            Promotion(8, 0, ROOK),
            Promotion(8, 1, KNIGHT),
            Promotion(8, 1, BISHOP),
            Promotion(8, 1, QUEEN),
            Promotion(8, 1, ROOK),
            Promotion(15, 6, KNIGHT),
            Promotion(15, 6, BISHOP),
            Promotion(15, 6, QUEEN),
            Promotion(15, 6, ROOK),
            Promotion(15, 7, KNIGHT),
            Promotion(15, 7, BISHOP),
            Promotion(15, 7, QUEEN),
            Promotion(15, 7, ROOK),
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);

    }

    #[test]
    fn pawn_move_promotion() {
        let game = Game::new_from_string("8/3P4/8/8/8/8/3p4/8".to_string(), true);

        let mut white_moves= game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Promotion(11, 3, KNIGHT),
            Promotion(11, 3, BISHOP),
            Promotion(11, 3, QUEEN),
            Promotion(11, 3, ROOK),
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);

    }

    #[test]
    fn knight_normal_move() {
        let game = Game::new_from_string("8/8/3n4/8/8/3N4/8/8".to_string(), true);

        let mut white_moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(43, 28, KNIGHT),
            Standard(43, 26, KNIGHT),
            Standard(43, 33, KNIGHT),
            Standard(43, 49, KNIGHT),
            Standard(43, 58, KNIGHT),
            Standard(43, 60, KNIGHT),
            Standard(43, 37, KNIGHT),
            Standard(43, 53, KNIGHT),
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);
    }

    #[test]
    fn knight_take_opponent_cant_take_teamate_move() {
        let game = Game::new_from_string("8/8/3np3/2p1P3/2P1p3/3NP3/8/8".to_string(), true);

        let mut white_moves = game.get_all_moves();


        let mut expected_white_moves: Vec<Move> = vec![
            Standard(28, 19, PAWN),
            Standard(43, 26, KNIGHT),
            Standard(43, 33, KNIGHT),
            Standard(43, 49, KNIGHT),
            Standard(43, 58, KNIGHT),
            Standard(43, 60, KNIGHT),
            Standard(43, 37, KNIGHT),
            Standard(43, 53, KNIGHT),
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);

    }

    #[test]
    fn knight_normal_move_corners() {
        let game = Game::new_from_string("N7/8/8/8/8/8/8/7N".to_string(), true);

        let mut white_moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(0, 10, KNIGHT),
            Standard(0, 17, KNIGHT),
            Standard(63, 46, KNIGHT),
            Standard(63, 53, KNIGHT)
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);
    }

    #[test]
    fn rook_all_move_test() {
        let game = Game::new_from_string("8/8/8/8/3R4/8/8/8".to_string(), true);

        let mut white_moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 27, ROOK),
            Standard(35, 19, ROOK),
            Standard(35, 11, ROOK),
            Standard(35, 3, ROOK),
            Standard(35, 43, ROOK),
            Standard(35, 51, ROOK),
            Standard(35, 59, ROOK),
            Standard(35, 34, ROOK),
            Standard(35, 33, ROOK),
            Standard(35, 32, ROOK),
            Standard(35, 36, ROOK),
            Standard(35, 37, ROOK),
            Standard(35, 38, ROOK),
            Standard(35, 39, ROOK),
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(white_moves, expected_white_moves);
    }

    #[test]
    fn rook_stops_on_opponents() {
        let game = Game::new_from_string("8/8/3p4/8/p2Rp3/8/8/3p4".to_string(), true);

        let mut white_moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 27, ROOK),
            Standard(35, 19, ROOK),
            Standard(35, 43, ROOK),
            Standard(35, 51, ROOK),
            Standard(35, 59, ROOK),
            Standard(35, 34, ROOK),
            Standard(35, 33, ROOK),
            Standard(35, 32, ROOK),
            Standard(35, 36, ROOK),
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(white_moves, expected_white_moves);
    }

    #[test]
    fn rook_stops_on_allies() {
        let game = Game::new_from_string("8/8/3P4/8/P2RP3/8/8/3P4".to_string(), true);

        let mut white_moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 27, ROOK),
            Standard(35, 43, ROOK),
            Standard(35, 51, ROOK),
            Standard(35, 34, ROOK),
            Standard(35, 33, ROOK),

            Standard(19, 11, PAWN),
            Standard(32, 24, PAWN),
            Standard(36, 28, PAWN),
            Standard(59, 51, PAWN)
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);
    }


    #[test]
    fn rook_top_left_corner() {
        let game = Game::new_from_string("R7/8/8/8/8/8/8/8".to_string(), true);

        let mut white_moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(0, 1, ROOK),
            Standard(0, 2, ROOK),
            Standard(0, 3, ROOK),
            Standard(0, 4, ROOK),
            Standard(0, 5, ROOK),
            Standard(0, 6, ROOK),
            Standard(0, 7, ROOK),

            Standard(0, 8, ROOK),
            Standard(0, 16, ROOK),
            Standard(0, 24, ROOK),
            Standard(0, 32, ROOK),
            Standard(0, 40, ROOK),
            Standard(0, 48, ROOK),
            Standard(0, 56, ROOK)
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);
    }

    #[test]
    fn rook_top_right_corner() {
        let game = Game::new_from_string("7R/8/8/8/8/8/8/8".to_string(), true);

        let mut white_moves= game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(7, 1, ROOK),
            Standard(7, 2, ROOK),
            Standard(7, 3, ROOK),
            Standard(7, 4, ROOK),
            Standard(7, 5, ROOK),
            Standard(7, 6, ROOK),
            Standard(7, 0, ROOK),

            Standard(7, 15, ROOK),
            Standard(7, 23, ROOK),
            Standard(7, 31, ROOK),
            Standard(7, 39, ROOK),
            Standard(7, 47, ROOK),
            Standard(7, 55, ROOK),
            Standard(7, 63, ROOK)
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);
    }

    #[test]
    fn rook_bottom_left_corner() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/R7".to_string(), true);

        let mut white_moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(56, 57, ROOK),
            Standard(56, 58, ROOK),
            Standard(56, 59, ROOK),
            Standard(56, 60, ROOK),
            Standard(56, 61, ROOK),
            Standard(56, 62, ROOK),
            Standard(56, 63, ROOK),

            Standard(56, 16, ROOK),
            Standard(56, 24, ROOK),
            Standard(56, 32, ROOK),
            Standard(56, 40, ROOK),
            Standard(56, 48, ROOK),
            Standard(56, 0, ROOK),
            Standard(56, 8, ROOK)
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);
    }

    #[test]
    fn rook_bottom_right_corner() {
        let game = Game::new_from_string("8/8/8/8/8/8/8/7R".to_string(), true);

        let mut white_moves= game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(63, 57, ROOK),
            Standard(63, 58, ROOK),
            Standard(63, 59, ROOK),
            Standard(63, 60, ROOK),
            Standard(63, 61, ROOK),
            Standard(63, 62, ROOK),
            Standard(63, 56, ROOK),

            Standard(63, 7, ROOK),
            Standard(63, 15, ROOK),
            Standard(63, 23, ROOK),
            Standard(63, 31, ROOK),
            Standard(63, 39, ROOK),
            Standard(63, 47, ROOK),
            Standard(63, 55, ROOK)
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);
    }*/


/*    #[test]
    fn bishop_all_moves_test() {
        let mut game = Game::new_from_string("8/8/8/8/3B4/8/8/8".to_string(), true);
        game.get_all_moves();
        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 26, BISHOP),
            Standard(35, 17, BISHOP),
            Standard(35, 08, BISHOP),
            Standard(35, 28, BISHOP),
            Standard(35, 21, BISHOP),
            Standard(35, 14, BISHOP),
            Standard(35, 07, BISHOP),
            Standard(35, 42, BISHOP),
            Standard(35, 49, BISHOP),
            Standard(35, 56, BISHOP),
            Standard(35, 44, BISHOP),
            Standard(35, 53, BISHOP),
            Standard(35, 62, BISHOP),
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);
    }*/

/*    #[test]
    fn bishop_stops_on_opponents() {
        let game = Game::new_from_string("8/p7/5p2/8/3B4/4p3/8/p7".to_string(), true);

        let mut white_moves= game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 26, BISHOP),
            Standard(35, 17, BISHOP),
            Standard(35, 08, BISHOP),

            Standard(35, 28, BISHOP),
            Standard(35, 21, BISHOP),

            Standard(35, 42, BISHOP),
            Standard(35, 49, BISHOP),
            Standard(35, 56, BISHOP),
            Standard(35, 44, BISHOP),
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(white_moves, expected_white_moves);
    }

    #[test]
    fn bishop_stops_on_allies() {
        let game = Game::new_from_string("8/P7/5P2/8/3B4/4P3/8/P7".to_string(), true);

        print_board(&game);
        print_bitboard_board(&game.move_gen.get_move(BISHOP, 35, game.board.get_white_occupancy(), game.board.get_white_occupancy() | game.board.get_black_occupancy()));

        let mut white_moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 26, BISHOP),
            Standard(35, 17, BISHOP),

            Standard(35, 28, BISHOP),

            Standard(35, 42, BISHOP),
            Standard(35, 49, BISHOP),
            Standard(44, 36, PAWN),
            Standard(56, 48, PAWN),
            Promotion(8, 0, KNIGHT),
            Promotion(8, 0, BISHOP),
            Promotion(8, 0, QUEEN),
            Promotion(8, 0, ROOK),
            Standard(21, 13, PAWN)
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(white_moves, expected_white_moves);
    }


    #[test]
    fn queen_all_moves_test() {
        let game = Game::new_from_string("8/8/8/8/3Q4/8/8/8".to_string(), true);

        let mut white_moves = game.get_all_moves();

        let mut expected_white_moves: Vec<Move> = vec![
            Standard(35, 26, QUEEN),
            Standard(35, 17, QUEEN),
            Standard(35, 08, QUEEN),
            Standard(35, 28, QUEEN),
            Standard(35, 21, QUEEN),
            Standard(35, 14, QUEEN),
            Standard(35, 07, QUEEN),
            Standard(35, 42, QUEEN),
            Standard(35, 49, QUEEN),
            Standard(35, 56, QUEEN),
            Standard(35, 44, QUEEN),
            Standard(35, 53, QUEEN),
            Standard(35, 62, QUEEN),

            Standard(35, 27, QUEEN),
            Standard(35, 19, QUEEN),
            Standard(35, 11, QUEEN),
            Standard(35, 3, QUEEN),
            Standard(35, 43, QUEEN),
            Standard(35, 51, QUEEN),
            Standard(35, 59, QUEEN),
            Standard(35, 34, QUEEN),
            Standard(35, 33, QUEEN),
            Standard(35, 32, QUEEN),
            Standard(35, 36, QUEEN),
            Standard(35, 37, QUEEN),
            Standard(35, 38, QUEEN),
            Standard(35, 39, QUEEN),
        ];

        expected_white_moves.sort();
        white_moves.sort();

        assert_eq!(expected_white_moves, white_moves);
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
    }*/




}
