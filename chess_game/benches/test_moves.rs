use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chess_game::debug_structs::debug_structs;
use chess_game::debug_structs::debug_structs::{generate_games, get_normal_board};
use chess_game::game::game::Game;

fn criterion_benchmark(c: &mut Criterion) {
    let game = Game::new_from_arr(debug_structs::get_normal_board(), true);
    let game2 = Game::new_from_arr(debug_structs::get_random_board(), true);
    let game3 = Game::new_from_arr(debug_structs::get_random_board2(), true);
    c.bench_function("calculate_moves_normal_board_white", |b| b.iter(|| black_box(game.get_all_moves())));
    c.bench_function("calculate_moves_heavy_pawn_white", |b| b.iter(|| black_box(game2.get_all_moves())));
    c.bench_function("calculate_moves_more_diverse_white", |b| b.iter(|| black_box(game3.get_all_moves())));

    let game = Game::new_from_arr(debug_structs::get_normal_board(), false);
    let game2 = Game::new_from_arr(debug_structs::get_random_board(), false);
    let game3 = Game::new_from_arr(debug_structs::get_random_board2(), false);
    c.bench_function("calculate_moves_normal_board_black", |b| b.iter(|| black_box(game.get_all_moves())));
    c.bench_function("calculate_moves_heavy_pawn_black", |b| b.iter(|| black_box(game2.get_all_moves())));
    c.bench_function("calculate_moves_more_diverse_black", |b| b.iter(|| black_box(game3.get_all_moves())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);