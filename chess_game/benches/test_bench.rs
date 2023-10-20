use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use chess_game::board::board::Board;
use chess_game::debug_structs::debug_structs::get_normal_board;

fn tester(mut arr: [i32; 1000000]) -> i32 {
    let mut sum = 0;
    for i in 0..10 {
        for j in 0..1000000 {
            arr[j] = arr[j] * arr[j];
        }
    }

    return sum
}

fn tester2(mut arr: [i32; 1000000]) -> i32 {
    let mut sum = 0;
    for j in 0..1000000 {
        arr[j] = arr[j] * arr[j];
    }
    for j in 0..1000000 {
        arr[j] = arr[j] * arr[j];
    }
    for j in 0..1000000 {
        arr[j] = arr[j] * arr[j];
    }
    for j in 0..1000000 {
        arr[j] = arr[j] * arr[j];
    }
    for j in 0..1000000 {
        arr[j] = arr[j] * arr[j];
    }
    for j in 0..1000000 {
        arr[j] = arr[j] * arr[j];
    }
    for j in 0..1000000 {
        arr[j] = arr[j] * arr[j];
    }
    for j in 0..1000000 {
        arr[j] = arr[j] * arr[j];
    }
    for j in 0..1000000 {
        arr[j] = arr[j] * arr[j];
    }
    for j in 0..1000000 {
        arr[j] = arr[j] * arr[j];
    }

    return sum
}

fn create_arr() -> [i32; 1000000] {
    let mut rng = rand::thread_rng();
    let rando: [i32; 1000000] = core::array::from_fn(|i| rng.gen());
    return rando;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test_compact", |b| b.iter(|| tester(create_arr())));
    c.bench_function("test_large", |b| b.iter(|| tester2(create_arr())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);