use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chess_game::board::board::Board;
use chess_game::debug_structs::debug_structs::get_normal_board;

fn criterion_benchmark(c: &mut Criterion) {
    let board = Board::new_from_arr(get_normal_board());
    c.bench_function("hash_boards", |b| b.iter(|| black_box(board.compute_hash())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);