use thunfisch::{
    move_generator::masks::{self, king_safety_mask},
    prelude::*,
    settings,
};

use crate::tunable_params::TunableParams;

// constants for MG/EG interpolation - don't tune these
pub const KNIGHT: i32 = 1;
pub const BISHOP: i32 = 1;
pub const ROOK: i32 = 2;
pub const QUEEN: i32 = 4;
pub const TOTAL: i32 = KNIGHT * 4 + BISHOP * 4 + ROOK * 4 + QUEEN * 2;

// how impactful to the game phase a figure is if it's still on the board
// for example: A game is 'more' endgame if there are no more queens on the board
pub const GAMEPHASE_INC: [i32; 12] = [
    0, 0, KNIGHT, KNIGHT, BISHOP, BISHOP, ROOK, ROOK, QUEEN, QUEEN, 0, 0,
];


// Flips square index to flip rows but keep columns the same
// e.g. a1 becomes a8; e4 -> e5
const fn flip(sq: usize) -> usize {
    sq ^ 0x38
}

/// Construct one evaluation table individual for each piece.
/// The evaluation table assigns a value to each square on the board.
/// Each piece has a raw material value `base_piece_value`, which is constant across the entire board.
/// The position table is variable across the board and will additively alter the value of a piece depending on where it is located.
const fn init_table(
    base_piece_value: &[i32; 6],
    base_position_table: &[[i32; 64]; 6],
) -> [[i32; 64]; 12] {
    let mut table = [[0i32; 64]; 12];
    let mut piece = 0;
    while piece < 6 {
        let piece_type = piece * 2;
        let mut square = 0;
        while square < 64 {
            let base = base_piece_value[piece];
            // PSTs are from black's POV (i.e. [0][0] corresponds to a8 instead of a0)
            // so flip the board for white
            let offset_white = base_position_table[piece][flip(square)];
            let offset_black = base_position_table[piece][square];
            table[piece_type][square] = base + offset_white;
            table[piece_type + 1][square] = base + offset_black;
            square += 1;
        }
        piece += 1;
    }
    table
}

