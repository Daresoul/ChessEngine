mod game;
mod board;
mod piece;
mod debug;
/*
 *
 *
 *  MapReduce -> (K, [V]) -> R
 *  Where K = 
 *
 *
 */


// Look into peekable: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.peekable

#[cfg(test)]
mod tests { 

    use crate::piece::piece::PieceType::{King, Rook, Bishop, Knight, Pawn, Queen}; 
    use crate::game::game::{Game};
    use crate::board::board::{Board};
    use crate::piece::piece::Piece;
    use std::time::Instant;
    use rand::{thread_rng, Rng};

/*
    #[test]
    fn test_get_state_early() {
        let val: u64 = 1 << 0; 
        let game: Game = Game::new(val);
        assert_eq!(Board::get_board_state_from_position(&game.board, 0), true);
        assert_eq!(Board::get_board_state_from_position(&game.board, 1), false);
    }

    #[test]
    fn test_get_state_false() {
        let val: u64 = 1 << 32; 
        let game: Game = Game::new(val);
        assert_eq!(Board::get_board_state_from_position(&game.board, 31), false);
        assert_eq!(Board::get_board_state_from_position(&game.board, 33), false);
        assert_eq!(Board::get_board_state_from_position(&game.board, 32), true);
    }

    #[test]
    fn test_get_state() {
        let val: u64 = 1 << 63; 
        let game: Game = Game::new(val);

        assert_eq!(Board::get_board_state_from_position(&game.board, 62), false);
        assert_eq!(Board::get_board_state_from_position(&game.board, 63), true);
        assert_eq!(Board::get_board_state_from_position(&game.board, 64), false);
    }

    #[test]
    fn create_new_game() {
        let game: Game = Game::new();
        assert_eq!(game.board.board_value, 0);
    }


    #[test]
    fn create_new_board_with_set_value() {
        let board: Board = Board::set_board_bin(50);
        assert_eq!(board.board_value, 50);
    }

    #[test]
    fn create_new_board() {
        let board: Board = Board::new();
        assert_eq!(board.board_value, 0);
    }


    #[test]
    fn create_new_board_with_set_value_2() {
        let board: Board = Board::set_board_bin(18_446_462_598_732_906_495);
        assert_eq!(board.board_value, 18_446_462_598_732_906_495);
    }
*/

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
        let mut arr : [Option<Piece>; 64] = [
        Some(Piece { piece_type: Rook, is_white: false}),
        Some(Piece { piece_type: Knight, is_white: false}),
        Some(Piece { piece_type: Bishop, is_white: false}),
        Some(Piece { piece_type: Queen, is_white: false}),
        Some(Piece { piece_type: King, is_white: false}),
        Some(Piece { piece_type: Bishop, is_white: false}),
        Some(Piece { piece_type: Knight, is_white: false}),
        Some(Piece { piece_type: Rook, is_white: false}),

        Some(Piece { piece_type: Pawn, is_white: false}),
        Some(Piece { piece_type: Pawn, is_white: false}),
        Some(Piece { piece_type: Pawn, is_white: false}),
        Some(Piece { piece_type: Pawn, is_white: false}),
        Some(Piece { piece_type: Pawn, is_white: false}),
        Some(Piece { piece_type: Pawn, is_white: false}),
        Some(Piece { piece_type: Pawn, is_white: false}),
        Some(Piece { piece_type: Pawn, is_white: false}),

        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,

        Some(Piece { piece_type: Pawn, is_white: true}),
        Some(Piece { piece_type: Pawn, is_white: true}),
        Some(Piece { piece_type: Pawn, is_white: true}),
        Some(Piece { piece_type: Pawn, is_white: true}),
        Some(Piece { piece_type: Pawn, is_white: true}),
        Some(Piece { piece_type: Pawn, is_white: true}),
        Some(Piece { piece_type: Pawn, is_white: true}),
        Some(Piece { piece_type: Pawn, is_white: true}),

        Some(Piece { piece_type: Rook, is_white: true}),
        Some(Piece { piece_type: Knight, is_white: true}),
        Some(Piece { piece_type: Bishop, is_white: true}),
        Some(Piece { piece_type: Queen, is_white: true}),
        Some(Piece { piece_type: King, is_white: true}),
        Some(Piece { piece_type: Bishop, is_white: true}),
        Some(Piece { piece_type: Knight, is_white: true}),
        Some(Piece { piece_type: Rook, is_white: true}),
        ];

