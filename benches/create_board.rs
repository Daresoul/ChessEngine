use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chess_game::game::game::Game;
use chess_game::debug_structs::debug_structs::get_normal_board;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create_board", |b| b.iter(|| Game::new_from_arr(black_box(get_normal_board()), true)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);