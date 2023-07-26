
pub mod game {
 
    use crate::board::board::{Board};

    pub struct Game {
        pub board: Board    
    }

    impl Game {
       pub fn new(boardset: u64) -> Game {
            return Game { board: Board::set_board_bin(boardset)}
       }
    }

}

