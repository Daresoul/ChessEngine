pub(crate) mod engine {
    use std::cmp::Ordering;
    use std::fmt::{Debug};
    use crate::board::board::Move;
    use crate::game::game::Game;


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

    impl Engine {
        pub fn get_sorted_moves(game: &mut Game, is_maximizing: bool, depth: usize,) -> (Vec<Branch>, usize) {
            let mut sorted_moves: Vec<Branch> = vec![];

            let mut alpha = i32::MIN;
            let mut beta = i32::MAX;

            let mut all_leafs = 0;

            game.get_all_moves();
            let mut stopped: bool = false;

            let moves = if is_maximizing { game.white_moves } else { game.black_moves };
            let max = if is_maximizing {game.white_moves_len} else {game.black_moves_len};

            for (i, mv) in moves.iter().enumerate()  {
                if i == max {break;}
                if !stopped {
                    let mut new_game = game.clone();
                    new_game.make_move(mv);

                    let (value, leafs) = Engine::alpha_beta_from_internet(&mut new_game, !is_maximizing, alpha, beta, depth);

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

        pub fn alpha_beta_from_internet(
            game: &mut Game,
            is_maximizing: bool,
            mut alpha: i32,
            mut beta: i32,
            depth: usize
        ) -> (i32, usize) {
            let mut leaves = 0;
            let mut val = 0;

            if depth == 0 {
                return (game.evaluate_board(), 1)
            }

            game.get_all_moves();

            let moves = if is_maximizing { game.white_moves } else { game.black_moves };
            let max = if is_maximizing {game.white_moves_len} else {game.black_moves_len};

            for (i, m) in moves.iter().enumerate() {
                if i == max {break;}
                let mut new_game = game.clone();
                new_game.make_move(m);

                let (score, leave) = Engine::alpha_beta_from_internet(&mut new_game, !is_maximizing, alpha, beta, depth - 1);

                if is_maximizing {
                    val = val.max(score);
                    alpha = alpha.max(score);
                } else {
                    val = val.min(score);
                    beta = beta.min(score);
                }

                leaves += leave;

                if beta <= alpha {
                    break;
                }
            }

            return (val, leaves)
        }
    }
}