pub mod game_board_info {
    #[derive(Clone)]
    pub struct GameBoardInfo {
        pub move_count: [usize; 12], // [P, B, N, R, Q, K, p, b, n, r, q, k]
        pub piece_count: [usize; 11],
        pub is_check: bool
    }

    impl GameBoardInfo {
        pub fn new() -> GameBoardInfo {
            GameBoardInfo {
                move_count: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                piece_count: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                is_check: false
            }
        }
    }
}