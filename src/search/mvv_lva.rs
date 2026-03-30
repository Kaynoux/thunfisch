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

/// https://www.chessprogramming.org/MVV-LVA
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

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    #[test]
    fn print_mvva_lva_table() {
        let mut value_dist: HashMap<i32, usize> = HashMap::new();
        for i in 0..6 {
            for ii in 0..6 {
                let mvv_lva_val = MVV_LVA_TABLE[i][ii];
                *value_dist.entry(mvv_lva_val).or_insert(0) += 1;
            }
        }
        let mut value_dist = value_dist.iter().collect::<Vec<(&i32, &usize)>>();
        value_dist.sort_by_key(|&(key, _)| key);
        value_dist.sort_by_key(|&(_, val)| val);
        println!("{:?}", value_dist.iter());

        // This is the output:
        // [(10000, 1), (19100, 1), (19500, 1), (19680, 1), (19700, 1), (19900, 1),
        //  (30000, 1), (32000, 1), (39100, 1), (39500, 1), (39680, 1), (39700, 1),
        //  (39900, 1), (41100, 1), (41500, 1), (41680, 1), (41700, 1), (41900, 1),
        //  (50000, 1), (59100, 1), (59500, 1), (59680, 1), (59700, 1), (59900, 1),
        //  (90000, 1), (99100, 1), (99500, 1), (99680, 1), (99700, 1), (99900, 1),
        //  (0, 6)]
    }
}
