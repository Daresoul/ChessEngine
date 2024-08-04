pub mod utils {
    pub const RANK_8: u64 = 0xff_u64;
    pub const RANK_7: u64 = 0xff00_u64;
    pub const RANK_6: u64 = 0xff0000_u64;
    pub const RANK_5: u64 = 0xff000000_u64;
    pub const RANK_4: u64 = 0xff00000000_u64;
    pub const RANK_3: u64 = 0xff0000000000_u64;
    pub const RANK_2: u64 = 0xff000000000000_u64;
    pub const RANK_1: u64 = 0xff00000000000000_u64;


    pub const FILE_H: u64 = 0x8080808080808080_u64;
    pub const FILE_G: u64 = 0x4040404040404040_u64;
    pub const FILE_F: u64 = 0x2020202020202020_u64;
    pub const FILE_E: u64 = 0x1010101010101010_u64;
    pub const FILE_D: u64 = 0x808080808080808_u64;
    pub const FILE_C: u64 = 0x404040404040404_u64;
    pub const FILE_B: u64 = 0x202020202020202_u64;
    pub const FILE_A: u64 = 0x101010101010101_u64;
    
    pub const POSITIONS: [u64; 64] = generate_shifts();
    pub const NEGATIVE_POSITIONS: [u64; 64] = generate_negative_shifts();


    const fn generate_shifts() -> [u64; 64] {
        let mut shifts = [0; 64];
        let mut i = 0;
        while i < 64 {
            shifts[i] = 1 << i;
            i += 1;
        }
        return shifts
    }

    const fn generate_negative_shifts() -> [u64; 64] {
        let mut shifts = [0; 64];
        let mut i = 0;
        while i < 64 {
            shifts[i] = !(1 << i);
            i += 1;
        }
        return shifts
    }

    pub fn pop_lsb(mask: &mut u64) -> usize {
        let bit_pos = mask.trailing_zeros();
        *mask &= *mask - 1;
        bit_pos as usize
    }

    pub fn bitscan_forward(board: u64) -> Option<usize> {
        if board == 0 {
            return None
        } else {
            return Some(board.trailing_zeros() as usize)
        }
    }

    pub fn bitscan_reverse(board: u64) -> usize {
        return 63 - board.leading_zeros() as usize
    }

    pub fn count_ones_i32(board: u64) -> i32 {
        board.count_ones() as i32
    }

    pub fn get_rank(pos: usize) -> usize {
        debug_assert!(pos < 64, "Pos has to be less than 64, you tried with {pos}!");

        7 - (pos / 8)
    }

    pub fn get_file(pos: usize) -> usize {
        pos % 8
    }

    pub fn get_file_and_rank(pos: usize) -> (usize, usize) {
        (get_file(pos), get_rank(pos))
    }

}