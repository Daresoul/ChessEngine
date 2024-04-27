use criterion::{black_box, criterion_group, criterion_main, Criterion};
use engine::game::game::Game;

pub const RANK_1: u64 = 0xff_u64;
pub const RANK_2: u64 = 0xff00_u64;
pub const RANK_3: u64 = 0xff0000_u64;
pub const RANK_4: u64 = 0xff000000_u64;
pub const RANK_5: u64 = 0xff00000000_u64;
pub const RANK_6: u64 = 0xff0000000000_u64;
pub const RANK_7: u64 = 0xff000000000000_u64;
pub const RANK_8: u64 = 0xff00000000000000_u64;


pub const FILE_H: u64 = 0x8080808080808080_u64;
pub const FILE_G: u64 = 0x4040404040404040_u64;
pub const FILE_F: u64 = 0x2020202020202020_u64;
pub const FILE_E: u64 = 0x1010101010101010_u64;
pub const FILE_D: u64 = 0x808080808080808_u64;
pub const FILE_C: u64 = 0x404040404040404_u64;
pub const FILE_B: u64 = 0x202020202020202_u64;
pub const FILE_A: u64 = 0x101010101010101_u64;

pub fn get_rank_mask(pos: usize) -> usize {
    let position: u64 = 1 << pos;
    if position == RANK_1 {
        return 0
    }

    if position == RANK_2 {
        return 1
    }

    if position == RANK_3 {
        return 2
    }

    if position == RANK_4 {
        return 3
    }

    if position == RANK_5 {
        return 4
    }

    if position == RANK_6 {
        return 5
    }

    if position == RANK_7 {
        return 6
    }

    if position == RANK_8 {
        return 7
    }

    return 0
}

pub fn get_file_mask(pos: usize) -> usize {
    let position: u64 = 1 << pos;
    if position & FILE_A > 0 {
        return 0
    }

    if position & FILE_B > 0 {
        return 1
    }

    if position & FILE_C > 0 {
        return 2
    }

    if position & FILE_D > 0 {
        return 3
    }

    if position & FILE_E > 0 {
        return 4
    }

    if position & FILE_F > 0 {
        return 5
    }

    if position & FILE_G > 0 {
        return 6
    }

    if position & FILE_H > 0 {
        return 7
    }

    return 0;
}

pub fn get_all_mask() -> (usize, usize) {
    let mut y= (0,0);

    for _ in 0..1000 {
        for x in 0..64 {
            y = (get_file_mask(x), get_rank_mask(x))
        }
    }
    return y
}

pub fn get_all_traditional()  -> (usize, usize) {
    let mut y= (0,0);
    for _ in 0..1000 {
        for x in 0..64 {
            y = (get_file_traditional(x), get_rank_traditional(x))
        }
    }
    return y
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Calculate with mask", |b| b.iter(|| black_box(get_all_mask())));
    c.bench_function("Calculate with traditional method", |b| b.iter(|| black_box(get_all_traditional())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);