        let val = Board::get_board_value(arr);

        assert_eq!(val, 18_446_462_598_732_906_495);
    }

    #[test]
    fn test_of_new_game_from_state() {
        let mut arr : [Option<Piece>; 64] = [
            Some(Piece { piece_type: Rook, is_white: false}),
            Some(Piece { piece_type: Knight, is_white: false}),
            Some(Piece { piece_type: Bishop, is_white: false}),
            Some(Piece { piece_type: Queen, is_white: false}),
            Some(Piece { piece_type: King, is_white: false}),
            Some(Piece { piece_type: Bishop, is_white: false}),
            Some(Piece { piece_type: Knight, is_white: false}),
            Some(Piece { piece_type: Rook, is_white: false}),

            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),

            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,

            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),

            Some(Piece { piece_type: Rook, is_white: true}),
            Some(Piece { piece_type: Knight, is_white: true}),
            Some(Piece { piece_type: Bishop, is_white: true}),
            Some(Piece { piece_type: Queen, is_white: true}),
            Some(Piece { piece_type: King, is_white: true}),
            Some(Piece { piece_type: Bishop, is_white: true}),
            Some(Piece { piece_type: Knight, is_white: true}),
            Some(Piece { piece_type: Rook, is_white: true}),
        ];

        let game = Game::new_from_arr(arr);

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

    fn generate_boards(boards_amount: u64) -> Vec<Board> {
        let mut boards: Vec<Board> = vec![];

        for _ in 0..boards_amount {
            let mut board: [Option<Piece>; 64] = [None; 64];

            let mut rng = thread_rng();

            for i in 0..64 {
                let piece: u64 = rng.gen_range(0..=12);
                board[i] = Piece::u64_to_piece(&piece);
            }

            let tmp: Board = Board {
                board_state: board,
                board_value: 0
            };

            boards.push(tmp);
        }


        println!("boards: {}", boards.len());

        return boards;
    }

    //#[test]
    fn hash_performance_test() {

        let boards = generate_boards(1_000_000);

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

    //#[test]
    fn create_board_performance_test() {

        let boards = generate_boards(1_000_000);

        let now = Instant::now();

        for board in boards {
            let game = Game::new_from_arr(board.board_state);
        }

        let elapsed = now.elapsed();

        println!("time: {:.2?}", elapsed);
    }

    #[test]
    fn make_move_performance_test() {
        let mut arr : [Option<Piece>; 64] = [
            Some(Piece { piece_type: Rook, is_white: false}),
            Some(Piece { piece_type: Knight, is_white: false}),
            Some(Piece { piece_type: Bishop, is_white: false}),
            Some(Piece { piece_type: Queen, is_white: false}),
            Some(Piece { piece_type: King, is_white: false}),
            Some(Piece { piece_type: Bishop, is_white: false}),
            Some(Piece { piece_type: Knight, is_white: false}),
            Some(Piece { piece_type: Rook, is_white: false}),

            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),
            Some(Piece { piece_type: Pawn, is_white: false}),

            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,

            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),
            Some(Piece { piece_type: Pawn, is_white: true}),

            Some(Piece { piece_type: Rook, is_white: true}),
            Some(Piece { piece_type: Knight, is_white: true}),
            Some(Piece { piece_type: Bishop, is_white: true}),
            Some(Piece { piece_type: Queen, is_white: true}),
            Some(Piece { piece_type: King, is_white: true}),
            Some(Piece { piece_type: Bishop, is_white: true}),
            Some(Piece { piece_type: Knight, is_white: true}),
            Some(Piece { piece_type: Rook, is_white: true}),
        ];

        let game = Game::new_from_arr(arr);

        let now = Instant::now();

        let all_moves = game.get_all_moves();

        let elapsed = now.elapsed();

        println!("moves_count: {}", all_moves.len());
        println!("time: {:.2?}", elapsed);
    }



}
