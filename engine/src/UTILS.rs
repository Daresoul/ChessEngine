pub mod utils {
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

}