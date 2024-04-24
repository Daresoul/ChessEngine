pub mod move_list {
    use std::ptr::write_bytes;
    use crate::board::board::{BoardMove, Move};
    use crate::move_gen::move_gen::PieceType;

    #[derive(Clone)]
    pub struct MoveList {
        pub moves: [Move; 250],
        pub len: usize
    }

    #[derive(Clone)]
    pub struct AttackMoveList {
        pub attack_boards: [BoardMove; 16],
        pub len: usize
    }

    impl MoveList {

        pub fn init() -> MoveList {
            MoveList {
                moves: [Move::None; 250],
                len: 0
            }
        }
        pub fn iter(&self) -> std::slice::Iter<'_, Move> {
            self.moves[0..self.len].iter()
        }

        pub fn add(&mut self, m: Move) -> () {
            self.moves[self.len] = m;
            self.len += 1;
        }

        pub fn reset(&mut self) -> () {
            unsafe {
                write_bytes(self.moves.as_mut_ptr(), 0, self.moves.len());
            }
        }
    }

    impl AttackMoveList {

        pub fn init() -> AttackMoveList {
            AttackMoveList {
                attack_boards: [BoardMove {
                    attack_board: 0,
                    piece_type: PieceType::None,
                    position: 0,
                    white: false,
                }; 16],
                len: 0
            }
        }
        pub fn iter(&self) -> std::slice::Iter<'_, BoardMove> {
            self.attack_boards[0..self.len].iter()
        }

        pub fn add(&mut self, m: BoardMove) -> () {
            self.attack_boards[self.len] = m;
            self.len += 1;
        }

        pub fn reset(&mut self) -> () {
            unsafe {
                write_bytes(self.attack_boards.as_mut_ptr(), 0, self.attack_boards.len());
            }
        }
    }
}