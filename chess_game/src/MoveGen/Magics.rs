pub mod magics {
    #[derive(Default, Clone, Copy)]
    pub struct Magics {
        pub magic_number: u64,
        pub offset: u64,
        pub shift: u32,
        pub mask: u64
    }

    impl Magics {
        pub fn default(magic_number: u64, offset: u64, shift: u32, mask: u64) -> Magics {
            Magics {
                magic_number,
                offset,
                shift,
                mask
            }
        }

        pub fn get_index(&self, occupancy: u64) -> usize {
            let blockerboard = occupancy & self.mask;
            ((blockerboard.wrapping_mul(self.magic_number) >> self.shift) + self.offset) as usize
        }
    }
}