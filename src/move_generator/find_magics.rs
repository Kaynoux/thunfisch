use crate::move_generator::blockers;
use crate::move_generator::rand_xor_shift::RandomXorShiftGenerator;
use crate::move_generator::shifts;
use crate::move_generator::sliding_targets;
use crate::prelude::*;

pub fn print_magics() {
    let mut offset = 0;
    let mut rng = RandomXorShiftGenerator::new();

    println!("pub const ROOK_MAGICS: [u64; 64] = [");
    let mut pos = IndexPosition(0);
    while pos.0 <= 63 {
        let magic = generate_magic(pos, &mut rng, true);
        print!("{},", magic);

        pos.0 += 1;
    }
    print!("];");
    println!();

    println!("pub const BISHOP_MAGICS: [u64; 64] = [");
    let mut pos1 = IndexPosition(0);
    while pos1.0 <= 63 {
        let magic = generate_magic(pos1, &mut rng, false);
        print!("{},", magic);

        pos1.0 += 1;
    }
    print!("];");
    println!();
}

pub fn generate_magic(pos: IndexPosition, rng: &mut RandomXorShiftGenerator, is_rook: bool) -> u64 {
    let (blockers, shift) = match is_rook {
        true => (blockers::ROOK_BLOCKERS[pos.0], shifts::ROOK_SHIFTS[pos.0]),
        false => (
            blockers::BISHOP_BLOCKERS[pos.0],
            shifts::BISHOP_SHIFTS[pos.0],
        ),
    };

    let size = 1usize << blockers.0.count_ones();
    let rook_target_boards = get_all_target_boards(pos, blockers, size, is_rook);

    for _ in 0..1_000_000 {
        // and cascade
        let magic = rng.next() & rng.next() & rng.next();
        // skip if they arent enough ones set because numbers is bad if so
        if (blockers.0.wrapping_mul(magic) & 0xFF00_0000_0000_0000).count_ones() < 6 {
            continue;
        }
        let result = check_magic_number(blockers, shift, &rook_target_boards, magic);
        if let Some(m) = result {
            return m;
        }
    }
    panic!("Magic not found");
}

pub fn check_magic_number(
    blocker: Bitboard,
    shift: u8,
    expected_rook_targets: &[Bitboard],
    magic: u64,
) -> Option<u64> {
    let mut hashed_targets = vec![Bitboard(0); expected_rook_targets.len()];
    let mut occ = Bitboard(0);
    for &expected in expected_rook_targets {
        let hash_idx = ((occ.0.wrapping_mul(magic)) >> shift) as usize;
        if hashed_targets[hash_idx] != Bitboard(0) && hashed_targets[hash_idx] != expected {
            return None;
        }
        hashed_targets[hash_idx] = expected;
        occ = Bitboard(occ.0.wrapping_sub(blocker.0) & blocker.0);
    }
    Some(magic)
}

pub fn get_all_target_boards(
    pos: IndexPosition,
    blocker: Bitboard,
    len: usize,
    is_rook: bool,
) -> Vec<Bitboard> {
    let mut boards = vec![Bitboard(0); len];
    let mut occ = Bitboard(0);
    for board in boards.iter_mut() {
        *board = match is_rook {
            true => sliding_targets::get_rook_positions(pos, occ),
            false => sliding_targets::get_bishop_positions(pos, occ),
        };
        // O(1) hack to go from 0b00 -> 0b01 -> 0b10 -> ob00 and all over again to infinity
        occ = Bitboard(occ.0.wrapping_sub(blocker.0) & blocker.0);
    }
    boards
}
