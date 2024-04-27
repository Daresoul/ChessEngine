
pub mod debug
{
    
    use crate::board::board::Board;
    use crate::game::game::Game;


    pub fn print_board(game: &Game) {
        println!("Board:");

        let board: &Board = &game.board;

        for i in 0..8 {
            for j in 0..8 {
                let currentVal = i * 8 + j;
                let currentBitVal = 1 << currentVal;

                print!("[{: ^width$} ({: ^2})]", getCheckPieceAtPosition(board, currentBitVal), currentVal, width = 3);
            }
            println!();
        }
    }

    pub fn print_board_from_board(board: &Board) {
        println!("Board:");

        for i in 0..8 {
            for j in 0..8 {
                let currentVal = i * 8 + j;
                let currentBitVal = 1 << currentVal;

                print!("[{: ^width$} ({: ^2})]", getCheckPieceAtPosition(board, currentBitVal), currentVal, width = 3);
            }
            println!();
        }
    }

    pub fn print_bitboard_board(num: &u64) {
        for i in 0..8 {
            for j in 0..8 {
                let current_val = 1 << i * 8 + j;

                print!("{} ", match current_val & num > 0 {
                    true => {
                        "1"
                    },
                    false => {
                        "0"
                    }
                });
            }
            println!();
        }
    }

    pub fn getCheckPieceAtPosition(board: &Board, bit: u64) -> String {
        let mut pieceString: String = " ".to_string();

        if board.black_king_board & bit > 0 {
            pieceString.push('k')
        }

        if board.black_queen_board & bit > 0 {
            pieceString.push('q')
        }

        if board.black_bishop_board & bit > 0 {
            pieceString.push('b')
        }

        if board.black_knight_board & bit > 0 {
            pieceString.push('n')
        }

        if board.black_rook_board & bit > 0 {
            pieceString.push('r')
        }

        if board.black_pawn_board & bit > 0 {
            pieceString.push('p')
        }

        if board.white_king_board & bit > 0 {
            pieceString.push('K')
        }

        if board.white_queen_board & bit > 0 {
            pieceString.push('Q')
        }

        if board.white_bishop_board & bit > 0 {
            pieceString.push('B')
        }

        if board.white_knight_board & bit > 0 {
            pieceString.push('N')
        }

        if board.white_rook_board & bit > 0 {
            pieceString.push('R')
        }

        if board.white_pawn_board & bit > 0 {
            pieceString.push('P')
        }

        return pieceString
    }

    fn print_bitboard(b: u64) {
        for i in 0..64 {
            print!("{}", 1 << i & b);

            if i % 8 == 0 {
                println!()
            }
        }
    }
}

