pub(crate) mod game_tree {
    use std::fmt::{Debug, Formatter};
    use std::fs::OpenOptions;
    use chess_game::board::board::MoveType;

    #[derive(Clone)]
    pub(crate) struct GameTree {
        pub board_eval: i32,
        pub move_type: Option<MoveType>,
        pub alpha: i32,
        pub beta: i32,
        pub best: i32,
        pub(crate) children: Vec<GameTree>
    }

    impl Debug for GameTree {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "(move: {:?}, board: {:?}, alpha: {:?}, beta: {:?})", self.move_type, self.board_eval, self.alpha, self.beta)
        }
    }

    impl GameTree {
        pub fn new(move_type: Option<MoveType>) -> GameTree {
            GameTree {
                move_type: move_type,
                board_eval: 0,
                best: 0,
                alpha: i32::MIN,
                beta: i32::MAX,
                children: Vec::new()
            }
        }
        pub fn add(&mut self, value: GameTree) {
            self.children.push(value);
        }

        pub fn children(&self) -> &Vec<GameTree> {
            &self.children
        }

        pub fn set_alpha_beta(&mut self, alpha: i32, beta: i32) -> () {
            self.alpha = alpha;
            self.beta = beta;
        }

        pub fn set_board_evals(&mut self, board_eval: i32, best: i32) -> () {
            self.board_eval = board_eval;
            self.best = best;
        }

        pub fn print_pretty(tree: &GameTree, indent: usize) {
            for _ in 0..indent {
                print!(" ");
            }
            println!("{:?}", tree);

            for child in tree.children() {
                GameTree::print_pretty(child, indent + 4);
            }
        }
    }
}