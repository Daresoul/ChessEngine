
pub mod game {
    use std::usize;
    use MoveType::{Defend, FutureMove, Promotion};
    use crate::board::board::{Board, MoveType};
    use crate::board::board::MoveType::{Attack, Capture, Castle, Standard};
    use crate::piece::piece::{Piece, PieceType};
    use crate::piece::piece::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
    use crate::game_board_info::game_board_info;
    use crate::game_board_info::game_board_info::GameBoardInfo;

    pub struct KingCapture {
        pub piece: Piece,
        pub position: u8
    }

    #[derive(Clone)]
    pub struct TurnResult {
        pub white_moves: Vec<MoveType>,
        pub black_moves: Vec<MoveType>,
        pub defence_moves: Vec<MoveType>,
        pub gbi: GameBoardInfo
    }

    impl TurnResult {
        pub fn new(
            white_moves: Vec<MoveType>,
            black_moves: Vec<MoveType>,
            defence_moves: Vec<MoveType>,
            gbi: GameBoardInfo
        ) -> TurnResult {
            TurnResult {
                white_moves,
                black_moves,
                defence_moves,
                gbi
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Game {
        pub board: Board,
        pub is_white_turn: bool,
        pub white_king_position: u8,
        pub black_king_position: u8,
        pub white_king_moved: bool,
        pub black_king_moved: bool,
        pub white_rook_left_moved: bool,
        pub white_rook_right_moved: bool,
        pub black_rook_left_moved: bool,
        pub black_rook_right_moved: bool,
    }

    impl Game {
       pub fn new(is_white_turn: bool) -> Game {
            return Game {
                board: Board::new(),
                is_white_turn: is_white_turn,
                white_king_moved: false,
                black_king_moved: false,
                white_rook_left_moved: false,
                white_rook_right_moved: false,
                black_rook_left_moved: false,
                black_rook_right_moved: false,
                white_king_position: 60,
                black_king_position: 4,
            }
       }

       pub fn new_from_arr(state: [Option<Piece>; 64], is_white_turn: bool) -> Game {
            Game {
                board: Board::new_from_arr(state),
                is_white_turn: is_white_turn,
                white_king_moved: false,
                black_king_moved: false,
                white_rook_left_moved: false,
                white_rook_right_moved: false,
                black_rook_left_moved: false,
                black_rook_right_moved: false,
                white_king_position: 60,
                black_king_position: 4,
            }
       }

        pub fn new_from_string(state: String, is_white_turn: bool) -> Game {
            let (white_king, black_king, board) = Game::transform_string_to_state(state);
            Game {
                board: Board::new_from_arr(board),
                is_white_turn: is_white_turn,
                white_king_moved: false,
                black_king_moved: false,
                white_rook_left_moved: false,
                white_rook_right_moved: false,
                black_rook_left_moved: false,
                black_rook_right_moved: false,
                white_king_position: white_king,
                black_king_position: black_king,
            }
        }

        // Forsyth–Edwards Notation
        pub fn transform_string_to_state(state: String) -> (u8, u8, [Option<Piece>; 64]) {
            let mut board: [Option<Piece>; 64] = [None; 64];
            let mut white_king = 0;
            let mut black_king = 0;
            let mut board_index: u8 = 0;

            for c in state.chars() {
                if board_index > 63 {
                    break;
                }
                match c {
                    '1' => board_index += 1,
                    '2' => board_index += 2,
                    '3' => board_index += 3,
                    '4' => board_index += 4,
                    '5' => board_index += 5,
                    '6' => board_index += 6,
                    '7' => board_index += 7,
                    '8' => board_index += 8,
                    'P' => {board[usize::from(board_index)] = Some(Piece { piece_type: Pawn, is_white: true}); board_index += 1},
                    'p' => {board[usize::from(board_index)] = Some(Piece { piece_type: Pawn, is_white: false}); board_index += 1},
                    'R' => {board[usize::from(board_index)] = Some(Piece { piece_type: Rook, is_white: true}); board_index += 1},
                    'r' => {board[usize::from(board_index)] = Some(Piece { piece_type: Rook, is_white: false}); board_index += 1},
                    'N' => {board[usize::from(board_index)] = Some(Piece { piece_type: Knight, is_white: true}); board_index += 1},
                    'n' => {board[usize::from(board_index)] = Some(Piece { piece_type: Knight, is_white: false}); board_index += 1},
                    'B' => {board[usize::from(board_index)] = Some(Piece { piece_type: Bishop, is_white: true}); board_index += 1},
                    'b' => {board[usize::from(board_index)] = Some(Piece { piece_type: Bishop, is_white: false}); board_index += 1},
                    'Q' => {board[usize::from(board_index)] = Some(Piece { piece_type: Queen, is_white: true}); board_index += 1},
                    'q' => {board[usize::from(board_index)] = Some(Piece { piece_type: Queen, is_white: false}); board_index += 1},
                    'K' => {board[usize::from(board_index)] = Some(Piece { piece_type: King, is_white: true}); white_king = board_index; board_index += 1},
                    'k' => {board[usize::from(board_index)] = Some(Piece { piece_type: King, is_white: false}); black_king = board_index; board_index += 1},
                    _ => continue
                };
            }
            (white_king, black_king, board)
        }

        pub fn check_attack_castle_white(&self, moves: &Vec<MoveType>) -> (bool, bool) {
            let mut can_castle_left = true;
            let mut can_castle_right = true;


            for single_move in moves.iter() {
                match single_move {
                    Attack(_, _from, to, can_move, color) => {
                        if !*color && *can_move {
                            if *to == 60 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 58 || *to == 59 {
                                can_castle_left = false;
                            } else if *to == 61 || *to == 62 {
                                can_castle_right = false;
                            }
                        }
                    },
                    Promotion(_from, to, _, color) => {
                        if !*color {
                            if *to == 60 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 58 || *to == 59 {
                                can_castle_left = false;
                            } else if *to == 61 || *to == 62 {
                                can_castle_right = false;
                            }
                        }
                    },
                    Capture(_, from, to, cp, color) => {
                        if !*color {
                            if *to == 60 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 58 || *to == 59 {
                                can_castle_left = false;
                            } else if *to == 61 || *to == 62 {
                                can_castle_right = false;
                            }
                        }
                    }
                    _ => continue
                }
            }

            (can_castle_left,can_castle_right)
        }

        pub fn castle_white(&self, all_moves: &Vec<MoveType>) -> Vec<MoveType> {
            let mut moves = vec![];


            let (can_left, can_right) = self.check_attack_castle_white(all_moves);

            // Check spaces between king and rook left are empty
            if can_left && !self.white_rook_left_moved {
                if !Board::get_board_state_from_position(&self.board, &57) &&
                    !Board::get_board_state_from_position(&self.board, &58) &&
                    !Board::get_board_state_from_position(&self.board, &59) {
                    moves.push(Castle(56, 59, 60, 58, true));
                }
            }

            // Check spaces between king and rook right are empty
            if can_right && !self.white_rook_right_moved {
                if !Board::get_board_state_from_position(&self.board, &61) &&
                    !Board::get_board_state_from_position(&self.board, &62) {
                    moves.push(Castle(63, 61, 60, 62, true));
                }
            }
            moves
        }

        pub fn check_attack_castle_black(&self, moves: &Vec<MoveType>) -> (bool, bool) {
            let mut can_castle_left = true;
            let mut can_castle_right = true;

            for single_move in moves.iter() {
                match single_move {
                    Attack(_, _from, to, can_move, color) => {
                        if *color && *can_move {
                            if *to == 4 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 2 || *to == 3 {
                                can_castle_left = false;
                            } else if *to == 5 || *to == 6 {
                                can_castle_right = false;
                            }
                        }
                    },
                    Promotion(_from, to, _, color) => {
                        if *color {
                            if *to == 4 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 2 || *to == 3 {
                                can_castle_left = false;
                            } else if *to == 5 || *to == 6 {
                                can_castle_right = false;
                            }
                        }
                    },
                    Capture(_, from, to, cp, color) => {
                        if *color {
                            if *to == 4 {
                                can_castle_left = false;
                                can_castle_right = false;
                                break;
                            }
                            if *to == 2 || *to == 3 {
                                can_castle_left = false;
                            } else if *to == 5 || *to == 6 {
                                can_castle_right = false;
                            }
                        }
                    }
                    _ => continue
                }
            }

            (can_castle_left,can_castle_right)
        }

        pub fn  castle_black(&self, all_moves: &Vec<MoveType>) -> Vec<MoveType> {
            let mut moves = vec![];


            let (can_left, can_right) = self.check_attack_castle_black(all_moves);

            match self.board.board_state[0] {
                Some(piece) => {
                    if piece.piece_type == Rook && !piece.is_white {
                        // Check spaces between king and rook left are empty
                        if can_left && !self.black_rook_left_moved {
                            if !Board::get_board_state_from_position(&self.board, &1) &&
                                !Board::get_board_state_from_position(&self.board, &2) &&
                                !Board::get_board_state_from_position(&self.board, &3) {
                                moves.push(Castle(0, 3, 4, 2, false));
                            }
                        }
                    }
                }
                None => ()
            }

            match self.board.board_state[7] {
                Some(piece) => {
                    if piece.piece_type == Rook && !piece.is_white {
                        // Check spaces between king and rook right are empty
                        if can_right && !self.black_rook_right_moved {
                            if !Board::get_board_state_from_position(&self.board, &5) &&
                                !Board::get_board_state_from_position(&self.board, &6) {
                                moves.push(Castle(7, 5, 4, 6, false));
                            }
                        }
                    }
                }
                None => ()
            }

            moves
        }

        pub fn get_all_moves(&self) -> TurnResult {
            let mut white_moves: Vec<MoveType> = vec![];
            let mut black_moves: Vec<MoveType> = vec![];
            let mut defence: Vec<MoveType> = vec![];
            let mut kings_captures: Vec<KingCapture> = vec![];
            white_moves.reserve(500);
            black_moves.reserve(500);
            defence.reserve(100);
            kings_captures.reserve(10);

            let mut game_info: GameBoardInfo = GameBoardInfo::new();


            for i in 0..64 {
                match self.board.board_state[i] {
                    Some(piece) => {
                        let mut moves = if piece.is_white { &mut white_moves } else { &mut black_moves };
                        let move_count = piece.get_moves(&self.board, &(i as u8), &mut moves, &mut defence, &mut kings_captures);

                        match piece.piece_type {
                            Pawn => if piece.is_white {
                                game_info.piece_count[0] += 1;
                                game_info.move_count[0] += move_count;
                            } else {
                                game_info.piece_count[6] += 1;
                                game_info.move_count[5] += move_count;
                            }
                            Bishop => if piece.is_white {
                                game_info.piece_count[1] += 1;
                                game_info.move_count[1] += move_count;
                            } else {
                                game_info.piece_count[7] += 1;
                                game_info.move_count[6] += move_count;
                            }
                            Knight => if piece.is_white {
                                game_info.piece_count[2] += 1;
                                game_info.move_count[2] += move_count;
                            } else {
                                game_info.piece_count[8] += 1;
                                game_info.move_count[7] += move_count;
                            }
                            Rook => if piece.is_white {
                                game_info.piece_count[3] += 1;
                                game_info.move_count[3] += move_count;
                            } else {
                                game_info.piece_count[9] += 1;
                                game_info.move_count[8] += move_count;
                            }
                            Queen => if piece.is_white {
                                game_info.piece_count[4] += 1;
                                game_info.move_count[4] += move_count;
                            } else {
                                game_info.piece_count[10] += 1;
                                game_info.move_count[9] += move_count;
                            }
                            King => if piece.is_white {
                                game_info.move_count[5] += 1
                            } else {
                                game_info.move_count[11] += 1
                            }
                        }
                    },
                    None => ()
                }
            }

            // check for castelling
            if self.is_white_turn && !self.white_king_moved && (!self.white_rook_left_moved || !self.white_rook_right_moved) {
                match self.board.board_state[60] {
                    Some(piece) => {
                        if piece.piece_type == King && piece.is_white == true {
                            let mut white_castle = self.castle_white(&black_moves);
                            if white_castle.len() > 0 {
                                white_moves.append(&mut white_castle);
                            }
                        }
                    },
                    None => ()
                }
            } else if !self.is_white_turn && !self.black_king_moved && (!self.black_rook_left_moved || !self.black_rook_right_moved) {
                match self.board.board_state[4] {
                    Some(piece) => {
                        if piece.piece_type == King && piece.is_white == false {
                            let mut black_castle = self.castle_black(&white_moves);
                            if black_castle.len() > 0 {
                                black_moves.append(&mut black_castle);
                            }
                        }
                    },
                    None => ()
                }
            }

            match self.board.board_state[usize::from(self.white_king_position)] {
                Some(piece) => {
                    piece.king_moves(&self.board, &self.white_king_position, &mut white_moves, &mut defence);
                    ()
                },
                _ => ()//panic!("White king not at correct position.")
            }

            match self.board.board_state[usize::from(self.black_king_position)] {
                Some(piece) => {
                    piece.king_moves(&self.board, &self.black_king_position, &mut black_moves, &mut defence);
                    ()
                },
                _ => ()//panic!("Black king not at correct position.")
            }

            if kings_captures.len() > 0 {
                game_info.is_check = true;
                if self.is_white_turn {
                    let mut new_white = vec![];

                    for m in white_moves.iter() {
                        match m {
                            FutureMove(_, _, _, _) => (),
                            Castle(_, _, _, _, _) => (),
                            Attack(_, _, _, false, _) => (),
                            _ => {
                                let mut newgame = self.clone();
                                newgame.make_move(m);
                                if !newgame.is_check(false) {
                                    new_white.push(*m)
                                }
                            }
                        }
                    }

                    white_moves = new_white
                } else {
                    let mut new_black = vec![];

                    for m in black_moves.iter() {
                        match m {
                            FutureMove(_, _, _, _) => (),
                            Castle(_, _, _, _, _) => (),
                            Attack(_, _, _, false, _) => (),
                            _ => {
                                let mut newgame = self.clone();
                                newgame.make_move(m);
                                if !newgame.is_check(true) {
                                    new_black.push(*m)
                                }
                            }
                        }
                    }

                    black_moves = new_black
                }
            }

            return TurnResult::new(white_moves, black_moves, defence, game_info)
        }

        fn is_check(&self, is_white: bool) -> bool {
            let mut moves = vec![];
            let mut kings_captures: Vec<KingCapture> = vec![];

            for i in 0..64 {
                match self.board.board_state[i] {
                    Some(piece) => {
                        if piece.is_white == is_white {
                            piece.get_moves(&self.board, &(i as u8), &mut moves, &mut vec![], &mut kings_captures);
                        }
                    },
                    None => ()
                }
            }

            for capture in kings_captures.iter() {
                if capture.piece.is_white == is_white {
                    return true
                }
            }

            return false
        }

        pub fn make_move(&mut self, chosen_move: &MoveType) -> bool {
            match chosen_move {
                Standard(from, to, _color) => {
                    self.board.make_move(usize::from(*from), usize::from(*to), chosen_move);
                    self.is_white_turn = !self.is_white_turn;
                    return true;
                },
                FutureMove(_p, _from, _to, _color) => {
                    panic!("Dont do future movedvdslkfgsdæljfkgædsj");
                },
                Promotion(from, to, piece, color) => {
                    self.board.make_move(usize::from(*from), usize::from(*to), chosen_move);
                    self.board.board_state[usize::from(*to)] = Some(Piece {piece_type: *piece, is_white: *color});
                    self.is_white_turn = !self.is_white_turn;
                    return true;
                },
                Castle(king_from, king_to, rook_from, rook_to, color) => {
                    self.board.make_move(usize::from(*king_from), usize::from(*king_to), chosen_move);
                    self.board.make_move(usize::from(*rook_from), usize::from(*rook_to), chosen_move);
                    self.is_white_turn = !self.is_white_turn;

                    if *color {
                        self.white_king_moved = true;
                        self.white_king_position = *king_to
                    } else {
                        self.black_king_moved = true;
                        self.black_king_position = *king_to
                    }
                    return true;
                }
                Attack(p, from, to, can_move, color) => {
                    if *can_move {
                        self.board.make_move(usize::from(*from), usize::from(*to), chosen_move);
                        self.is_white_turn = !self.is_white_turn;

                        self.rook_move(p, from, color);
                        self.king_move(p, color, to);
                        return true;
                    }
                    panic!("Piece cannot move there.")
                },
                Capture(p, from, to, cp, color) => {
                    self.board.make_move(usize::from(*from), usize::from(*to), chosen_move);
                    self.is_white_turn = !self.is_white_turn;

                    if *cp == Rook {
                        if *color {
                            if *to == 56 {
                                self.white_rook_left_moved = true;
                            } else if *to == 63 {
                                self.white_rook_right_moved = true;
                            }
                        } else {
                            if *to == 0 {
                                self.black_rook_left_moved = true;
                            } else if *to == 7 {
                                self.black_rook_right_moved = true;
                            }
                        }
                    }

                    self.rook_move(p, from, color);
                    self.king_move(p, color, to);
                    return true;
                },
                Defend(_p, _from, _to, _d, _color) => {
                    panic!("Dont do defend movedvdslkfgsdæljfkgædsj");
                },
            }
        }

        fn king_move(&mut self, p: &PieceType, color: &bool, to: &u8) {
            if *p == King {
                if *color {
                    self.white_king_position = *to;
                    self.white_king_moved = true;
                } else {
                    self.black_king_position = *to;
                    self.black_king_moved = true;
                }
            }
        }

        fn rook_move(&mut self, p: &PieceType, from: &u8, color: &bool) {
            if *p == Rook {
                if *color {
                    if *from == 56 {
                        self.white_rook_left_moved = true;
                    }

                    if *from == 63 {
                        self.white_rook_right_moved = true;
                    }
                } else {
                    if *from == 0 {
                        self.black_rook_left_moved = true;
                    }

                    if *from == 7 {
                        self.black_rook_right_moved = true;
                    }
                }
            }
        }

    }

}