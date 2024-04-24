pub mod game {
    use crate::board::board::{Board, BoardMove, Move};
    use crate::move_gen::move_gen::MoveGen;
    use crate::move_list::move_list::{AttackMoveList, MoveList};

    #[derive(Clone)]
    pub struct Game {
        pub board: Board,
        pub is_white_turn: bool,
        pub move_gen: MoveGen,
        pub white_attack_boards: AttackMoveList,
        pub white_moves: MoveList,
        pub black_attack_boards: AttackMoveList,
        pub black_moves: MoveList,
    }

    impl Game {
       pub fn new(is_white_turn: bool) -> Game {
            return Game {
                board: Board::new(),
                is_white_turn: is_white_turn,
                move_gen: MoveGen::init(),
                white_attack_boards: AttackMoveList::init(),
                white_moves: MoveList::init(),
                black_attack_boards: AttackMoveList::init(),
                black_moves: MoveList::init(),
            }
       }

        pub fn new_from_string(state: String, is_white_turn: bool) -> Game {
            let board: Board = Board::from_string(state);
            Game {
                board: board,
                is_white_turn: is_white_turn,
                move_gen: MoveGen::init(),
                white_attack_boards: AttackMoveList::init(),
                white_moves: MoveList::init(),
                black_attack_boards: AttackMoveList::init(),
                black_moves: MoveList::init(),
            }
        }

        pub fn get_all_moves(&mut self) -> () {

            let occupancy = self.board.get_board_value();
            let white_occupancy = self.board.get_white_occupancy();
            let black_occupancy = self.board.get_black_occupancy();

            self.black_attack_boards.reset();
            self.white_attack_boards.reset();

            if self.is_white_turn {self.white_moves.reset()} else {self.black_moves.reset()}

            self.board.get_moves(white_occupancy, black_occupancy, occupancy, true, &mut self.white_attack_boards, &self.move_gen);

            self.board.get_moves(black_occupancy, white_occupancy, occupancy, false, &mut self.black_attack_boards, &self.move_gen);

            if self.is_white_turn {
                self.board.attack_boards_to_moves(&self.white_attack_boards, &mut self.white_moves, true);
            } else {
                self.board.attack_boards_to_moves(&self.black_attack_boards, &mut self.white_moves,false);
            }
        }

        pub fn make_move(&mut self, m: &Move) -> () {
            match m {
                Move::Standard(pos, to, piece_type) => {
                    self.board.make_move(&piece_type, &pos, &to, self.is_white_turn);
                },
                Move::Promotion(_pos, _to, _piece_type) => {

                }
                _ => ()
            }

            self.is_white_turn = !self.is_white_turn
        }

    }

}