
mod debug
{
    use crate::game::game::Game;
    use crate::board::board::Board;

    pub fn debug_board(game: &Game) {
       print!("board_value: {}", game.board.board_value);
       for (i, _) in (0..64).enumerate() {
           if i % 8 == 0 {
               print!("\n");
           }
           print!("{} ", Board::get_board_state_from_position(&game.board, i as u8));
        }
    }
}

