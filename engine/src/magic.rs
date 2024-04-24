pub mod magic {
    #[derive(Default, Clone, Copy)]
    pub struct Magic {
        pub magic_number: u64,
        pub offset: u64,
        pub shift: u32,
        pub mask: u64
    }

    impl Magic {
        pub fn default(magic_number: u64, offset: u64, shift: u32, mask: u64) -> Magic {
            Magic {
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