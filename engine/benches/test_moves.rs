use criterion::{black_box, criterion_group, criterion_main, Criterion};
use engine::game::game::Game;

fn criterion_benchmark(c: &mut Criterion) {
    let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);
    let mut game2 = Game::new_from_string("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8".to_string(), true);
    let mut game3 = Game::new_from_string("r1bk3r/p2pBpNp/k4k2/1p1KP2P/6P1/3P4/P1P1K3/q5b1".to_string(), true);
    c.bench_function("calculate_moves_normal_board_white", |b| b.iter(|| black_box(game.get_all_moves())));
    c.bench_function("calculate_moves_heavy_pawn_white", |b| b.iter(|| black_box(game2.get_all_moves())));
    c.bench_function("calculate_moves_more_diverse_white", |b| b.iter(|| black_box(game3.get_all_moves())));

    let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), false);
    let mut game2 = Game::new_from_string("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8".to_string(), false);
    let mut game3 = Game::new_from_string("r1bk3r/p2pBpNp/k4k2/1p1KP2P/6P1/3P4/P1P1K3/q5b1".to_string(), false);
    c.bench_function("calculate_moves_normal_board_black", |b| b.iter(|| black_box(game.get_all_moves())));
    c.bench_function("calculate_moves_heavy_pawn_black", |b| b.iter(|| black_box(game2.get_all_moves())));
    c.bench_function("calculate_moves_more_diverse_black", |b| b.iter(|| black_box(game3.get_all_moves())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);