/// Evaluates the board
/// Uses Piece-Square Tables a base, and augments values of individual pieces as fitting.
/// Concepts that aren't specific to a certain piece (e.g. doubled pawns) are evaluated seperately and added to the augmented PSQT score.
/// Throughout the function, scores are to be interpreted as follows:
/// - positive -> advantage for white,
/// - negative -> advantage for black,
///
/// !! Return type is relativized for the current player for negamax search
/// Unit = Centipawns, 100 Centipawns => 1 Pawn
pub fn evaluate(board: &Board, params: &TunableParams) -> i32 {
    let white = 0usize;
    let black = 1usize;
    let mut mg = [0i32; 2];
    let mut eg = [0i32; 2];

    // cache the movement bitboards so this information can be used for both king safety and mobility
    let mut figure_movements = [Bitboard::EMPTY; 12];

    let open_files = board.open_files();

    let mg_table = init_table(&params.mg_piece_values, &params.mg_base_position_table);
    let eg_table = init_table(&params.eg_piece_values, &params.eg_base_position_table);

    let mut phase = TOTAL;
    for i in 0..=11 {
        let mut bb = board.figure_bb_by_index(i);

        // mobility - only needs to be done once per figure type
            let figure_mobility = board.calculate_piece_mobility(i, &mut figure_movements);
                mg[i & 1] += params.mobility_coefficients[0][i >> 1] * figure_mobility;
                eg[i & 1] += params.mobility_coefficients[1][i >> 1] * figure_mobility;
            // for correctness, mobility calculation removes all pieces of the same color (we can't take our own piece)
            // however for safety calculation we would like to also count a piece as "reaching the king zone" if it is physically in the zone
            // So we re-add only the figure to the movement mask
            figure_movements[i] |= bb;

        for bit in bb.iter_mut() {
            if open_files.is_position_set(bit) {
                // rooks on open files
                    mg[i & 1] += params.rook_open_file_bonus[0];
                    eg[i & 1] += params.rook_open_file_bonus[1];
                    mg[i & 1] += params.king_open_file_penalty[0];
                    eg[i & 1] += params.king_open_file_penalty[1];
            }

            let square = bit.to_square();
            mg[i & 1] += mg_table[i][square];
            eg[i & 1] += eg_table[i][square];

            phase -= GAMEPHASE_INC[i];
        }
    }
    // value is larger the less pieces are on the board
    // value is bound by the interval [0, 256]
    // (0 = starting position, 256 = only pawns and kings)
    let gamephase = (phase * 256 + (TOTAL / 2)) / TOTAL;

    let mut mg_score = mg[white] - mg[black];
    let mut eg_score = eg[white] - eg[black];

    let (mg_pawn_structure, eg_pawn_structure) = board.pawn_structure();
    mg_score += i32::from(mg_pawn_structure[white] - mg_pawn_structure[black]);
    eg_score += i32::from(eg_pawn_structure[white] - eg_pawn_structure[black]);

        let (mg_bishop_pair, eg_bishop_pair) = bishop_pair_boni(board, params);
        mg_score += i32::from(mg_bishop_pair[white] - mg_bishop_pair[black]);
        eg_score += i32::from(eg_bishop_pair[white] - eg_bishop_pair[black]);

        let (mg_king_safety, eg_king_safety) = king_safety(board, &figure_movements, params);
        // println!("mg king safety w/b: {mg_king_safety:?}");
        mg_score -= i32::from(mg_king_safety[white] - mg_king_safety[black]);
        eg_score -= i32::from(eg_king_safety[white] - eg_king_safety[black]);

    let current_color_multiplier = match board.current_color() {
        White => 1,
        Black => -1,
    };

        mg_score += current_color_multiplier * params.initiative;
        eg_score += current_color_multiplier * params.initiative;

    // Final aggregation of scoring aspects
    let mut score = (mg_score * (256 - gamephase) + eg_score * gamephase) >> 8;

        let doubled_pawns = doubled_pawn_penalties(board, params);
        score += doubled_pawns[white] - doubled_pawns[black];

    score * current_color_multiplier
}

/// Format:
/// (mg: [white, black], eg: [white, black])
pub fn pawn_structure(board: &Board, params: &TunableParams) -> ([i16; 2], [i16; 2]) {
    // [white, black]
    let mut mg_pawn_offset = [0i16; 2];
    let mut eg_pawn_offset = [0i16; 2];

    for i in 0..=1 {
        let color = Color::from_usize(i);
        for pawn in board.figure_bb_by_index(i).iter_mut() {
                let bonus = passed_pawn_bonus(board, pawn, color, params);
                mg_pawn_offset[i] += bonus[0];
                eg_pawn_offset[i] += bonus[1];
                let penalty = isolated_pawn_penalty(board, pawn, color, params);
                mg_pawn_offset[i] += penalty[0];
                eg_pawn_offset[i] += penalty[1];
        }
    }

    (mg_pawn_offset, eg_pawn_offset)
}

/// Format: `[MG, EG]`
/// Note: Penalties are negative, i.e. should be added
#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
fn isolated_pawn_penalty(
    board: &Board,
    pawn: Bit,
    friendly: Color,
    params: &TunableParams,
) -> [i16; 2] {
    let friendly_pawns = board.figure_bb(friendly, Piece::Pawn);
    let x = pawn.to_x() as i16;
    // we only look at the neighbouring files
    // double isolated pawns are still isolated, in fact an even worse liability than a singular isolated pawn
    let scan_mask = Bitboard::file((x - 1).max(0)) | Bitboard::file((x + 1).min(7));
    let is_isolated = i16::from((scan_mask & friendly_pawns).is_empty());

    [
        params.isolated_pawn_penalty[0] * is_isolated,
        params.isolated_pawn_penalty[1] * is_isolated,
    ]
}

