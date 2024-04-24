use crate::Magics::magics::Magics;
use crate::Generate::pieces::Direction::{East, North, South, West};
use crate::Generate::pieces::{Direction, MoveGen};


impl MoveGen {

    const ROOK_MAGIC_NUMBERS: [u64; 64] = [
        324259448050975248u64, 162139001189302336u64, 4647750006529359880u64, 144121785691422736u64,
        16176938657641660544u64, 9367489423970945072u64, 36051338366288384u64, 36029147746665088u64,
        3518447965192208u64, 4614078830617822340u64, 9241949523864129664u64, 11540615780106252u64,
        730287067600519297u64, 144819425575437312u64, 1225261127674627584u64, 40814017656160512u64,
        594475700577118276u64, 283675082228259u64, 148058037853261952u64, 14411662294658320384u64,
        2394186703782912u64, 1157847866488718336u64, 2306407062973841412u64, 4576167411597460u64,
        2323857959626489888u64, 18860477004136448u64, 621497027752297522u64, 3027553647748714496u64,
        9241953785514295424u64, 1970363492082688u64, 1729664285938024960u64, 4836870457972064321u64,
        141012374650913u64, 4652253601601699840u64, 58687601506263040u64, 281543780081672u64,
        1157433900411130112u64, 81628378934806544u64, 2310366730829959192u64, 2900476768907429780u64,
        36558770110480u64, 9042384969023488u64, 180425597514743824u64, 5487636764434923528u64,
        5766860422494879764u64, 9224498487624761348u64, 41702298761822218u64, 45599234000551940u64,
        70370891935872u64, 19210671497487104u64, 387030266675328u64, 289215847808893056u64,
        576469550545240192u64, 1153216449143113729u64, 9350715278336u64, 288521763922764288u64,
        282782794268833u64, 595672521157161122u64, 436884352794689609u64, 9241667927690743809u64,
        5188428314494240769u64, 1157988067282792450u64, 1152939243166828548u64, 4611967569673330817u64,
    ];

    pub fn init_magics(&mut self) {
        let mut offset = 0;

        for sq in 0..64_usize {
            let mask = self.rook_masks[sq];

            let bits = mask.count_ones(); // Number of set bits in the mask
            let permutations = 2u64.pow(bits); // Number of blocker boards to be indexed.
            let end = offset + permutations - 1; // End point in the attack table.
            let blocker_boards = Self::blocker_boards(mask);

            let attack_boards = Self::rook_attack_boards(sq, &blocker_boards);

            let mut magic: Magics = Default::default();
            let r_magic_nr = Self::ROOK_MAGIC_NUMBERS[sq];

            magic.mask = mask;
            magic.shift = 64 - bits;
            magic.offset = offset;
            magic.magic_number = r_magic_nr;

            for i in 0..permutations {
                let next = i as usize;
                let index = magic.get_index(blocker_boards[next]);

                if self.rook_table[index] == 0 {
                    let fail_low = index < offset as usize;
                    let fail_high = index > end as usize;
                    assert!(!fail_low && !fail_high, "Indexing error. Error in Magics.");
                    self.rook_table[index] = attack_boards[next];
                } else {
                    panic!("Attack table index not empty. Error in Magics.");
                }
            }

            // No failures  during indexing. Store this magic.
            self.rook_magics[sq as usize] = magic;

            // Do the next magic.
            offset += permutations;
        }

        // All permutations (blocker boards) should have been indexed.
        let expectation = 102_400;
        const ERROR: &str = "Initializing magics failed. Check magic numbers.";

        assert!(offset == expectation, "{}", ERROR);
    }

    pub fn magician(&self, position: usize, occupancy: u64) -> u64 {
        let index = self.rook_magics[position].get_index(occupancy);
        self.rook_table[index]
    }

    pub fn rook_attack_boards(square: usize, blockers: &[u64]) -> Vec<u64>{
        let mut bb_attack_boards: Vec<u64> = Vec::new();

        for b in blockers.iter() {
            let bb_attacks = Self::bb_ray(*b, square, North)
                | Self::bb_ray(*b, square, East)
                | Self::bb_ray(*b, square, South)
                | Self::bb_ray(*b, square, West);
            bb_attack_boards.push(bb_attacks);
        }

        bb_attack_boards
    }


    pub fn blocker_boards(mask: u64) -> Vec<u64> {
        let d: u64 = mask;
        let mut bb_blocker_boards: Vec<u64> = Vec::new();
        let mut n: u64 = 0;

        // Carry-Rippler
        // https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
        loop {
            bb_blocker_boards.push(n);
            n = n.wrapping_sub(d) & d;
            if n == 0 {
                break;
            }
        }

        bb_blocker_boards
    }
}