// These values or known to perform well
const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 300;
const BISHOP_VALUE: i32 = 320;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 10000;

const ORDERING_OFFSET: i32 = 10000;
const ORDERING_MULTIPLIER: i32 = 100;

const PIECE_VALUES: [i32; 6] = [
    PAWN_VALUE,
    KNIGHT_VALUE,
    BISHOP_VALUE,
    ROOK_VALUE,
    QUEEN_VALUE,
    KING_VALUE,
];

const fn calculate_mvv_lva_score(victim_idx: usize, attacker_idx: usize) -> i32 {
    // King cannot be captured
    if victim_idx >= 5 {
        return 0;
    }

    let victim_value = PIECE_VALUES[victim_idx];
    let attacker_value = PIECE_VALUES[attacker_idx];

    victim_value * ORDERING_MULTIPLIER - attacker_value + ORDERING_OFFSET
}

pub const MVV_LVA_TABLE: [[i32; 6]; 6] = {
    let mut table = [[0i32; 6]; 6];
    let mut attacker_idx = 0;
    while attacker_idx < 6 {
        let mut victim_idx = 0;
        // < 5 because king can be ignored
        while victim_idx < 5 {
            table[attacker_idx][victim_idx] = calculate_mvv_lva_score(victim_idx, attacker_idx);
            victim_idx += 1;
        }
        attacker_idx += 1;
    }
    table
};