/// Returns the passed pawn bonusses for `pawn`.
/// Format: `[MG, EG]`.
/// If the pawn is not passed, returns `[0, 0]`
fn passed_pawn_bonus(
    board: &Board,
    pawn: Bit,
    friendly: Color,
    params: &TunableParams,
) -> [i16; 2] {
    let opponent_pawns = board.figure_bb(!friendly, Piece::Pawn);
    let scan_mask = Bitboard::passed_pawn_mask(pawn, friendly);
    let is_passed = i16::from((scan_mask & opponent_pawns).is_empty());

    let idx: usize = match friendly {
        White => flip(pawn.to_square().0),
        Black => pawn.to_square().0,
    };

    [
        is_passed * params.mg_passed_pawn_table[idx],
        is_passed * params.eg_passed_pawn_table[idx],
    ]
}

/// Calculate the penalties for doubled pawns for both white and black
/// returns: 2-element array: `[white_penalty, black_penalty]`
///
/// Note: penalties are negative for both sides
/// TODO: this can probably be improved by weighing it against the remaining pawns (the less pawns are on the board, the worse it is if they're doubled)
#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn doubled_pawn_penalties(board: &Board, params: &TunableParams) -> [i32; 2] {
    let white_pawns = board.figure_bb_by_index(0);
    let black_pawns = board.figure_bb_by_index(1);
    // [white, black]
    let mut penalties: [i32; 2] = [0i32; 2];
    for i in 0..=7 {
        let file = Bitboard::file(i);
        let file_pawns = [(file & white_pawns).0, (file & black_pawns).0];
        // x & (x - 1) flips the lowest set bit -> essentially "removing" one pawn from the file
        // we explicitly allow overflows to deal with the case where there's 0 pawns
        let file_pawns = [
            (file_pawns[0] & file_pawns[0].wrapping_sub(1)).count_ones() as i32,
            (file_pawns[1] & file_pawns[1].wrapping_sub(1)).count_ones() as i32,
        ];

        penalties[0] += file_pawns[0] * params.doubled_pawn_penalty;
        penalties[1] += file_pawns[1] * params.doubled_pawn_penalty;
    }

    penalties
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn calculate_piece_mobility(
    board: &Board,
    figure: usize,
    figure_movements: &mut [Bitboard; 12],
) -> i32 {
    let attackmask = masks::calculate_attackmask_by_figure(board, board.occupied(), figure, None);
    figure_movements[figure] = attackmask;
    // here, don't count taking your own piece as a movable square
    // note that this distinction is made after we persisted the attack masks for the king safety calculations
    (attackmask & !board.color_bbs(Color::from_usize(figure & 1))).get_count() as i32
}

/// return Format:
/// (mg: [white, black], eg: [white, black])
#[inline]
#[allow(clippy::cast_possible_truncation)]
fn bishop_pair_boni(board: &Board, params: &TunableParams) -> ([i16; 2], [i16; 2]) {
    let white_bishops = board.figure_bb(Color::White, Piece::Bishop).get_count();
    let black_bishops = board.figure_bb(Color::Black, Piece::Bishop).get_count();

    (
        [
            params.bishop_pair_bonus[0] * (white_bishops >> 1) as i16,
            params.bishop_pair_bonus[0] * (black_bishops >> 1) as i16,
        ],
        [
            params.bishop_pair_bonus[1] * (white_bishops >> 1) as i16,
            params.bishop_pair_bonus[1] * (black_bishops >> 1) as i16,
        ],
    )
}

/// Calculate a bitboard marking open files (files without any pawns on them)
pub fn open_files(board: &Board) -> Bitboard {
    let pawn_structure =
        board.figure_bb(Color::White, Piece::Pawn) | board.figure_bb(Color::Black, Piece::Pawn);
    let mut open_files = Bitboard::EMPTY;
    for i in 0..=7 {
        let file = Bitboard::file(i);
        if (file & pawn_structure).is_empty() {
            open_files += file;
        }
    }
    open_files
}

