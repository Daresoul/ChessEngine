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
    
    use crate::piece::piece::PieceType::{King, Queen, Bishop, Knight, Rook, Pawn};
    use crate::game::game::{Game};
    use crate::board::board::{Board};
    use crate::piece::piece::Piece;
    use md5::{Md5, Digest};
    use rand::{thread_rng, Rng};


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
        let game: Game = Game::new(0);
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

    #[test]
    fn hash_test(){
        let b1: Board = Board::new();
        let board_u8_arr = b1.get_evaluation();
        
        let mut hasher = Md5::new();
        hasher.update(board_u8_arr);
        
        let hash = hasher.finalize();
        let mut str: String = String::new();
        
        hash.iter().for_each(|x: &u8| str.push_str(format!("{:x}", x).as_str()));
        assert_eq!(hash.len(), 16);
        assert_eq!(str.as_str(), "3b5d3c7d207e37dceeedd31e35e2e58");

    }

    #[test]
    fn hash_performance_test_md5() {
        use std::time::Instant;

        let mut boards: Vec<Board> = vec![];

        for _ in 0..1_000_000 {
            let mut board: [Option<Piece>; 64] = [None; 64];
            
            let mut rng = thread_rng();

            for i in 0..64 {
                let piece: u8 = rng.gen_range(0..=12);
                board[i] = Board::get_piece(piece); 
            }

            let mut tmp: Board = Board {
                board_state: board,
                board_value: 0,
                vec_questionMark: vec![]
            };
       
            boards.push(tmp);

            
        }
            
        println!("boards: {}", boards.len());

        
        let cloned_boards2 = boards.clone();
        let now = Instant::now();

        for board in cloned_boards2 {
            let hash: usize = board.compute_hash();
        }
        let elapsed = now.elapsed();
            

        println!("time: {:.2?}", elapsed);

        let cloned_boards = boards.clone();
        let now = Instant::now();

        for board in cloned_boards {
            let hash: u64 = board.compute_hash_working();
        }
        let elapsed = now.elapsed();
            

        println!("time: {:.2?}", elapsed);

    }



}
