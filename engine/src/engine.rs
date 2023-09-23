pub(crate) mod engine {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fmt::{Debug};
    use std::iter::once_with;
    use chess_game::board::board::MoveType;
    use chess_game::game::game::Game;
    use chess_game::piece::piece::PieceType::Pawn;
    use crate::game_tree::game_tree::GameTree;

    #[derive(Debug)]
    pub struct Branch {
        pub m: MoveType,
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
        pub(crate) fn build_tree(game: & mut Game, parent: &mut GameTree, maximizing_player: bool, depth: usize) -> i32 {
            let is_white_turn = game.is_white_turn;

            let (all_moves, defence_moves) = game.get_all_moves();

            if depth == 0 {
                return game.evaluate_board(&all_moves);
            }

            if is_white_turn == maximizing_player {
                let mut max_eval = i32::MIN;

                for m in all_moves.iter() {
                    if Engine::should_skip_move(*m, is_white_turn) {
                        continue;
                    }

                    let mut new_game = game.clone();
                    new_game.make_move(&m);

                    let mut new_node: GameTree = GameTree::new(Some(*m));
                    let eval = Engine::build_tree(&mut new_game, &mut new_node, !maximizing_player, depth - 1);

                    let board_eval = new_game.evaluate_board(&all_moves);

                    new_node.set_board_evals(board_eval, eval);
                    parent.add(new_node);

                    max_eval = i32::max(max_eval, eval);
                }

                return max_eval
            }
            else {
                let mut min_eval = i32::MAX;

                for m in all_moves.iter() {
                    if Engine::should_skip_move(*m, is_white_turn) {
                        continue;
                    }

                    let mut new_game = game.clone();
                    new_game.make_move(&m);

                    let mut new_node: GameTree = GameTree::new(Some(*m));
                    let eval = Engine::build_tree(&mut new_game, &mut new_node, !maximizing_player, depth - 1);

                    let board_eval = new_game.evaluate_board(&all_moves);

                    new_node.set_board_evals(board_eval, eval);
                    parent.add(new_node);

                    min_eval = i32::min(min_eval, eval);
                }

                return min_eval
            }
        }

        pub fn get_sorted_moves(game: &mut Game, map: &mut HashMap<u64, i32>, is_maximizing: bool, depth: usize, with_sorting: bool) -> (Vec<Branch>, usize) {
            let mut sorted_moves: Vec<Branch> = vec![];

            let mut alpha = i32::MIN;
            let mut beta = i32::MAX;

            let mut all_leafs = 0;

            let (mut all_moves, defence_moves) = game.get_all_moves();

            //all_moves.sort_by(Engine::middle_game_sort());

            if is_maximizing {
                for mv in all_moves.iter() {
                    if Engine::should_skip_move(*mv, game.is_white_turn) {
                        continue
                    }

                    let mut new_game = game.clone();
                    new_game.make_move(mv);

                    let (value, leafs) = Engine::alpha_beta_from_internet(&new_game, map, !is_maximizing, alpha, beta, depth, with_sorting);

                    sorted_moves.push(Branch {
                        m: *mv,
                        val: value,
                        leafs
                    });
                    all_leafs = leafs;

                    alpha = alpha.max(value);

                    if beta <= alpha {
                        break;
                    }
                }
            } else {
                for mv in all_moves.iter() {
                    if Engine::should_skip_move(*mv, game.is_white_turn) {
                        continue
                    }

                    let mut new_game = game.clone();
                    new_game.make_move(mv);

                    let (value, leafs) = Engine::alpha_beta_from_internet(&new_game, map, !is_maximizing, alpha, beta, depth, with_sorting);

                    sorted_moves.push(Branch {
                        m: *mv,
                        val: value,
                        leafs
                    });
                    all_leafs = leafs;

                    beta = beta.min(value);

                    if beta <= alpha {
                        break;
                    }
                }
            }

            sorted_moves.sort();

            if !is_maximizing {
                sorted_moves.reverse()
            }
            (sorted_moves, all_leafs)
        }

        pub fn alpha_beta_from_internet(
            game: &Game,
            map: &mut HashMap<u64, i32>,
            is_maximizing: bool,
            mut alpha: i32,
            mut beta: i32,
            depth: usize,
            with_sorting: bool
        ) -> (i32, usize) {

            let is_white_turn = game.is_white_turn;

            let (mut all_moves, _) = game.get_all_moves();

            if depth == 0 {
                let board_hash = game.board.compute_hash();
                match map.get(&board_hash) {
                    Some(v) => {
                        return (*v, 1)
                    },
                    None => {
                        let value = game.evaluate_board(&all_moves);
                        map.insert(board_hash, value);
                        return (value, 1);
                    }
                }
            }

            if with_sorting {
                all_moves.sort_by(Engine::middle_game_sort());
            }

            if is_maximizing {
                let mut val = i32::MIN;
                let mut leaves = 0;

                for m in all_moves.iter() {
                    if Engine::should_skip_move(*m, is_white_turn) {
                        continue;
                    }

                    let mut new_game = *game;
                    new_game.make_move(m);

                    let (score, leave) = Engine::alpha_beta_from_internet(&new_game, map, !is_maximizing, alpha, beta, depth - 1, with_sorting);

                    val = val.max(score);
                    alpha = alpha.max(score);
                    leaves += leave;

                    if beta <= alpha {
                        break;
                    }
                }

                return (val, leaves)
            } else {
                let mut val = i32::MAX;
                let mut leaves = 0;

                for m in all_moves.iter() {
                    if Engine::should_skip_move(*m, is_white_turn) {
                        continue;
                    }

                    let mut new_game = game.clone();
                    new_game.make_move(m);

                    let (score, leave) = Engine::alpha_beta_from_internet(&new_game, map, !is_maximizing, alpha, beta, depth - 1, with_sorting);

                    val = val.min(score);
                    beta = beta.min(score);
                    leaves += leave;

                    if beta <= alpha {
                        break;
                    }
                }

                return (val, leaves);
            }
        }

        fn should_skip_move(m: MoveType, is_white_turn: bool) -> bool {
            match m {
                MoveType::Standard(_, _, c) => { if c != is_white_turn {return true} }
                MoveType::FutureMove(_, _, _, _c) => {return true}
                MoveType::Castle(_, _, _, _, c) => { if c != is_white_turn {return true} }
                MoveType::Promotion(_, _, _, c) => { if c != is_white_turn {return true} }
                MoveType::Attack(_, _, _, can_move, c) => { if c != is_white_turn || !can_move {return true} }
                MoveType::Capture(_, _, _, _, c) => { if c != is_white_turn {return true} }
                MoveType::Defend(_, _, _, _, _c) => { panic!("UMMMMMM DEFENCE HERE?") }
            }
            false
        }

        fn middle_game_sort() -> fn(&MoveType, &MoveType) -> Ordering {
            return |m1, m2|
                match m1 {
                    MoveType::Standard(from1, _, _) => {
                        match m2 {
                            MoveType::Standard(from2, _, _) => {
                                Ordering::Equal
                            }
                            MoveType::Capture(_, _, _, _, _) => {
                                Ordering::Greater
                            }
                            MoveType::Attack(p, _, _, _, _) => {
                                Ordering::Greater
                            }
                            MoveType::Castle(_, _, _, _, _) => Ordering::Less,
                            _ => Ordering::Greater
                        }
                    },
                    MoveType::Attack(p, _, _, _, _) => {
                        match m2 {
                            MoveType::Standard(_, _, _) => {
                                Ordering::Greater
                            }
                            MoveType::Capture(_, _, _, _, _) => {
                                Ordering::Less
                            }
                            MoveType::Attack(pt, _, _, _, _) => {
                                let p1_val = p.get_piece_type_value(true);
                                let p2_val = pt.get_piece_type_value(true);

                                if p1_val < p2_val {
                                    Ordering::Greater
                                }
                                else if *p == Pawn {
                                    Ordering::Greater
                                }
                                else if p1_val == p2_val {
                                    Ordering::Equal
                                }
                                else {
                                    Ordering::Less
                                }
                            }
                            MoveType::Castle(_, _, _, _, _) => Ordering::Less,
                            _ => Ordering::Greater
                        }
                    }
                    MoveType::Capture(pt, _, _, cp, _) => {
                        match m2 {
                            MoveType::Standard(_, _, _) => {
                                Ordering::Greater
                            }
                            MoveType::Capture(cpt, _, _, ccp, _) => {
                                Ordering::Equal
                            }
                            MoveType::Attack(p, _, _, _, _) => {
                                Ordering::Greater
                            }
                            MoveType::Castle(_, _, _, _, _) => Ordering::Less,
                            _ => Ordering::Greater
                        }
                    }
                    MoveType::Castle(_, _, _, _, _) => {
                        Ordering::Greater
                    }
                    _ => Ordering::Greater
                }
        }

    }
}