/// Calculates the King Safety Score
/// WARNING: technically computes danger; so the higher the value, the worse the position is.
/// return Format:
/// (mg: [white, black], eg: [white, black])
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::similar_names
)]
pub fn king_safety(
    board: &Board,
    figure_movements: &[Bitboard; 12],
    params: &TunableParams,
) -> ([i16; 2], [i16; 2]) {
    let mut mg_safety = [0i16; 2];
    let mut eg_safety = [0i16; 2];

    let king_zones = [
        king_safety_mask(board, Color::White),
        king_safety_mask(board, Color::Black),
    ];
    let pawn_shield_zones = [
        king_zones[0] & ((Bitboard(0xff) << (8 * board.king(Color::White).to_xy().1)) << 8),
        king_zones[1] & ((Bitboard(0xff) << (8 * board.king(Color::Black).to_xy().1)) >> 8),
    ];
    // pawn shields
    for i in 0..=1 {
        mg_safety[i] -= (pawn_shield_zones[i] & board.figure_bb_by_index(i)).get_count() as i16
            * params.pawn_shield_bonus[0];
        eg_safety[i] -= (pawn_shield_zones[i] & board.figure_bb_by_index(i)).get_count() as i16
            * params.pawn_shield_bonus[1];
    }

    // skip pawns and kings in the evaluation
    for i in 2..=9 {
        let friend = i & 1;
        let opp = friend ^ 1;
        let attacker_mg = params.piece_attack_values[0][i >> 1];
        let attacker_eg = params.piece_attack_values[1][i >> 1];
        let defender_mg = params.piece_defend_values[0][i >> 1];
        let defender_eg = params.piece_defend_values[1][i >> 1];

        // decrease friendly danger score for friendly piece in friendly king zone
        mg_safety[friend] -=
            (king_zones[friend] & figure_movements[i]).get_count() as i16 * defender_mg;
        eg_safety[friend] -=
            (king_zones[friend] & figure_movements[i]).get_count() as i16 * defender_eg;

        mg_safety[opp] += (king_zones[opp] & figure_movements[i]).get_count() as i16 * attacker_mg;
        eg_safety[opp] += (king_zones[opp] & figure_movements[i]).get_count() as i16 * attacker_eg;
    }

    #[allow(clippy::cast_sign_loss)]
    (
        [
            params.mg_king_safety_table[mg_safety[0].clamp(0, 99) as usize],
            params.mg_king_safety_table[mg_safety[1].clamp(0, 99) as usize],
        ],
        [
            params.eg_king_safety_table[eg_safety[0].clamp(0, 99) as usize],
            params.eg_king_safety_table[eg_safety[1].clamp(0, 99) as usize],
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_eval_works() {
        let fen = "2B5/kpp2rqr/pbbp4/8/8/B7/RP3PPP/QRNn2K1 w - - 0 1";
        let board = Board::new(fen);
        let thunfisch_eval = board.evaluate();
        let params = TunableParams::default();
        let tuner_eval = evaluate(&board, &params);

        assert_eq!(thunfisch_eval, tuner_eval)
    }
    #[test]
    fn print_built_psqts() {
        let params = TunableParams::default();

        let start = std::time::Instant::now();
        let mg_table = init_table(&params.mg_piece_values, &params.mg_base_position_table);
        let mg_duration = start.elapsed();

        let start = std::time::Instant::now();
        let eg_table = init_table(&params.eg_piece_values, &params.eg_base_position_table);
        let eg_duration = start.elapsed();

        println!("mg ({mg_duration:?}):");
        // for table in mg_table {
        //     println!("{table:?}");
        // }
        println!("eg ({eg_duration:?}):");
        // for table in eg_table {
        //     println!("{table:?}");
        // }
   }
}
