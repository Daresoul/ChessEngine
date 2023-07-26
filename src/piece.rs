pub mod piece {

    struct PieceArrayIterator {
        array: [Option<Piece>; 64],
        index: usize,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum PieceType {
        Pawn, // 001
        Rook, // 010
        Knight, // 011
        Bishop, // 100
        Queen, // 101
        King // 110
    }



    #[derive(Debug, Clone, Copy)]
    pub struct Piece {
        pub piece_type: PieceType,
        pub is_white: bool
    }



    impl FromIterator<Option<Piece>> for PieceArrayIterator {
        fn from_iter<I: IntoIterator<Item = Option<Piece>>>(iter: I) -> Self {
            let iter = iter.into_iter();
            let array = {
                let mut array = [None; 64];
                for (i, item) in iter.enumerate().take(64) {
                    array[i] = item;
                }
                array
            };
            PieceArrayIterator {
                array,
                index: 0,
            }
        }
    }

    impl Iterator for PieceArrayIterator {
        type Item = Option<Piece>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < 64 {
                let value = self.array[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }
}
