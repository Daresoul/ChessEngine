pub(crate) mod engine {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fmt::{Debug};
    use std::mem;
    use std::mem::size_of;
    use chess_game::board::board::MoveType;
    use chess_game::debug::debug::print_board;
    use chess_game::game::game::{Game, TurnResult};
    use chess_game::piece::piece::PieceType::Pawn;
    use crate::game_tree::game_tree::GameTree;

    #[derive(Debug)]
    pub struct Branch {
        pub m: MoveType,
        pub val: i32,
        pub leafs: usize
    }

    pub struct PositionInfo {
        pub tr: TurnResult,
        pub val: Option<i32>
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

            let tr = game.get_all_moves();

            if depth == 0 {
                    return game.evaluate_board(&tr);
            }

            if is_white_turn == maximizing_player {
                let mut max_eval = i32::MIN;

                for m in tr.white_moves.iter() {
                    if Engine::should_skip_move(*m, is_white_turn) {
                        continue;
                    }

                    let mut new_game = game.clone();
                    new_game.make_move(&m);

                    let mut new_node: GameTree = GameTree::new(Some(*m));
                    let eval = Engine::build_tree(&mut new_game, &mut new_node, !maximizing_player, depth - 1);

                    let board_eval = new_game.evaluate_board(&tr);

                    new_node.set_board_evals(board_eval, eval);
                    parent.add(new_node);

                    max_eval = i32::max(max_eval, eval);
                }

                return max_eval
            }
            else {
                let mut min_eval = i32::MAX;

                for m in tr.black_moves.iter() {
                    if Engine::should_skip_move(*m, is_white_turn) {
                        continue;
                    }

                    let mut new_game = game.clone();
                    new_game.make_move(&m);

                    let mut new_node: GameTree = GameTree::new(Some(*m));
                    let eval = Engine::build_tree(&mut new_game, &mut new_node, !maximizing_player, depth - 1);

                    let board_eval = new_game.evaluate_board(&tr);

                    new_node.set_board_evals(board_eval, eval);
                    parent.add(new_node);

                    min_eval = i32::min(min_eval, eval);
                }

                return min_eval
            }
        }

        pub fn get_sorted_moves(game: &mut Game, map: &mut HashMap<u64, PositionInfo>, is_maximizing: bool, depth: usize, with_sorting: bool) -> (Vec<Branch>, usize) {
            let mut sorted_moves: Vec<Branch> = vec![];

            let mut alpha = i32::MIN;
            let mut beta = i32::MAX;

            let mut all_leafs = 0;

            let tr = game.get_all_moves();

            let mut stopped: bool = false;

            //all_moves.sort_by(Engine::middle_game_sort());

            if is_maximizing {
                for mv in tr.white_moves.iter() {
                    if Engine::should_skip_move(*mv, game.is_white_turn) {
                        continue
                    }

                    if !stopped {
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
                            stopped = true;
                        }
                    } else {
                        sorted_moves.push(Branch {
                            m: *mv,
                            val: i32::MIN,
                            leafs: 0
                        });
                    }
                }
            } else {
                for mv in tr.black_moves.iter() {
                    if Engine::should_skip_move(*mv, game.is_white_turn) {
                        continue
                    }
                    if !stopped {
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
                            stopped = true;
                        }
                    } else {
                        sorted_moves.push(Branch {
                            m: *mv,
                            val: i32::MIN,
                            leafs: 0
                        });
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
            map: &mut HashMap<u64, PositionInfo>,
            is_maximizing: bool,
            mut alpha: i32,
            mut beta: i32,
            depth: usize,
            with_sorting: bool
        ) -> (i32, usize) {

            let is_white_turn = game.is_white_turn;

            let mut tr: TurnResult;
            let board_hash = game.board.compute_hash();
            //println!("board_hash: {}", board_hash);
            if board_hash == 15449896517834521811 {
                println!("board_hash: {}", board_hash);
                print_board(game)
            }
            match map.get_mut(&board_hash) {
                Some(pos) => {
                    tr = pos.tr.clone();
                }
                None => {
                    tr = game.get_all_moves();
                    println!("{}", mem::size_of::<PositionInfo>());
                    let pi = PositionInfo {
                        tr: tr.clone(),
                        val: None
                    };
                    map.insert(board_hash, pi);
                    ()
                }
            }



            if depth == 0 {
                match map.get_mut(&board_hash) {
                    Some(pos) => {
                        match pos.val {
                            Some(v) => return (v, 1),
                            None => {
                                if tr.gbi.is_check {
                                    if is_white_turn {
                                        if tr.white_moves.len() < 1 {
                                            return (i32::MIN, 1)
                                        } else {
                                            let mut new_game = *game;
                                            let (score, leave) = Engine::alpha_beta_from_internet(&new_game, map, !is_maximizing, alpha, beta, 0, with_sorting);
                                            return (score, leave)
                                        }
                                    } else {
                                        if tr.black_moves.len() < 1 {
                                            return (i32::MAX, 1)
                                        } else {
                                            let mut new_game = *game;
                                            let (score, leave) = Engine::alpha_beta_from_internet(&new_game, map, !is_maximizing, alpha, beta, 0, with_sorting);
                                            return (score, leave)
                                        }
                                    }
                                } else {
                                    let value = game.evaluate_board(&tr);
                                    pos.val = Some(value);
                                    return (value, 1);
                                }
                            }
                        }
                    },
                    None => {
                        let value = game.evaluate_board(&tr);
                        return (value, 1);
                        panic!("No map generated?????")
                    }
                }
            }

            if is_maximizing {
                let mut val = i32::MIN;
                let mut leaves = 0;

                if with_sorting {
                    tr.white_moves.sort_by(Engine::middle_game_sort());
                }

                for m in tr.white_moves.iter() {
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

                if with_sorting {
                    tr.black_moves.sort_by(Engine::middle_game_sort());
                }

                for m in tr.black_moves.iter() {
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
                                if pt.get_piece_type_value(true) < cp.get_piece_type_value(true) {
                                    return Ordering::Greater
                                }
                                Ordering::Less
                            }
                            MoveType::Capture(cpt, _, _, ccp, _) => {
                                if cpt.get_piece_type_value(true) <= ccp.get_piece_type_value(true)
                                && pt.get_piece_type_value(true) <= cp.get_piece_type_value(true) {
                                    if ccp.get_piece_type_value(true) - cpt.get_piece_type_value(true) <
                                        cp.get_piece_type_value(true) - pt.get_piece_type_value(true) {
                                        return Ordering::Less
                                    } else {
                                        return Ordering::Greater
                                    }
                                } else if cpt.get_piece_type_value(true) <= ccp.get_piece_type_value(true) {
                                    return Ordering::Less
                                }
                                else if pt.get_piece_type_value(true) <= cp.get_piece_type_value(true) {
                                    return Ordering::Greater
                                }

                                Ordering::Equal
                            }
                            MoveType::Attack(p, _, _, _, _) => {
                                if pt.get_piece_type_value(true) <= cp.get_piece_type_value(true) {
                                    return Ordering::Greater
                                }
                                Ordering::Less
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