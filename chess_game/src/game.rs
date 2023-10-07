
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

        fn piece_value_mg(&self, piece: Piece, is_mg: bool) -> (i32, i32) {
            let mut all_piece_value: i32 = 0;
            let mut piece_value: i32 = 0;
            let mut pawn_value: i32 = 0;


            if piece.is_white {
                match piece.piece_type {
                    Pawn => pawn_value += piece.get_value(is_mg),
                    _ => piece_value += piece.get_value(is_mg)
                }
                all_piece_value += piece.get_value(is_mg)
            } else {
                match piece.piece_type {
                    Pawn => pawn_value -= piece.get_value(is_mg),
                    _ => piece_value -= piece.get_value(is_mg)
                }
                all_piece_value -= piece.get_value(is_mg)
            }

            let is_bishop = if piece.piece_type == Bishop && piece.is_white {
                1
            } else if piece.piece_type == Bishop && !piece.is_white {
                2
            } else {
                0
            };

            (all_piece_value + piece_value + pawn_value, is_bishop)
        }

        fn psqt_mg(&self, index: usize, piece: Piece, is_mg: bool) -> i32 {
            let mut v = 0;

            let bonus =
                if is_mg {
                    [
                        [
                            [-175,-92,-74,-73],[-77,-41,-27,-15],[-61,-17,6,12],[-35,8,40,49],[-34,13,44,51],[-9,22,58,53],[-67,-27,4,37],[-201,-83,-56,-26]
                        ],
                        [
                            [-53,-5,-8,-23],[-15,8,19,4],[-7,21,-5,17],[-5,11,25,39],[-12,29,22,31],[-16,6,1,11],[-17,-14,5,0],[-48,1,-14,-23]
                        ],
                        [
                            [-31,-20,-14,-5],[-21,-13,-8,6],[-25,-11,-1,3],[-13,-5,-4,-6],[-27,-15,-4,3],[-22,-2,6,12],[-2,12,16,18],[-17,-19,-1,9]
                        ],
                        [
                            [3,-5,-5,4],[-3,5,8,12],[-3,6,13,7],[4,5,9,8],[0,14,12,5],[-4,10,6,8],[-5,6,10,8],[-2,-2,1,-2]
                        ],
                        [
                            [271,327,271,198],[278,303,234,179],[195,258,169,120],[164,190,138,98],[154,179,105,70],[123,145,81,31],[88,120,65,33],[59,89,45,-1]
                        ]
                    ]
                } else {
                    [
                    [[-96,-65,-49,-21],[-67,-54,-18,8],[-40,-27,-8,29],[-35,-2,13,28],[-45,-16,9,39],[-51,-44,-16,17],[-69,-50,-51,12],[-100,-88,-56,-17]],
                    [[-57,-30,-37,-12],[-37,-13,-17,1],[-16,-1,-2,10],[-20,-6,0,17],[-17,-1,-14,15],[-30,6,4,6],[-31,-20,-1,1],[-46,-42,-37,-24]],
                    [[-9,-13,-10,-9],[-12,-9,-1,-2],[6,-8,-2,-6],[-6,1,-9,7],[-5,8,7,-6],[6,1,-7,10],[4,5,20,-5],[18,0,19,13]],
                    [[-69,-57,-47,-26],[-55,-31,-22,-4],[-39,-18,-9,3],[-23,-3,13,24],[-29,-6,9,21],[-38,-18,-12,1],[-50,-27,-24,-8],[-75,-52,-43,-36]],
                    [[1,45,85,76],[53,100,133,135],[88,130,169,175],[103,156,172,172],[96,166,199,199],[92,172,184,191],[47,121,116,131],[11,59,73,78]]
                    ]
                };

            let pbonus =
                if is_mg {
                    [
                        [0,0,0,0,0,0,0,0],[3,3,10,19,16,19,7,-5],[-9,-15,11,15,32,22,5,-22],[-4,-23,6,20,40,17,4,-8],[13,0,-13,1,11,-2,-13,5],
                        [5,-12,-7,22,-8,-5,-15,-8],[-7,7,-3,-13,5,-16,10,-8],[0,0,0,0,0,0,0,0]]
                } else {
                    [
                        [0,0,0,0,0,0,0,0],[-10,-6,10,0,14,7,-5,-19],[-10,-10,-10,4,4,3,-6,-4],[6,-2,-8,-4,-13,-12,-10,-9],[10,5,4,-5,-5,-5,14,9],
                        [28,20,21,28,30,7,6,13],[0,-11,12,21,25,19,4,7],[0,0,0,0,0,0,0,0]
                    ]
                };

            let row: usize = index / 8;
            let column: usize = index % 8;
            if piece.is_white {
                v += match piece.piece_type {
                    Pawn => pbonus[7 - row][column],
                    Knight => bonus[0][7 - row][usize::min(column, 7 - column)],
                    Bishop => bonus[1][7 - row][usize::min(column, 7 - column)],
                    Rook => bonus[2][7 - row][usize::min(column, 7 - column)],
                    Queen => bonus[3][7 - row][usize::min(column, 7 - column)],
                    King => bonus[4][7 - row][usize::min(column, 7 - column)]
                }
            } else {
                v -= match piece.piece_type {
                    Pawn => pbonus[7 - row][column],
                    Knight => bonus[0][7 - row][usize::min(column, 7 - column)],
                    Bishop => bonus[1][7 - row][usize::min(column, 7 - column)],
                    Rook => bonus[2][7 - row][usize::min(column, 7 - column)],
                    Queen => bonus[3][7 - row][usize::min(column, 7 - column)],
                    King => bonus[4][7 - row][usize::min(column, 7 - column)]
                }
            }

            v
        }

        /*fn board(&self, x: usize, y: usize) -> char {
            match self.board.board_state[x + 8 * y] {
                Some(piece) =>
                    {
                        match piece.piece_type {
                            Pawn => if piece.is_white { "P" } else { "p" }
                            Queen => if piece.is_white { "Q" } else { "q" }
                            King => if piece.is_white { "K" } else { "k" }
                            Knight => if piece.is_white { "N" } else { "n" }
                            Bishop => if piece.is_white { "B" } else { "b" }
                            Rook => if piece.is_white { "R" } else { "r" }
                            _ => "e"
                        }
                    }
                None => "e"

            }
        }*/

        pub fn attacks(&self, game_info: &GameBoardInfo) -> (usize, usize) {
            let mut white = 0;
            let mut black = 0;
            for i in 0..12 {
                if i <= 5{
                    white += game_info.move_count[i];
                } else {
                    black += game_info.move_count[i];
                }
            }

            (white, black)
        }

        /*
        function imbalance_total(pos, square) {
            var v = 0;
            v += imbalance(pos) - imbalance(colorflip(pos));
            v += bishop_pair(pos) - bishop_pair(colorflip(pos));
            return (v / 16) << 0;
        }
         */

        /*
        function bishop_pair(pos, square) {
          if (bishop_count(pos) < 2) return 0;
          if (square == null) return 1438;
          return board(pos, square.x, square.y) == "B" ? 1 : 0;
        }
         */
        fn bishop_pair(&self, bishop_count_white: usize, bishop_count_black: usize) -> i32 {
            let mut v = 0;

            if bishop_count_white == 2 {
                v += 1438;
            }

            if bishop_count_black == 2 {
                v -= 1438;
            }

            return (v / 16) << 0;
        }


        fn mobility(&self, moves: &Vec<MoveType>, index: &usize) -> usize {
            let mut moves_count = 0;

            for m in moves.iter() {
                match m {
                    Attack(p, from, to, can_attack, color) => {
                        if usize::from(*from) == *index {
                            moves_count += 1;
                        }
                    }
                    Capture(p, from, to, cp, coloor) => {
                        if usize::from(*from) == *index {
                            moves_count += 1;
                        }
                    }
                    _ => ()
                }
            }

            return moves_count;
        }

        /*
            function mobility_mg(pos, square) {
              if (square == null) return sum(pos, mobility_mg);
              return mobility_bonus(pos, square, true);
            }
         */
        fn mobility_bonus(&self, index: &usize, piece: &Piece, moves: &Vec<MoveType>, is_mg: bool) -> i32 {
            let knight_bonus = if is_mg {[-62,-53,-12,-4,3,13,22,28,33]} else {[-81,-56,-31,-16,5,11,17,20,25]};
            let bishop_bonus = if is_mg {[-48,-20,16,26,38,51,55,63,63,68,81,81,91,98]} else {[-59,-23,-3,13,24,42,54,57,65,73,78,86,88,97]};
            let rook_bonus = if is_mg {[-60,-20,2,3,3,11,22,31,40,40,41,48,57,57,62]} else {[-78,-17,23,39,70,99,103,121,134,139,158,164,168,169,172]};
            let queen_bonus = if is_mg {[-30,-12,-8,-9,20,23,23,35,38,53,64,65,65,66,67,67,72,72,77,79,93,108,108,108,110,114,114,116]} else {[-48,-30,-7,19,40,55,59,75,78,96,96,100,121,127,131,133,136,141,147,150,151,168,168,171,182,182,192,219]};

            let mut mobility = self.mobility(moves, index);
            if mobility > 0 {
                mobility = mobility - 1
            }
            //println!("{}: {}", piece, mobility);
            match piece.piece_type {
                Knight => return if mobility >= knight_bonus.len() {knight_bonus[knight_bonus.len() - 1]} else {knight_bonus[mobility]},
                Bishop => return if mobility >= bishop_bonus.len() {bishop_bonus[bishop_bonus.len() - 1]} else {bishop_bonus[mobility]},
                Rook => return if mobility >= rook_bonus.len() {rook_bonus[rook_bonus.len() - 1]} else {rook_bonus[mobility]},
                Queen => return if mobility >= queen_bonus.len() {queen_bonus[queen_bonus.len() - 1]} else {queen_bonus[mobility]},
                _ => return 0
            };
        }

        /*
        function imbalance(pos, square) {
          if (square == null) return sum(pos, imbalance);
          var qo = [[0],[40,38],[32,255,-62],[0,104,4,0],[-26,-2,47,105,-208],[-189,24,117,133,-134,-6]];
          var qt = [[0],[36,0],[9,63,0],[59,65,42,0],[46,39,24,-24,0],[97,100,-42,137,268,0]];
          var j = "XPNBRQxpnbrq".indexOf(board(pos, square.x, square.y));
          if (j < 0 || j > 5) return 0;
          var bishop = [0, 0], v = 0;
          for (var x = 0; x < 8; x++) {
            for (var y = 0; y < 8; y++) {
              var i = "XPNBRQxpnbrq".indexOf(board(pos, x, y));
              if (i < 0) continue;
              if (i == 9) bishop[0]++;
              if (i == 3) bishop[1]++;
              if (i % 6 > j) continue;
              if (i > 5) v += qt[j][i-6];
                    else v += qo[j][i];
            }
          }
          if (bishop[0] > 1) v += qt[j][0];
          if (bishop[1] > 1) v += qo[j][0];
          return v;
        }
         */
        fn imbalance(&self, piece: &Piece) -> i32 {
            0
        }

        pub fn evaluate_board(&self, tr: &TurnResult) -> i32 {
            let mut score: i32 = 0;
            let mut white_bishop = 0;
            let mut black_bishop = 0;

            let (white_attack, black_attack) = self.attacks(&tr.gbi);

            score += i32::try_from(white_attack).unwrap() - i32::try_from(black_attack).unwrap();

            for i in 0..64 {
                match self.board.board_state[i] {
                    Some(piece) => {
                        let (val, bishop) = self.piece_value_mg(piece, true);
                        score += val;
                        score += self.psqt_mg(i, piece, true);

                        if bishop == 1 { white_bishop += 1; } else if bishop == 2 { black_bishop += 1; }

                        if piece.is_white {
                            score += self.mobility_bonus(&i, &piece, &tr.white_moves, true);
                        } else {
                            score -= self.mobility_bonus(&i, &piece, &tr.black_moves, true);
                        }
                    },
                    None => ()
                }
            }

            score += self.bishop_pair(white_bishop, black_bishop);

            score
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