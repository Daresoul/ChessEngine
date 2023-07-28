pub mod piece {
    use crate::board::board::Board;

    #[derive(Debug, Clone, Copy)]
    pub enum PieceType {
        Pawn, // 001
        Rook, // 010
        Knight, // 011
        Bishop, // 100
        Queen, // 101
        King // 110
    }



    #[derive(Debug, Clone, Copy)]
    pub struct Piece {
        pub piece_type: PieceType,
        pub is_white: bool
    }


    impl PartialEq for Piece {
        fn eq(&self, other: &Self) -> bool {
            self.piece_type as u8 == other.piece_type as u8 && self.is_white == other.is_white
        }
    }

    impl Piece {
        pub fn u64_to_piece(val: &u64) -> Option<Piece> {
            match val {
                0 =>  None,
                1 =>  Some(Piece { is_white: true, piece_type: PieceType::Pawn}),
                2 =>  Some(Piece { is_white: true, piece_type: PieceType::Rook}),
                3 =>  Some(Piece { is_white: true, piece_type: PieceType::Knight}),
                4 =>  Some(Piece { is_white: true, piece_type: PieceType::Bishop}),
                5 =>  Some(Piece { is_white: true, piece_type: PieceType::Queen}),
                6 =>  Some(Piece { is_white: true, piece_type: PieceType::King}),
                7 =>  Some(Piece { is_white: false, piece_type: PieceType::Pawn}),
                8 =>  Some(Piece { is_white: false, piece_type: PieceType::Rook}),
                9 =>  Some(Piece { is_white: false, piece_type: PieceType::Knight}),
                10 => Some(Piece { is_white: false, piece_type: PieceType::Bishop}),
                11 => Some(Piece { is_white: false, piece_type: PieceType::Queen}),
                12 => Some(Piece { is_white: false, piece_type: PieceType::King}),
                _ => None
            }
        }

        pub fn piece_to_u64(piece: &Option<Piece>) -> u64 {
            match piece {
                Some(piece) => {
                    if piece.is_white {
                        piece.piece_type as u64 + 1
                    } else {
                        piece.piece_type as u64 + 7
                    }
                },
                None => {
                    return 0;
                }
            }
        }

        pub fn get_moves(&self, board: &Board, index: &u8) -> Vec<u64> {
            match self.piece_type {
                PieceType::Pawn => self.pawn_moves(board, index),
                _ => vec![]
            }
        }

        fn pawn_moves(&self, board: &Board, index: &u8) -> Vec<u64> {
            let mut moves: Vec<u64> = vec![];
            let straight_move = 1 << *index + 8;
            if !Board::get_board_state_from_position(board, &straight_move) {
                moves.push(u64::from(straight_move));
            }
            moves
        }


    }



}
