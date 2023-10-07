pub mod piece {
    use std::fmt;
    use std::fmt::{Display, Formatter, write};
    use std::path::is_separator;
    use crate::board::board::Board;
    use crate::board::board::MoveType;
    use crate::game::game::KingCapture;
    use crate::piece::piece::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};

    #[derive(Debug, Clone, Copy)]
    pub enum PieceType {
        Pawn, // 001
        Rook, // 010
        Knight, // 011
        Bishop, // 100
        Queen, // 101
        King // 110
    }

    impl PieceType {
        pub fn get_piece_type_value(&self, is_mg: bool) -> i32 {
            if is_mg {
                match self {
                    Pawn => 124,
                    Knight => 781,
                    Bishop => 825,
                    Rook => 1276,
                    Queen => 2538,
                    _ => 0
                }
            } else {
                match self {
                    Pawn => 206,
                    Knight => 854,
                    Bishop => 915,
                    Rook => 1380,
                    Queen => 2682,
                    _ => 0
                }
            }
        }

    }

    impl fmt::Display for PieceType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Pawn => write!(f, "P"),
                Rook => write!(f, "R"),
                Knight => write!(f, "N"),
                Bishop => write!(f, "B"),
                Queen => write!(f, "Q"),
                King => write!(f, "K")
            }
        }
    }
    impl PartialEq for PieceType {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Pawn => match other {
                    Pawn => true,
                    _ => false
                },
                Rook => match other {
                    Rook => true,
                    _ => false
                },
                Knight => match other {
                    Knight => true,
                    _ => false
                },
                Bishop => match other {
                    Bishop => true,
                    _ => false
                },
                Queen => match other {
                    Queen => true,
                    _ => false
                },
                King => match other {
                    King => true,
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
                true => write!(f, "W{}", &self.piece_type.to_string()),
                false => write!(f, "B{}", &self.piece_type.to_string())
            }
        }
    }

    impl Piece {
        pub fn u64_to_piece(val: &u64) -> Option<Piece> {
            match val {
                0 =>  None,
                1 =>  Some(Piece { is_white: true, piece_type: Pawn}),
                2 =>  Some(Piece { is_white: true, piece_type: Rook}),
                3 =>  Some(Piece { is_white: true, piece_type: Knight}),
                4 =>  Some(Piece { is_white: true, piece_type: Bishop}),
                5 =>  Some(Piece { is_white: true, piece_type: Queen}),
                6 =>  Some(Piece { is_white: true, piece_type: King}),
                7 =>  Some(Piece { is_white: false, piece_type: Pawn}),
                8 =>  Some(Piece { is_white: false, piece_type: Rook}),
                9 =>  Some(Piece { is_white: false, piece_type: Knight}),
                10 => Some(Piece { is_white: false, piece_type: Bishop}),
                11 => Some(Piece { is_white: false, piece_type: Queen}),
                12 => Some(Piece { is_white: false, piece_type: King}),
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

        pub fn get_value(&self, is_mg: bool) -> i32 {
            self.piece_type.get_piece_type_value(is_mg)
        }

        pub fn get_moves(&self, board: &Board, index: &u8, moves: &mut Vec<MoveType>, defence: &mut Vec<MoveType>, kings_capture: &mut Vec<KingCapture>) -> usize {
            match self.piece_type {
                Pawn => self.pawn_moves(board, index, moves, defence, kings_capture),
                Rook => self.rook_moves(board, index, moves, defence, kings_capture, Rook),
                Bishop => self.bishop_moves(board, index, moves, defence, kings_capture,Bishop),
                Knight => self.knight_moves(board, index, moves, defence, kings_capture),
                Queen => self.queen_moves(board, index, moves, defence, kings_capture),
                _ => 0
            }
        }

        fn pawn_moves(&self, board: &Board, index: &u8, moves: &mut Vec<MoveType>, defence: &mut Vec<MoveType>, kings_capture: &mut Vec<KingCapture>) -> usize {
            let mut counter = 0;

            // Move straight
            let straight_move = if self.is_white {index.checked_sub(8)} else {index.checked_add(8)};
            match straight_move {
                None => (),
                Some(move_val) => {
                    if !Board::get_board_state_from_position(board, &move_val) {
                        if move_val < 64 {
                            moves.push(MoveType::Standard(*index, move_val, self.is_white));
                            counter += 1;
                        }
                    }
                }
            }


            let should_check_left = if self.is_white {*index % 8 != 0} else {*index % 8 != 7};
            let should_check_right = if self.is_white {*index % 8 != 7} else {*index % 8 != 0};

            // Move diagonal left
            if should_check_left {
                let diagonal_left = if self.is_white { index.checked_sub(9) } else { index.checked_add(9) };
                match diagonal_left {
                    None => (),
                    Some(move_val) => {
                        if Board::get_board_state_from_position(board, &move_val) {
                            match board.board_state[usize::from(move_val)] {
                                Some(piece) => {
                                    if piece.is_white != self.is_white && move_val < 64 {
                                        if piece.piece_type == King {
                                            kings_capture.push(KingCapture {
                                                piece: *self,
                                                position: *index
                                            })
                                        }
                                        moves.push(MoveType::Capture(Pawn, *index, move_val, piece.piece_type, self.is_white));
                                        counter += 1;
                                    } else {
                                        if move_val < 64 {
                                            defence.push(MoveType::Defend(Pawn, *index, move_val, piece.piece_type, self.is_white));
                                            counter += 1;
                                        }
                                    }
                                },
                                None => panic!("Should never happen")
                            }
                        } else {
                            moves.push(MoveType::Attack(Pawn, *index, move_val, false, self.is_white));
                        }
                    }
                }
            }

            // Move diagonal right
            if should_check_right {
                let diagonal_right = if self.is_white { index.checked_sub(7) } else { index.checked_add(7) };
                match diagonal_right {
                    None => (),
                    Some(move_value) => {
                        let is_on_edge = if !self.is_white { *index % 8 != 0 } else { *index % 8 != 7 };
                        if is_on_edge {
                            if Board::get_board_state_from_position(board, &move_value) {
                                match board.board_state[usize::from(move_value)] {
                                    Some(piece) => {
                                        if piece.is_white != self.is_white && move_value < 64 {
                                            if piece.piece_type == King {
                                                kings_capture.push(KingCapture {
                                                    piece: *self,
                                                    position: *index
                                                })
                                            }
                                            moves.push(MoveType::Capture(Pawn, *index, move_value, piece.piece_type, self.is_white));
                                            counter += 1;
                                        } else {
                                            if move_value < 64 {
                                                defence.push(MoveType::Defend(Pawn, *index, move_value, piece.piece_type, self.is_white));
                                                counter += 1;
                                            }
                                        }
                                    },
                                    None => panic!("Should never happen")
                                }
                            } else {
                                moves.push(MoveType::Attack(Pawn, *index, move_value, false, self.is_white));
                            }
                        }
                    }
                }
            }

            // Double move
            if self.is_white && *index >= 48 && *index <= 55 {
                let double_move = *index - 16;
                if !Board::get_board_state_from_position(board, &double_move) {
                    moves.push(MoveType::Standard(*index, double_move, self.is_white));
                    counter += 1;
                }

            }
            else if !self.is_white && *index >= 8 && *index <= 15 {
                let double_move = *index + 16;
                if !Board::get_board_state_from_position(board, &double_move) {
                    moves.push(MoveType::Standard(*index, double_move, self.is_white));
                    counter += 1;
                }
            }

            counter
        }

        fn rook_move(&self, board: &Board, from: u8, index: u8, count: &u8, piece_type: PieceType) -> (u8, Option<MoveType>) {

            if Board::get_board_state_from_position(board, &index) {
                match board.board_state[usize::from(index)] {
                    Some(piece) => {
                        if piece.is_white != self.is_white {
                            let rook_move = if *count == 0 {
                                MoveType::Capture(piece_type, from,index, piece.piece_type,self.is_white)
                            } else {
                                MoveType::FutureMove(piece_type,from,index, self.is_white)
                            };
                            return (1, Some(rook_move))
                        }
                        else {
                            if *count == 0 {
                                return (2, Some(MoveType::Defend(piece_type, from,index, piece.piece_type,self.is_white)))
                            }
                            return (2, None)
                        }
                    },
                    None => panic!("Should never happen")
                }
            } else {
                let rook_move =
                    if *count == 0 {
                        MoveType::Attack(piece_type, from,index, true,self.is_white)
                    } else {
                        MoveType::FutureMove(piece_type,from,index, self.is_white)
                    };
                return (0, Some(rook_move))
            }
        }

        pub fn rook_moves(
            &self,
            board: &Board,
            index: &u8,
            moves: &mut Vec<MoveType>,
            defence: &mut Vec<MoveType>,
            kings_capture: &mut Vec<KingCapture>,
            piece_type: PieceType
        ) -> usize {

            let mut counter = 0;

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
                            let (added, rook_up_move) =
                                self.rook_move(board, *index, up_index, &count_up, piece_type);
                            count_up += added;
                            match rook_up_move {
                                Some(move_type) =>
                                    match move_type {
                                        MoveType::Defend(_, _, _, _, _) => {defence.push(move_type); counter += 1},
                                        MoveType::Capture(_p, _f, _t, cp, _c) => {
                                            if cp == King {
                                                kings_capture.push(KingCapture {
                                                    piece: *self,
                                                    position: *index
                                                })
                                            }
                                            moves.push(move_type);
                                            counter += 1;
                                        }
                                        _ => {moves.push(move_type); counter += 1}
                                    },
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
                                let (added, rook_down_move) =
                                    self.rook_move(board, *index, down_index, &count_down, piece_type);
                                count_down += added;
                                match rook_down_move {
                                    Some(move_type) =>
                                        match move_type {
                                            MoveType::Defend(_, _, _, _, _) => {defence.push(move_type); counter += 1},
                                            MoveType::Capture(_p, _f, _t, cp, _c) => {
                                                if cp == King {
                                                    kings_capture.push(KingCapture {
                                                        piece: *self,
                                                        position: *index
                                                    })
                                                }
                                                moves.push(move_type);
                                                counter += 1;
                                            }
                                            _ => { moves.push(move_type); counter += 1}
                                        },
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
                                let (added, rook_right_move) =
                                    self.rook_move(board, *index, right_index, &count_right, piece_type);
                                count_right += added;
                                match rook_right_move {
                                    Some(move_type) =>
                                        match move_type {
                                            MoveType::Defend(_, _, _, _, _) => {defence.push(move_type); counter += 1},
                                            MoveType::Capture(_p, _f, _t, cp, _c) => {
                                                if cp == King {
                                                    kings_capture.push(KingCapture {
                                                        piece: *self,
                                                        position: *index
                                                    })
                                                }
                                                moves.push(move_type);
                                                counter += 1;
                                            }
                                            _ => { moves.push(move_type); counter += 1}
                                        },
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
                                let (added, rook_left_move) =
                                    self.rook_move(board, *index, left_index, &count_left, piece_type);
                                count_left += added;
                                match rook_left_move {
                                    Some(move_type) =>
                                        match move_type {
                                            MoveType::Defend(_, _, _, _, _) => {defence.push(move_type); counter += 1},
                                            MoveType::Capture(_p, _f, _t, cp, _c) => {
                                                if cp == King {
                                                    kings_capture.push(KingCapture {
                                                        piece: *self,
                                                        position: *index
                                                    })
                                                }
                                                moves.push(move_type);
                                                counter += 1;
                                            }
                                            _ => { moves.push(move_type); counter += 1}
                                        },
                                    None => {}
                                }
                            }
                        },
                        None => { count_left = 2; }
                    }
                }
            }

            counter
        }

        pub fn king_moves(
            &self,
            board: &Board,
            index: &u8,
            moves: &mut Vec<MoveType>,
            defence: &mut Vec<MoveType>
        ) -> usize {

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
                                        moves.push(
                                            MoveType::Capture(
                                                King,
                                                *index,
                                                val,
                                                piece.piece_type,
                                                self.is_white
                                            )
                                        );
                                    }
                                    else {
                                        defence.push(
                                            MoveType::Defend(
                                                King,
                                                *index,
                                                val,
                                                piece.piece_type,
                                                self.is_white
                                            )
                                        );
                                    }
                                },
                                None => panic!("Should never happen")
                            }
                        }
                        else {
                            moves.push(
                                MoveType::Attack(King, *index,val, true,self.is_white)
                            );
                        }
                    },
                    None => {}
                }
            }

            0
        }

        fn bishop_move(&self, board: &Board, from: u8, index: u8, count: &u8, piece_type: PieceType) -> (u8, Option<MoveType>) {
            if Board::get_board_state_from_position(board, &index) {
                match board.board_state[usize::from(index)] {
                    Some(piece) => {
                        if piece.is_white != self.is_white {
                            let bishop_move =
                                if *count == 0 {
                                    MoveType::Capture(piece_type, from,index, piece.piece_type,self.is_white)
                                } else {
                                    MoveType::FutureMove(piece_type, from,index, self.is_white)
                                };
                            return (*count + 1, Some(bishop_move))
                        }
                        else {
                            if *count == 0 {
                                return (2, Some(MoveType::Defend(piece_type, from,index, piece.piece_type,self.is_white)))
                            }
                            return (2, None)
                        }
                    },
                    None => panic!("Should never happen")
                }
            } else {
                let bishop_move =
                    if *count == 0 {
                        MoveType::Attack(piece_type, from,index, true,self.is_white)
                    } else {
                        MoveType::FutureMove(piece_type,from,index, self.is_white)
                    };
                return (0, Some(bishop_move))
            }
        }

        pub fn bishop_moves(
            &self,
            board: &Board,
            index: &u8,
            moves: &mut Vec<MoveType>,
            defence: &mut Vec<MoveType>,
            kings_capture: &mut Vec<KingCapture>,
            piece_type: PieceType
        ) -> usize {

            let mut counter = 0;

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
                                let (added, bishop_up_right_move) = self.bishop_move(board, *index, up_index, &diagonal_up_right, piece_type);
                                diagonal_up_right += added;
                                match bishop_up_right_move {
                                    Some(move_type) => match move_type {
                                        MoveType::Defend(_, _, _, _, _) => {defence.push(move_type); counter += 1},
                                        MoveType::Capture(_p, _f, _t, cp, _c) => {
                                            if cp == King {
                                                kings_capture.push(KingCapture {
                                                    piece: *self,
                                                    position: *index
                                                })
                                            }
                                            moves.push(move_type);
                                            counter += 1;
                                        }
                                        _ => {moves.push(move_type); counter += 1}
                                    },
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
                                let (added, bishop_up_left_move) = self.bishop_move(board, *index, up_index, &diagonal_up_left, piece_type);
                                diagonal_up_left += added;
                                match bishop_up_left_move {
                                    Some(move_type) =>
                                        match move_type {
                                            MoveType::Defend(_, _, _, _, _) => {defence.push(move_type); counter += 1},
                                            MoveType::Capture(_p, _f, _t, cp, _c) => {
                                                if cp == King {
                                                    kings_capture.push(KingCapture {
                                                        piece: *self,
                                                        position: *index
                                                    })
                                                }
                                                moves.push(move_type);
                                                counter += 1;
                                            }
                                            _ => {moves.push(move_type); counter += 1}
                                        },
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
                                let (added, bishop_down_right_move) = self.bishop_move(board, *index, down_index, &diagonal_down_right, piece_type);
                                diagonal_down_right += added;
                                match bishop_down_right_move {
                                    Some(move_type) =>
                                        match move_type {
                                            MoveType::Defend(_, _, _, _, _) => {defence.push(move_type); counter += 1},
                                            MoveType::Capture(_p, _f, _t, cp, _c) => {
                                                if cp == King {
                                                    kings_capture.push(KingCapture {
                                                        piece: *self,
                                                        position: *index
                                                    })
                                                }
                                                moves.push(move_type);
                                                counter += 1;
                                            }
                                            _ => {moves.push(move_type); counter += 1}
                                        },
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
                                let (added, bishop_down_left_move) = self.bishop_move(board, *index, down_index, &diagonal_down_left, piece_type);
                                diagonal_down_left += added;
                                match bishop_down_left_move {
                                    Some(move_type) =>
                                        match move_type {
                                            MoveType::Defend(_, _, _, _, _) => {defence.push(move_type); counter += 1},
                                            MoveType::Capture(_p, _f, _t, cp, _c) => {
                                                if cp == King {
                                                    kings_capture.push(KingCapture {
                                                        piece: *self,
                                                        position: *index
                                                    })
                                                }
                                                moves.push(move_type);
                                                counter += 1;
                                            }
                                            _ => {moves.push(move_type); counter += 1}
                                        },
                                    None => {}
                                }
                            }
                        },
                        None => { diagonal_down_left = 2; }
                    }
                }
            }

            counter
        }


        pub fn knight_moves(
            &self,
            board: &Board,
            index: &u8,
            moves: &mut Vec<MoveType>,
            defence: &mut Vec<MoveType>,
            kings_capture: &mut Vec<KingCapture>
        ) -> usize {

            let mut counter = 0;

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
                                        if piece.piece_type == King {
                                            kings_capture.push(KingCapture {
                                                piece: *self,
                                                position: *index
                                            })
                                        }
                                        moves.push(
                                            MoveType::Capture(
                                                Knight,
                                                *index,
                                                val,
                                                piece.piece_type,
                                                self.is_white
                                            )
                                        );
                                        counter += 1;
                                    }
                                    else {
                                        defence.push(
                                            MoveType::Defend(
                                                Knight,
                                                *index,
                                                val,
                                                piece.piece_type,
                                                self.is_white
                                            )
                                        );
                                        counter += 1;
                                    }
                                },
                                None => panic!("Should never happen")
                            }
                        }
                        else {
                            moves.push(MoveType::Attack(Knight, *index,val, true,self.is_white));
                            counter += 1;
                        }
                    },
                    None => {}
                }
            }

            counter
        }

        pub fn queen_moves(
            &self,
            board: &Board,
            index: &u8,
            moves: &mut Vec<MoveType>,
            defence: &mut Vec<MoveType>,
            kings_capture: &mut Vec<KingCapture>
        ) -> usize {
            let mut counter = self.bishop_moves(board, index, moves, defence, kings_capture,Queen);
            counter += self.rook_moves(board, index, moves, defence, kings_capture,Queen);
            counter
        }


    }



}
