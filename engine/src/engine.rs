pub(crate) mod engine {
    use std::cmp::{max, min, Ordering, PartialOrd};
    use std::fmt::{Debug};
    use std::ops::Sub;
    use PieceType::{BISHOP, KNIGHT, PAWN, QUEEN, ROOK};
    use crate::board::board::Move;
    use crate::debug::debug::print_board;
    use crate::game::game::Game;
    use crate::move_gen::move_gen::PieceType;
    use crate::move_gen::move_gen::PieceType::KING;
    use crate::print_moves;
    use crate::utils::utils::pop_lsb;

    #[derive(Debug)]
    pub struct Branch {
        pub m: Move,
        pub val: i32,
        pub leafs: usize
    }

    impl Eq for Branch {}

    impl PartialEq<Self> for Branch {
        fn eq(&self, other: &Self) -> bool {
            self.val == other.val
        }
    }

    impl PartialOrd<Self> for Branch {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.val.partial_cmp(&self.val)
        }
    }

    impl Ord for Branch {
        fn cmp(&self, other: &Self) -> Ordering {
            other.val.cmp(&self.val)
        }
    }

    #[derive(Clone)]
    pub(crate) struct Engine {
        pub(crate) i: i32,
    }

    impl PartialOrd for PieceType {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self {
                PieceType::None => match other {
                    PieceType::None => Some(Ordering::Equal),
                    _ => Some(Ordering::Less)
                }
                PAWN => match other {
                    PieceType::None => Some(Ordering::Greater),
                    PAWN => Some(Ordering::Equal),
                    _ => Some(Ordering::Less)
                }
                KNIGHT => match other {
                    PieceType::None => Some(Ordering::Greater),
                    PAWN => Some(Ordering::Greater),
                    KNIGHT => Some(Ordering::Equal),
                    BISHOP => Some(Ordering::Equal),
                    _ => Some(Ordering::Less)
                }
                BISHOP => match other {
                    PieceType::None => Some(Ordering::Greater),
                    PAWN => Some(Ordering::Greater),
                    KNIGHT => Some(Ordering::Equal),
                    BISHOP => Some(Ordering::Equal),
                    _ => Some(Ordering::Less)
                }
                ROOK => match other {
                    PieceType::None => Some(Ordering::Greater),
                    PAWN => Some(Ordering::Greater),
                    BISHOP => Some(Ordering::Greater),
                    KNIGHT => Some(Ordering::Greater),
                    ROOK => Some(Ordering::Equal),
                    _ => Some(Ordering::Less)
                }
                QUEEN => match other {
                    KING => Some(Ordering::Less),
                    QUEEN => Some(Ordering::Equal),
                    _ => Some(Ordering::Greater)
                }
                KING => match other {
                    KING => Some(Ordering::Equal),
                    _ => Some(Ordering::Less)
                }
            }
        }
    }

    impl Sub for &PieceType {
        type Output = i32;

        fn sub(self, rhs: Self) -> Self::Output {
            let points_lhs = match self {
                PieceType::None => 0,
                PAWN => 1,
                ROOK => 5,
                KING => 100,
                KNIGHT => 3,
                BISHOP => 3,
                QUEEN => 9
            };

            let points_rhs = match rhs {
                PieceType::None => 0,
                PAWN => 1,
                ROOK => 5,
                KING => 100,
                KNIGHT => 3,
                BISHOP => 3,
                QUEEN => 9
            };

            return points_lhs - points_rhs
        }
    }

    impl Engine {
        pub fn get_sorted_moves(game: &mut Game, is_maximizing: bool, depth: usize) -> (Vec<Branch>, usize) {
            let mut sorted_moves: Vec<Branch> = vec![];

            let mut alpha = i32::MIN;
            let mut beta = i32::MAX;

            let mut all_leafs = 0;

            let (mut moves, _, _) = game.get_all_moves();

            moves.sort_by(Self::ordering_moves);

            let mut i = 1;

            let mut stopped: bool = false;
            for mv in moves.iter() {
                if !stopped {
                    game.make_move(mv);

                    let (value, leafs) = Engine::alpha_beta_from_internet(game, !is_maximizing, alpha, beta, depth - 1);

                    game.undo_move();

                    sorted_moves.push(Branch {
                        m: *mv,
                        val: value,
                        leafs
                    });
                    all_leafs += leafs;

                    if is_maximizing {
                        alpha = alpha.max(value)
                    } else {
                        beta = beta.min(value);
                    }

                    if beta <= alpha {
                        stopped = true;
                    }
                    i += 1;
                } else {
                    sorted_moves.push(Branch {
                        m: *mv,
                        val: if is_maximizing { i32::MIN } else { i32::MAX },
                        leafs: 0
                    });
                }
            }

            sorted_moves.sort();

            if !is_maximizing {
                sorted_moves.reverse()
            }
            (sorted_moves, all_leafs)
        }

        fn check_if_over() {

        }


        /*
        if is_maximizing {
                let shifted_position = 1_u64.overflowing_shl(game.board.black_king_board.leading_zeros()).0;
                if attack_board_black & shifted_position > 1 {
                    return (i32::MIN, 0)
                }
            } else {
                let shifted_position = 1_u64.overflowing_shl(game.board.black_king_board.leading_zeros()).0;
                if attack_board_white & shifted_position > 1 {
                    return (i32::MAX, 0)
                }
            }
         */

        pub fn alpha_beta_from_internet(
            game: &mut Game,
            is_maximizing: bool,
            mut alpha: i32,
            mut beta: i32,
            depth: usize
        ) -> (i32, usize) {

            if depth == 0 {
                return (game.evaluate_board(), 1)
            }

            let (mut moves, attack_board_white, attack_board_black) = game.get_all_moves();

            moves.sort_by(Self::ordering_moves);

            if is_maximizing {
                let shifted_position = 1_u64.overflowing_shl(game.board.black_king_board.leading_zeros()).0;
                if attack_board_white & shifted_position > 1 {
                    return (i32::MIN, 0)
                }
            } else {
                let shifted_position = 1_u64.overflowing_shl(game.board.white_king_board.leading_zeros()).0;
                if attack_board_black & shifted_position > 1 {
                    return (i32::MAX, 0)
                }
            }

            if is_maximizing {
                let mut max_eval = i32::MIN;
                let mut total_leafs = 0;
                for m in moves {
                    game.make_move(&m);
                    let (eval, leaves) = Self::alpha_beta_from_internet(game, !is_maximizing, alpha, beta, depth - 1);
                    total_leafs += leaves;
                    game.undo_move();
                    max_eval = i32::max(eval, max_eval);
                    alpha = i32::max(alpha, eval);
                    if beta <= alpha {
                        break
                    }
                }

                return (max_eval, total_leafs)
            }else {
                let mut min_eval = i32::MAX;
                let mut total_leafs = 0;
                for m in moves {
                    game.make_move(&m);
                    let (eval, leaves) = Self::alpha_beta_from_internet(game, !is_maximizing, alpha, beta, depth - 1);
                    total_leafs += leaves;
                    game.undo_move();
                    min_eval = i32::min(eval, min_eval);
                    beta = i32::min(beta, eval);
                    if beta <= alpha {
                        break
                    }
                }

                return (min_eval, total_leafs)
            }
        }

        fn ordering_moves(a: &Move, b: &Move) -> Ordering {
            match a {
                Move::Standard(from, to, piece, color) => match b {
                    Move::None => Ordering::Less,
                    Move::Standard(_, _, _, _) => Ordering::Equal,
                    Move::Capture(_, _, _, _, _) => Ordering::Greater,
                    Move::Promotion(_, _, _, _, _) => Ordering::Greater,
                    Move::Castle(_, _, _) => Ordering::Greater
                }
                Move::Capture(from, to, p, cp, color) => {
                    match b {
                        Move::Standard(_, _, _, _) => Ordering::Less,
                        Move::Capture(_, _, p_, cp_, color_) => {
                            let first_diff = p - cp;
                            let second_diff = p_ - cp_;

                            if first_diff > second_diff {
                                return Ordering::Less
                            } else if first_diff == second_diff {
                                return Ordering::Equal
                            } else {
                                return Ordering::Greater
                            }
                        }
                        Move::Promotion(_, _, _, _, _) => Ordering::Greater,
                        _ => panic!("Move that cant be sorted with {:?} {:?}", a, b)
                    }
                }
                Move::Promotion(from, to, promotion_piece, cp, color) => {
                    match b {
                        Move::Standard(_, _, _, _) => Ordering::Less,
                        Move::Capture(_, _, _, _, _) => Ordering::Less,
                        Move::Promotion(from, to, promotion_piece_, cp_, color_) => {
                            if promotion_piece > promotion_piece_ {
                                return Ordering::Less
                            } else if promotion_piece < promotion_piece_ {
                                return Ordering::Greater
                            } else {
                                return Ordering::Equal
                            }
                        }
                        _ => panic!("Move that cant be sorted with {:?} {:?}", a, b)
                    }
                }
                _ => panic!("Move that cant be sorted with {:?} {:?}", a, b)
            }
        }
    }
}