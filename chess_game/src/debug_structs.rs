pub mod debug_structs {
    use rand::{thread_rng, Rng};
    use crate::board::board::{Board, MoveType};
    use crate::game::game::Game;
    use crate::piece::piece::Piece;
    use crate::piece::piece::PieceType::{King, Rook, Bishop, Knight, Pawn, Queen};

    pub fn generate_boards(boards_amount: u64) -> Vec<Board> {
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

    pub fn generate_games(games_amount: u64) -> Vec<Game> {
        let mut games: Vec<Game> = vec![];

        for _ in 0..games_amount {
            let mut board: [Option<Piece>; 64] = [None; 64];

            let mut rng = thread_rng();

            for i in 0..64 {
                let piece: u64 = rng.gen_range(0..=12);
                board[i] = Piece::u64_to_piece(&piece);
            }

            let tmp: Game = Game::new_from_arr(board, true);

            games.push(tmp);
        }


        println!("games: {}", games.len());

        return games;
    }

    pub fn get_empty_move_board() -> [Option<Vec<MoveType>>; 64] {
        [
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None
        ]
    }

    pub fn get_normal_board() -> [Option<Piece>; 64] {
        [
            Some(Piece { piece_type: Rook, is_white: false }),
            Some(Piece { piece_type: Knight, is_white: false }),
            Some(Piece { piece_type: Bishop, is_white: false }),
            Some(Piece { piece_type: Queen, is_white: false }),
            Some(Piece { piece_type: King, is_white: false }),
            Some(Piece { piece_type: Bishop, is_white: false }),
            Some(Piece { piece_type: Knight, is_white: false }),
            Some(Piece { piece_type: Rook, is_white: false }),
            Some(Piece { piece_type: Pawn, is_white: false }),
            Some(Piece { piece_type: Pawn, is_white: false }),
            Some(Piece { piece_type: Pawn, is_white: false }),
            Some(Piece { piece_type: Pawn, is_white: false }),
            Some(Piece { piece_type: Pawn, is_white: false }),
            Some(Piece { piece_type: Pawn, is_white: false }),
            Some(Piece { piece_type: Pawn, is_white: false }),
            Some(Piece { piece_type: Pawn, is_white: false }),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            Some(Piece { piece_type: Pawn, is_white: true }),
            Some(Piece { piece_type: Pawn, is_white: true }),
            Some(Piece { piece_type: Pawn, is_white: true }),
            Some(Piece { piece_type: Pawn, is_white: true }),
            Some(Piece { piece_type: Pawn, is_white: true }),
            Some(Piece { piece_type: Pawn, is_white: true }),
            Some(Piece { piece_type: Pawn, is_white: true }),
            Some(Piece { piece_type: Pawn, is_white: true }),
            Some(Piece { piece_type: Rook, is_white: true }),
            Some(Piece { piece_type: Knight, is_white: true }),
            Some(Piece { piece_type: Bishop, is_white: true }),
            Some(Piece { piece_type: Queen, is_white: true }),
            Some(Piece { piece_type: King, is_white: true }),
            Some(Piece { piece_type: Bishop, is_white: true }),
            Some(Piece { piece_type: Knight, is_white: true }),
            Some(Piece { piece_type: Rook, is_white: true }),
        ]
    }

    pub fn get_debug_pawn_board() -> [Option<Piece>; 64] {
        [
            None, None, None, None, None, None, None, None,

            None, None, None,
            Some(Piece { piece_type: Pawn, is_white: false }),
            None, None, None, None,

            None, None, Some(Piece { piece_type: Pawn, is_white: true }), None, Some(Piece { piece_type: Pawn, is_white: true }), None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None,
            Some(Piece { piece_type: Pawn, is_white: true }),
            None, None, None, None,
            None, None, None, None, None, None, None, None
        ]
    }

    pub fn get_random_board() -> [Option<Piece>; 64] {
        [
            None, None, None, None, None, None, None, None,
            None, None, None, None, None,
            Some(Piece { piece_type: King, is_white: false }),
            None, None,

            None, None, None,
            Some(Piece { piece_type: Pawn, is_white: false }),
            None, None, None, None,

            None,
            Some(Piece { piece_type: Pawn, is_white: false }),
            None,
            Some(Piece { piece_type: Pawn, is_white: true }),
            Some(Piece { piece_type: Pawn, is_white: false }),
            None, None,
            Some(Piece { piece_type: Pawn, is_white: false }),

            Some(Piece { piece_type: Pawn, is_white: false }),
            Some(Piece { piece_type: Pawn, is_white: true }),
            None, None,
            Some(Piece { piece_type: Pawn, is_white: true }),
            Some(Piece { piece_type: Pawn, is_white: false }),
            None,
            Some(Piece { piece_type: Pawn, is_white: true }),

            Some(Piece { piece_type: Pawn, is_white: true }),
            None, None, None, None,
            Some(Piece { piece_type: Pawn, is_white: true }),
            None,
            Some(Piece { piece_type: King, is_white: true }),

            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None
        ]
    }

    pub fn get_random_board2() -> [Option<Piece>; 64] {
        [Some(Piece { piece_type: Rook, is_white: false }), None, Some(Piece { piece_type: Bishop, is_white: false }), Some(Piece { piece_type: King, is_white: false }), None, None, None, Some(Piece { piece_type: Rook, is_white: false }), Some(Piece { piece_type: Pawn, is_white: false }), None, None, Some(Piece { piece_type: Pawn, is_white: false }), Some(Piece { piece_type: Bishop, is_white: true }), Some(Piece { piece_type: Pawn, is_white: false }), Some(Piece { piece_type: Knight, is_white: true }), Some(Piece { piece_type: Pawn, is_white: false }), Some(Piece { piece_type: Knight, is_white: false }), None, None, None, None, Some(Piece { piece_type: Knight, is_white: false }), None, None, None, Some(Piece { piece_type: Pawn, is_white: false }), None, Some(Piece { piece_type: Knight, is_white: true }), Some(Piece { piece_type: Pawn, is_white: true }), None, None, Some(Piece { piece_type: Pawn, is_white: true }), None, None, None, None, None, None, Some(Piece { piece_type: Pawn, is_white: true }), None, None, None, None, Some(Piece { piece_type: Pawn, is_white: true }), None, None, None, None, Some(Piece { piece_type: Pawn, is_white: true }), None, Some(Piece { piece_type: Pawn, is_white: true }), None, Some(Piece { piece_type: King, is_white: true }), None, None, None, Some(Piece { piece_type: Queen, is_white: false }), None, None, None, None, None, Some(Piece { piece_type: Bishop, is_white: false }), None]
    }
}