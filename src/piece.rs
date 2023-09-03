pub mod piece {
    use std::cmp::min;
    use std::fmt;
    use std::fmt::write;
    use crate::board::board::Board;
    use crate::board::board::MoveType;

    #[derive(Debug, Clone, Copy)]
    pub enum PieceType {
        Pawn, // 001
        Rook, // 010
        Knight, // 011
        Bishop, // 100
        Queen, // 101
        King // 110
    }

    impl PartialEq for PieceType {
        fn eq(&self, other: &Self) -> bool {
            match self {
                PieceType::Pawn => match other {
                    PieceType::Pawn => true,
                    _ => false
                },
                PieceType::Rook => match other {
                    PieceType::Rook => true,
                    _ => false
                },
                PieceType::Knight => match other {
                    PieceType::Knight => true,
                    _ => false
                },
                PieceType::Bishop => match other {
                    PieceType::Bishop => true,
                    _ => false
                },
                PieceType::Queen => match other {
                    PieceType::Queen => true,
                    _ => false
                },
                PieceType::King => match other {
                    PieceType::King => true,
                    _ => false
                }
            }
        }
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

    impl fmt::Display for Piece {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.is_white {
                true => match self.piece_type {
                    PieceType::Pawn => write!(f, "WP"),
                    PieceType::Rook => write!(f, "WR"),
                    PieceType::Knight => write!(f, "WN"),
                    PieceType::Bishop => write!(f, "WB"),
                    PieceType::Queen => write!(f, "WQ"),
                    PieceType::King => write!(f, "WK")
                },
                false => match self.piece_type {
                    PieceType::Pawn => write!(f, "BP"),
                    PieceType::Rook => write!(f, "BR"),
                    PieceType::Knight => write!(f, "BN"),
                    PieceType::Bishop => write!(f, "BB"),
                    PieceType::Queen => write!(f, "BQ"),
                    PieceType::King => write!(f, "BK")
                },
            }
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

        pub fn get_value(&self) -> u8 {
            match self.piece_type {
                PieceType::Pawn => 1,
                PieceType::Rook => 5,
                PieceType::Knight => 3,
                PieceType::Bishop => 3,
                PieceType::Queen => 9,
                _ => 0
            }
        }

        pub fn get_moves(&self, board: &Board, index: &u8) -> Vec<MoveType> {
            match self.piece_type {
                PieceType::Pawn => self.pawn_moves(board, index),
                PieceType::Rook => self.rook_moves(board, index),
                PieceType::Bishop => self.bishop_moves(board, index),
                PieceType::King => self.king_moves(board, index),
                PieceType::Knight => self.knight_moves(board, index),
                PieceType::Queen => self.queen_moves(board, index)
            }
        }

        fn pawn_moves(&self, board: &Board, index: &u8) -> Vec<MoveType> {
            let mut moves: Vec<MoveType> = vec![];

            // Move straight
            let straight_move = if self.is_white {*index - 8} else {*index + 8};
            if !Board::get_board_state_from_position(board, &straight_move) {
                moves.push(MoveType::Standard(*index, straight_move, self.is_white));
            }

            // Move diagonal left
            let diagonal_left = if self.is_white {*index - 9} else {*index + 7};
            if Board::get_board_state_from_position(board, &diagonal_left) {
                match board.board_state[usize::from(diagonal_left)] {
                    Some(piece) => {
                        if piece.is_white != self.is_white {
                            moves.push(MoveType::Standard(*index,diagonal_left, self.is_white));
                        }
                    },
                    None => panic!("Should never happen")
                }
            }

            // Move diagonal right
            let diagonal_right = if self.is_white {*index - 7} else {*index + 9};
            if Board::get_board_state_from_position(board, &diagonal_right) {
                match board.board_state[usize::from(diagonal_right)] {
                    Some(piece) => {
                        if piece.is_white != self.is_white {
                            moves.push(MoveType::Standard(*index,diagonal_right, self.is_white));
                        }
                    },
                    None => panic!("Should never happen")
                }
            }

            // Double move
            if self.is_white && *index >= 48 && *index <= 55 {
                let double_move = *index - 16;
                if !Board::get_board_state_from_position(board, &double_move) {
                    moves.push(MoveType::Standard(*index,double_move, self.is_white));
                }

            }
            else if !self.is_white && *index >= 8 && *index <= 15 {
                let double_move = *index + 16;
                if !Board::get_board_state_from_position(board, &double_move) {
                    moves.push(MoveType::Standard(*index,double_move, self.is_white));
                }
            }

            // Return
            moves
        }

        fn rook_move(&self, board: &Board, from: u8, index: u8, count: &u8) -> (u8, Option<MoveType>) {
            if Board::get_board_state_from_position(board, &index) {
                match board.board_state[usize::from(index)] {
                    Some(piece) => {
                        if piece.is_white != self.is_white {
                            let rook_move = if *count == 0 { MoveType::Standard(from,index, self.is_white)} else { MoveType::FutureMove(from,index, self.is_white)};
                            return (1, Some(rook_move))
                        }
                        else {
                            return (2, None)
                        }
                    },
                    None => panic!("Should never happen")
                }
            } else {
                let rook_move = if *count == 0 { MoveType::Standard(from,index, self.is_white)} else { MoveType::FutureMove(from,index, self.is_white)};
                return (0, Some(rook_move))
            }
        }

        pub fn rook_moves(&self, board: &Board, index: &u8) -> Vec<MoveType> {
            let mut moves: Vec<MoveType> = vec![];

            // Counts are for option types
            let mut count_up = 0;
            let mut count_down = 0;
            let mut count_left = 0;
            let mut count_right = 0;

            for i in 0..7 {
                if count_up < 2 {
                    let up = index.checked_sub((i + 1) * 8);
                    match up {
                        Some(up_index) => {
                            let (added, rook_up_move) = self.rook_move(board, *index, up_index, &count_up);
                            count_up += added;
                            match rook_up_move {
                                Some(move_type) => moves.push(move_type),
                                None => {}
                            }
                        },
                        None => { count_up = 2; }
                    }
                }

                if count_down < 2 {
                    let down = index.checked_add((i + 1) * 8);
                    match down {
                        Some(down_index) => {
                            if down_index > 63 {
                                count_down = 2;
                            }
                            else {
                                let (added, rook_down_move) = self.rook_move(board, *index, down_index, &count_down);
                                count_down += added;
                                match rook_down_move {
                                    Some(move_type) => moves.push(move_type),
                                    None => {}
                                }
                            }
                        },
                        None => { count_down = 2; }
                    }
                }

                if count_right < 2 {
                    let right = index.checked_add(i + 1);
                    match right {
                        Some(right_index) => {

                            if right_index % 8 == 0 {
                                count_right = 2;
                            }
                            else{
                                let (added, rook_right_move) = self.rook_move(board, *index, right_index, &count_right);
                                count_right += added;
                                match rook_right_move {
                                    Some(move_type) => moves.push(move_type),
                                    None => {}
                                }
                            }
                        },
                        None => { count_right = 2; }
                    }
                }

                if count_left < 2 {
                    let left = index.checked_sub(i + 1);
                    match left {
                        Some(left_index) => {

                            if left_index % 8 == 7 {
                                count_left = 2;
                            } else {
                                let (added, rook_left_move) = self.rook_move(board, *index, left_index, &count_left);
                                count_left += added;
                                match rook_left_move {
                                    Some(move_type) => moves.push(move_type),
                                    None => {}
                                }
                            }
                        },
                        None => { count_left = 2; }
                    }
                }
            }

            moves
        }

        pub fn king_moves(&self, board: &Board, index: &u8) -> Vec<MoveType> {
            let mut moves: Vec<MoveType> = vec![];

            let king_move_indexes: [Option<u8>; 8] = [
                index.checked_sub(1), index.checked_sub(9), index.checked_sub(8),
                index.checked_sub(7), index.checked_add(1), index.checked_add(7),
                index.checked_add(8), index.checked_add(9)
            ];
            // For anything that subtracts
            for i in 0..8 {
                match king_move_indexes[i] {
                    Some(val) => {

                        if val % 8 == 0 && index % 8 == 7 {
                            continue;
                        }

                        if val % 8 == 7 && index % 8 == 0 {
                            continue;
                        }

                        if val > 63 {
                            continue;
                        }

                        if Board::get_board_state_from_position(board, &val) {
                            match board.board_state[usize::from(val)] {
                                Some(piece) => {
                                    if piece.is_white != self.is_white {
                                        moves.push(MoveType::Standard(*index,val, self.is_white));
                                    }
                                },
                                None => panic!("Should never happen")
                            }
                        }
                        else {
                            moves.push(MoveType::Standard(*index,val, self.is_white));
                        }
                    },
                    None => {}
                }
            }

            moves
        }

        fn bishop_move(&self, board: &Board, from: u8, index: u8, count: &u8) -> (u8, Option<MoveType>) {
            if Board::get_board_state_from_position(board, &index) {
                match board.board_state[usize::from(index)] {
                    Some(piece) => {
                        if piece.is_white != self.is_white {
                            let bishop_move = if *count == 0 { MoveType::Standard(from,index, self.is_white)} else { MoveType::FutureMove(from,index, self.is_white)};
                            return (*count + 1, Some(bishop_move))
                        }
                        else {
                            return (2, None)
                        }
                    },
                    None => panic!("Should never happen")
                }
            } else {
                let bishop_move = if *count == 0 { MoveType::Standard(from,index, self.is_white)} else { MoveType::FutureMove(from,index, self.is_white)};
                return (0, Some(bishop_move))
            }
        }

        pub fn bishop_moves(&self, board: &Board, index: &u8) -> Vec<MoveType> {
            let mut moves = vec![];

            let mut diagonal_up_right = 0;
            let mut diagonal_up_left = 0;
            let mut diagonal_down_right = 0;
            let mut diagonal_down_left = 0;

            for i in 0..8 {
                if diagonal_up_right < 2 {
                    let up_right = index.checked_sub((i + 1) * 7);
                    match up_right {
                        Some(up_index) => {
                            if up_index % 8 == 0 {
                                diagonal_up_right = 2;
                            } else {
                                let (added, bishop_up_right_move) = self.bishop_move(board, *index, up_index, &diagonal_up_right);
                                diagonal_up_right += added;
                                match bishop_up_right_move {
                                    Some(move_type) => moves.push(move_type),
                                    None => {}
                                }
                            }
                        },
                        None => { diagonal_up_right = 2; }
                    }
                }

                if diagonal_up_left < 2 {
                    let up_left = index.checked_sub((i + 1) * 9);
                    match up_left {
                        Some(up_index) => {
                            if up_index % 8 == 7 {
                                diagonal_up_left = 2;
                            } else {
                                let (added, bishop_up_left_move) = self.bishop_move(board, *index, up_index, &diagonal_up_left);
                                diagonal_up_left += added;
                                match bishop_up_left_move {
                                    Some(move_type) => moves.push(move_type),
                                    None => {}
                                }
                            }
                        },
                        None => { diagonal_up_left = 2; }
                    }
                }

                if diagonal_down_right < 2 {
                    let down_right = index.checked_add((i + 1) * 9);
                    match down_right {
                        Some(down_index) => {
                            if down_index > 63 {
                                diagonal_down_left = 2;
                                continue;
                            } else if down_index % 8 == 0 {
                                diagonal_down_right = 2;
                            } else {
                                let (added, bishop_down_right_move) = self.bishop_move(board, *index, down_index, &diagonal_down_right);
                                diagonal_down_right += added;
                                match bishop_down_right_move {
                                    Some(move_type) => moves.push(move_type),
                                    None => {}
                                }
                            }
                        },
                        None => { diagonal_down_right = 2; }
                    }
                }

                if diagonal_down_left < 2 {
                    let down_left = index.checked_add((i + 1) * 7);
                    match down_left {
                        Some(down_index) => {
                            if down_index > 63 {
                                diagonal_down_left = 2;
                            } else if down_index % 8 == 7 {
                                diagonal_down_left = 2;
                            } else {
                                let (added, bishop_down_left_move) = self.bishop_move(board, *index, down_index, &diagonal_down_left);
                                diagonal_down_left += added;
                                match bishop_down_left_move {
                                    Some(move_type) => moves.push(move_type),
                                    None => {}
                                }
                            }
                        },
                        None => { diagonal_down_left = 2; }
                    }
                }
            }

            return moves;
        }


        pub fn knight_moves(&self, board: &Board, index: &u8) -> Vec<MoveType> {
            let mut moves: Vec<MoveType> = vec![];

            let knight_move_indexes: [Option<u8>; 8] = [
                index.checked_sub(6), index.checked_sub(10), index.checked_sub(15),
                index.checked_sub(17), index.checked_add(6), index.checked_add(10),
                index.checked_add(15), index.checked_add(17)
            ];

            // 0 = goes left
            // 1 = goes right
            // 2 = goes left twice
            // 3 = goes right twice
            let chance: [u8; 8] = [3, 2, 1, 0, 2, 3, 0, 1];

            // For anything that subtracts
            for i in 0..8 {
                match knight_move_indexes[i] {
                    Some(val) => {

                        if val > 63 {
                            continue;
                        }

                        if chance[i] == 0 {
                            if val % 8 == 7 {
                                continue;
                            }
                        } else if chance[i] == 1 {
                            if val % 8 == 0 {
                                continue;
                            }
                        } else if chance[i] == 2 {
                            let column = val % 8;
                            if column == 7 || column == 6 {
                                continue;
                            }
                        } else if chance[i] == 3 {
                            let column = val % 8;
                            if column == 0 || column == 1 {
                                continue;
                            }
                        }


                        if Board::get_board_state_from_position(board, &val) {
                            match board.board_state[usize::from(val)] {
                                Some(piece) => {
                                    if piece.is_white != self.is_white {
                                        moves.push(MoveType::Standard(*index,val, self.is_white));
                                    }
                                },
                                None => panic!("Should never happen")
                            }
                        }
                        else {
                            moves.push(MoveType::Standard(*index,val, self.is_white));
                        }
                    },
                    None => {}
                }
            }
            moves
        }

        pub fn queen_moves(&self, board: &Board, index: &u8) -> Vec<MoveType> {
            let mut moves: Vec<MoveType> = self.bishop_moves(board, index);
            moves.append(&mut self.rook_moves(board, index));

            moves
        }


    }



}
