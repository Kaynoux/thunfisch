use crate::{
    move_generator::masks::{self, king_safety_mask},
    prelude::*,
    settings,
};

pub const MATE_SCORE: i32 = 30_000;

const KNIGHT: i32 = 1;
const BISHOP: i32 = 1;
const ROOK: i32 = 2;
const QUEEN: i32 = 4;
const TOTAL: i32 = KNIGHT * 4 + BISHOP * 4 + ROOK * 4 + QUEEN * 2;

// [pawn, knight, bishop, rook, queen, king]
const MG_PIECE_VALUES: [i32; 6] = [82, 337, 365, 477, 1025, 0];
const EG_PIECE_VALUES: [i32; 6] = [94, 281, 297, 512, 936, 0];
const MG_TABLE: [[i32; 64]; 12] = init_table(&MG_PIECE_VALUES, &MG_BASE_POSITION_TABLE);
const EG_TABLE: [[i32; 64]; 12] = init_table(&EG_PIECE_VALUES, &EG_BASE_POSITION_TABLE);
// how impactful to the game phase a figure is if it's still on the board
// for example: A game is 'more' endgame if there are no more queens on the board
pub const GAMEPHASE_INC: [i32; 12] = [
    0, 0, KNIGHT, KNIGHT, BISHOP, BISHOP, ROOK, ROOK, QUEEN, QUEEN, 0, 0,
];

// rooks on open files are a rather weak positional idea so this should be kept pretty low
// Additionally, Endgames typically have a lot of open files, so there's no benefit to occupying one (hence 0 EG score)
// format: [MG, EG]
const ROOK_OPEN_FILE_BONUS: [i32; 2] = [25, 0];
const KING_OPEN_FILE_PENALTY: [i32; 2] = [-25, 0];

// TODO: probably interpolation of these values between MG and EG makes sense
// a doubled pawn should be worth about half a pawn
// For now we just linearly scale this; may be worth tho looking at punishing tripled pawns harder than doubled pawns
const DOUBLED_PAWN_PENALTY: i32 = -10;

// A pawn is isolated if it has no pawns on the file next to it
// Generally isolated pawns are bad as they require pieces to defend and thus are easy targets
const ISOLATED_PAWN_PENALTY: [i16; 2] = [-23, -13];

// Some say having the bishop pair is a slight advantage because having only one bishop essentially makes half the board unreachable
// Personally I'm indifferent to the bishop pair but it's an easy implementation and may gain a little bit
const BISHOP_PAIR_BONUS: [i16; 2] = [15, 25];

pub const MOBILITY_COEFFICIENTS: [[i32; 6]; 2] = [[0, 5, 3, 2, 1, 0], [0, 5, 3, 4, 1, 0]];

pub const PIECE_ATTACK_VALUES: [[i16; 6]; 2] = [[0, 2, 2, 3, 5, 0], [0, 2, 2, 4, 6, 0]];
pub const PIECE_DEFEND_VALUES: [[i16; 6]; 2] = [[0, 1, 1, 2, 4, 0], [0, 1, 1, 3, 5, 0]];


// bonus for the side to move
pub const INITIATIVE: i32 = 15;

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

/// Values here are vaguely inspired in their Shape by Fatalii
/// However I've changed them to only give bonusses and never subtract from the eval
/// let's see how this does haha
#[rustfmt::skip]
pub const MG_PASSED_PAWN_TABLE: [i16; 64] = [
         0,    0,    0,    0,    0,    0,    0,    0,
         0,    0,    0,    0,    0,    0,    0,    0,
        15,   20,   20,   10,   10,   20,   20,   15,
        10,   15,   15,    5,    5,   15,   15,   10,
        10,   15,   15,    5,    5,   15,   15,   10,
        15,   20,   20,   10,   10,   20,   20,   15,
        15,   20,   20,   10,   10,   20,   20,   15,
         0,    0,    0,    0,    0,    0,    0,    0,
];

#[rustfmt::skip]
pub const EG_PASSED_PAWN_TABLE: [i16; 64] = [
         0,    0,    0,    0,    0,    0,    0,    0,
         0,    0,    0,    0,    0,    0,    0,    0,
        90,   85,   80,   70,   70,   80,   85,   90,
        65,   60,   55,   45,   45,   55,   60,   65,
        40,   35,   30,   20,   20,   30,   35,   40,
        30,   25,   20,   10,   10,   20,   25,   30,
        25,   20,   15,    5,    5,   15,   20,   25,
         0,    0,    0,    0,    0,    0,    0,    0,
];

pub const MG_KING_SAFETY_TABLE: [i16; 100] = [
    0, 0, 1, 2, 3, 5, 7, 9, 12, 15, 18, 22, 26, 30, 35, 39, 44, 50, 56, 62, 68, 75, 82, 85, 89, 97,
    105, 113, 122, 131, 140, 150, 169, 180, 191, 202, 213, 225, 237, 248, 260, 272, 283, 295, 307,
    319, 330, 342, 354, 366, 377, 389, 401, 412, 424, 436, 448, 459, 471, 483, 494, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
];
pub const EG_KING_SAFETY_TABLE: [i16; 100] = [
    0, 0, 0, 1, 2, 3, 4, 6, 8, 10, 12, 14, 17, 20, 23, 26, 29, 33, 37, 41, 45, 50, 54, 56, 59, 64,
    70, 75, 81, 87, 93, 100, 112, 120, 127, 134, 142, 150, 158, 165, 173, 181, 188, 196, 204, 212,
    220, 228, 236, 244, 251, 259, 267, 274, 282, 290, 298, 306, 314, 322, 329, 333, 333, 333, 333,
    333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333,
    333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333,
];

/// Middlegame-Piece-Square Tables Base Values used for calculation of the real eval values
pub const MG_BASE_POSITION_TABLE: [[i32; 64]; 6] = [
    // 0: Pawn
    [
        0, 0, 0, 0, 0, 0, 0, 0, 98, 134, 61, 95, 68, 126, 34, -11, -6, 7, 26, 31, 65, 56, 25, -20,
        -14, 13, 6, 21, 23, 12, 17, -23, -27, -2, -5, 12, 17, 6, 10, -25, -26, -4, -4, -10, 3, 3,
        33, -12, -35, -1, -20, -23, -15, 24, 38, -22, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    // 1: Knight
    [
        -167, -89, -34, -49, 61, -97, -15, -107, -73, -41, 72, 36, 23, 62, 7, -17, -47, 60, 37, 65,
        84, 129, 73, 44, -9, 17, 19, 53, 37, 69, 18, 22, -13, 4, 16, 13, 28, 19, 21, -8, -23, -9,
        12, 10, 19, 17, 25, -16, -29, -53, -12, -3, -1, 18, -14, -19, -105, -21, -58, -33, -17,
        -28, -19, -23,
    ],
    // 2: Bishop
    [
        -29, 4, -82, -37, -25, -42, 7, -8, -26, 16, -18, -13, 30, 59, 18, -47, -16, 37, 43, 40, 35,
        50, 37, -2, -4, 5, 19, 50, 37, 37, 7, -2, -6, 13, 13, 26, 34, 12, 10, 4, 0, 15, 15, 15, 14,
        27, 18, 10, 4, 15, 16, 0, 7, 21, 33, 1, -33, -3, -14, -21, -13, -12, -39, -21,
    ],
    // 3: Rook
    [
        32, 42, 32, 51, 63, 9, 31, 43, 27, 32, 58, 62, 80, 67, 26, 44, -5, 19, 26, 36, 17, 45, 61,
        16, -24, -11, 7, 26, 24, 35, -8, -20, -36, -26, -12, -1, 9, -7, 6, -23, -45, -25, -16, -17,
        3, 0, -5, -33, -44, -16, -20, -9, -1, 11, -6, -71, -19, -13, 1, 17, 16, 7, -37, -26,
    ],
    // 4: Queen
    [
        -28, 0, 29, 12, 59, 44, 43, 45, -24, -39, -5, 1, -16, 57, 28, 54, -13, -17, 7, 8, 29, 56,
        47, 57, -27, -27, -16, -16, -1, 17, -2, 1, -9, -26, -9, -10, -2, -4, 3, -3, -14, 2, -11,
        -2, -5, 2, 14, 5, -35, -8, 11, 2, 8, 15, -3, 1, -1, -18, -9, 10, -15, -25, -31, -50,
    ],
    // 5: King
    [
        -65, 23, 16, -15, -56, -34, 2, 13, 29, -1, -20, -7, -8, -4, -38, -29, -9, 24, 2, -16, -20,
        6, 22, -22, -17, -20, -12, -27, -30, -25, -14, -36, -49, -1, -27, -39, -46, -44, -33, -51,
        -14, -14, -22, -46, -44, -30, -15, -27, 1, 7, -8, -64, -43, -16, 9, 8, -15, 36, 12, -54, 8,
        -28, 24, 14,
    ],
];

/// Endgame-Piece-Square Tables Base Values used for calculation of the real eval values
pub const EG_BASE_POSITION_TABLE: [[i32; 64]; 6] = [
    // 0: Pawn
    [
        0, 0, 0, 0, 0, 0, 0, 0, 178, 173, 158, 134, 147, 132, 165, 187, 94, 100, 85, 67, 56, 53,
        82, 84, 32, 24, 13, 5, -2, 4, 17, 17, 13, 9, -3, -7, -7, -8, 3, -1, 4, 7, -6, 1, 0, -5, -1,
        -8, 13, 8, 8, 10, 13, 0, 2, -7, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    // 1: Knight
    [
        -58, -38, -13, -28, -31, -27, -63, -99, -25, -8, -25, -2, -9, -25, -24, -52, -24, -20, 10,
        9, -1, -9, -19, -41, -17, 3, 22, 22, 22, 11, 8, -18, -18, -6, 16, 25, 16, 17, 4, -18, -23,
        -3, -1, 15, 10, -3, -20, -22, -42, -20, -10, -5, -2, -20, -23, -44, -29, -51, -23, -15,
        -22, -18, -50, -64,
    ],
    // 2: Bishop
    [
        -14, -21, -11, -8, -7, -9, -17, -24, -8, -4, 7, -12, -3, -13, -4, -14, 2, -8, 0, -1, -2, 6,
        0, 4, -3, 9, 12, 9, 14, 10, 3, 2, -6, 3, 13, 19, 7, 10, -3, -9, -12, -3, 8, 10, 13, 3, -7,
        -15, -14, -18, -7, -1, 4, -9, -15, -27, -23, -9, -23, -5, -9, -16, -5, -17,
    ],
    // 3: Rook
    [
        13, 10, 18, 15, 12, 12, 8, 5, 11, 13, 13, 11, -3, 3, 8, 3, 7, 7, 7, 5, 4, -3, -5, -3, 4, 3,
        13, 1, 2, 1, -1, 2, 3, 5, 8, 4, -5, -6, -8, -11, -4, 0, -5, -1, -7, -12, -8, -16, -6, -6,
        0, 2, -9, -9, -11, -3, -9, 2, 3, -1, -5, -13, 4, -20,
    ],
    // 4: Queen
    [
        -9, 22, 22, 27, 27, 19, 10, 20, -17, 20, 32, 41, 58, 25, 30, 0, -20, 6, 9, 49, 47, 35, 19,
        9, 3, 22, 24, 45, 57, 40, 57, 36, -18, 28, 19, 47, 31, 34, 39, 23, -16, -27, 15, 6, 9, 17,
        10, 5, -22, -23, -30, -16, -16, -23, -36, -32, -33, -28, -22, -43, -5, -32, -20, -41,
    ],
    // 5: King
    [
        -74, -35, -18, -18, -11, 15, 4, -17, -12, 17, 14, 17, 17, 38, 23, 11, 10, 17, 23, 15, 20,
        45, 44, 13, -8, 22, 24, 27, 26, 33, 26, 3, -18, -4, 21, 24, 27, 23, 9, -11, -19, -3, 11,
        21, 23, 16, 7, -9, -27, -11, 4, 13, 14, 4, -5, -17, -53, -34, -21, -11, -28, -14, -24, -43,
    ],
];

/// Evaluates the board
/// Uses Piece-Square Tables a base, and augments values of individual pieces as fitting.
/// Concepts that aren't specific to a certain piece (e.g. doubled pawns) are evaluated seperately and added to the augmented PSQT score.
/// Throughout the function, scores are to be interpreted as follows:
/// - positive -> advantage for white,
/// - negative -> advantage for black,
///
/// !! Return type is relativized for the current player for negamax search
/// Unit = Centipawns, 100 Centipawns => 1 Pawn
impl Board {
    pub fn evaluate(&self) -> i32 {
        let white = 0usize;
        let black = 1usize;
        let mut mg = [0i32; 2];
        let mut eg = [0i32; 2];

        // cache the movement bitboards so this information can be used for both king safety and mobility
        let mut figure_movements = [Bitboard::EMPTY; 12];

        let open_files = self.open_files();

        let mut phase = TOTAL;
        for i in 0..=11 {
            let mut bb = self.figure_bb_by_index(i);

            // mobility - only needs to be done once per figure type
            if settings::KING_SAFETY {
                let figure_mobility = self.calculate_piece_mobility(i, &mut figure_movements);
                if settings::MOBILITY {
                    mg[i & 1] += MOBILITY_COEFFICIENTS[0][i >> 1] * figure_mobility;
                    eg[i & 1] += MOBILITY_COEFFICIENTS[1][i >> 1] * figure_mobility;
                }
                // for correctness, mobility calculation removes all pieces of the same color (we can't take our own piece)
                // however for safety calculation we would like to also count a piece as "reaching the king zone" if it is physically in the zone
                // So we re-add only the figure to the movement mask
                figure_movements[i] |= bb;
            }

            for bit in bb.iter_mut() {
                if open_files.is_position_set(bit) {
                    // rooks on open files
                    if settings::ROOKS_OPEN_FILES && (i == 6 || i == 7) {
                        mg[i & 1] += ROOK_OPEN_FILE_BONUS[0];
                        eg[i & 1] += ROOK_OPEN_FILE_BONUS[1];
                    }
                    if settings::KINGS_OPEN_FILES && (i == 10 && i == 11) {
                        mg[i & 1] += KING_OPEN_FILE_PENALTY[0];
                        eg[i & 1] += KING_OPEN_FILE_PENALTY[1];
                    }
                }

                let square = bit.to_square();
                mg[i & 1] += MG_TABLE[i][square];
                eg[i & 1] += EG_TABLE[i][square];

                phase -= GAMEPHASE_INC[i];
            }
        }
        // value is larger the less pieces are on the board
        // value is bound by the interval [0, 256]
        // (0 = starting position, 256 = only pawns and kings)
        let gamephase = (phase * 256 + (TOTAL / 2)) / TOTAL;

        let mut mg_score = mg[white] - mg[black];
        let mut eg_score = eg[white] - eg[black];

        let (mg_pawn_structure, eg_pawn_structure) = self.pawn_structure();
        mg_score += i32::from(mg_pawn_structure[white] - mg_pawn_structure[black]);
        eg_score += i32::from(eg_pawn_structure[white] - eg_pawn_structure[black]);

        if settings::BISHOP_PAIR {
            let (mg_bishop_pair, eg_bishop_pair) = self.bishop_pair_boni();
            mg_score += i32::from(mg_bishop_pair[white] - mg_bishop_pair[black]);
            eg_score += i32::from(eg_bishop_pair[white] - eg_bishop_pair[black]);
        }

        if settings::KING_SAFETY {
            let (mg_king_safety, eg_king_safety) = self.king_safety(&figure_movements);
            mg_score += i32::from(mg_king_safety[white] - mg_king_safety[black]);
            eg_score += i32::from(eg_king_safety[white] - eg_king_safety[black]);
        }

        let current_color_multiplier = match self.current_color() {
            White => 1,
            Black => -1,
        };

        if settings::INITIATIVE {
            mg_score += current_color_multiplier * INITIATIVE;
            eg_score += current_color_multiplier * INITIATIVE;
        }

        // Final aggregation of scoring aspects
        let mut score = (mg_score * (256 - gamephase) + eg_score * gamephase) >> 8;

        if settings::DOUBLED_PAWNS {
            let doubled_pawns = self.doubled_pawn_penalties();
            score += doubled_pawns[white] - doubled_pawns[black];
        }

        score * current_color_multiplier
    }

    /// Format:
    /// (mg: [white, black], eg: [white, black])
    pub fn pawn_structure(&self) -> ([i16; 2], [i16; 2]) {
        // [white, black]
        let mut mg_pawn_offset = [0i16; 2];
        let mut eg_pawn_offset = [0i16; 2];

        for i in 0..=1 {
            let color = Color::from_usize(i);
            for pawn in self.figure_bb_by_index(i).iter_mut() {
                if settings::PASSED_PAWNS {
                    let bonus = self.passed_pawn_bonus(pawn, color);
                    mg_pawn_offset[i] += bonus[0];
                    eg_pawn_offset[i] += bonus[1];
                }
                if settings::ISOLATED_PAWNS {
                    let penalty = self.isolated_pawn_penalty(pawn, color);
                    mg_pawn_offset[i] += penalty[0];
                    eg_pawn_offset[i] += penalty[1];
                }
            }
        }

        (mg_pawn_offset, eg_pawn_offset)
    }

    /// Format: `[MG, EG]`
    /// Note: Penalties are negative, i.e. should be added
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn isolated_pawn_penalty(&self, pawn: Bit, friendly: Color) -> [i16; 2] {
        let friendly_pawns = self.figure_bb(friendly, Piece::Pawn);
        let x = pawn.to_x() as i16;
        // we only look at the neighbouring files
        // double isolated pawns are still isolated, in fact an even worse liability than a singular isolated pawn
        let scan_mask = Bitboard::file((x - 1).max(0)) | Bitboard::file((x + 1).min(7));
        let is_isolated = i16::from((scan_mask & friendly_pawns).is_empty());

        [
            ISOLATED_PAWN_PENALTY[0] * is_isolated,
            ISOLATED_PAWN_PENALTY[1] * is_isolated,
        ]
    }

    /// Returns the passed pawn bonusses for `pawn`.
    /// Format: `[MG, EG]`.
    /// If the pawn is not passed, returns `[0, 0]`
    fn passed_pawn_bonus(&self, pawn: Bit, friendly: Color) -> [i16; 2] {
        let opponent_pawns = self.figure_bb(!friendly, Piece::Pawn);
        let scan_mask = Bitboard::passed_pawn_mask(pawn, friendly);
        let is_passed = i16::from((scan_mask & opponent_pawns).is_empty());

        let idx: usize = match friendly {
            White => flip(pawn.to_square().0),
            Black => pawn.to_square().0,
        };

        [
            is_passed * MG_PASSED_PAWN_TABLE[idx],
            is_passed * EG_PASSED_PAWN_TABLE[idx],
        ]
    }

    /// Calculate the penalties for doubled pawns for both white and black
    /// returns: 2-element array: `[white_penalty, black_penalty]`
    ///
    /// Note: penalties are negative for both sides
    /// TODO: this can probably be improved by weighing it against the remaining pawns (the less pawns are on the board, the worse it is if they're doubled)
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    pub fn doubled_pawn_penalties(&self) -> [i32; 2] {
        let white_pawns = self.figure_bb_by_index(0);
        let black_pawns = self.figure_bb_by_index(1);
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

            penalties[0] += file_pawns[0] * DOUBLED_PAWN_PENALTY;
            penalties[1] += file_pawns[1] * DOUBLED_PAWN_PENALTY;
        }

        penalties
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn calculate_piece_mobility(
        &self,
        figure: usize,
        figure_movements: &mut [Bitboard; 12],
    ) -> i32 {
        let attackmask = masks::calculate_attackmask_by_figure(self, self.occupied(), figure, None);
        figure_movements[figure] = attackmask;
        // here, don't count taking your own piece as a movable square
        // note that this distinction is made after we persisted the attack masks for the king safety calculations
        (attackmask & !self.color_bbs(Color::from_usize(figure & 1))).get_count() as i32
    }

    /// return Format:
    /// (mg: [white, black], eg: [white, black])
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    const fn bishop_pair_boni(&self) -> ([i16; 2], [i16; 2]) {
        let white_bishops = self.figure_bb(Color::White, Piece::Bishop).get_count();
        let black_bishops = self.figure_bb(Color::Black, Piece::Bishop).get_count();

        (
            [
                BISHOP_PAIR_BONUS[0] * (white_bishops >> 1) as i16,
                BISHOP_PAIR_BONUS[0] * (black_bishops >> 1) as i16,
            ],
            [
                BISHOP_PAIR_BONUS[1] * (white_bishops >> 1) as i16,
                BISHOP_PAIR_BONUS[1] * (black_bishops >> 1) as i16,
            ],
        )
    }

    /// Calculate a bitboard marking open files (files without any pawns on them)
    pub fn open_files(&self) -> Bitboard {
        let pawn_structure =
            self.figure_bb(Color::White, Piece::Pawn) | self.figure_bb(Color::Black, Piece::Pawn);
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
    /// return Format:
    /// (mg: [white, black], eg: [white, black])
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::similar_names
    )]
    pub fn king_safety(&self, figure_movements: &[Bitboard; 12]) -> ([i16; 2], [i16; 2]) {
        let mut mg_safety = [0i16; 2];
        let mut eg_safety = [0i16; 2];

        let king_zones = [
            king_safety_mask(self, Color::White),
            king_safety_mask(self, Color::Black),
        ];

        // skip pawns and kings in the evaluation
        for i in 2..=9 {
            let friend = i & 1;
            let opp = friend ^ 1;
            let attacker_mg = PIECE_ATTACK_VALUES[0][i >> 1];
            let attacker_eg = PIECE_ATTACK_VALUES[1][i >> 1];
            let defender_mg = PIECE_DEFEND_VALUES[0][i >> 1];
            let defender_eg = PIECE_DEFEND_VALUES[1][i >> 1];

            // println!("{:?}", Figure::from_idx(i));
            // println!(
            //     "defend: {:?}, -> {} points",
            //     (king_zones[friend] & figure_movements[i]).get_count(),
            //     (king_zones[friend] & figure_movements[i]).get_count() as i16 * defender_mg
            // );
            // println!(
            //     "attack: {:?}, -> {} points",
            //     (king_zones[opp] & figure_movements[i]).get_count(),
            //     (king_zones[opp] & figure_movements[i]).get_count() as i16 * attacker_mg
            // );

            // decrease friendly danger score for friendly piece in friendly king zone
            mg_safety[friend] +=
                (king_zones[friend] & figure_movements[i]).get_count() as i16 * defender_mg;
            eg_safety[friend] +=
                (king_zones[friend] & figure_movements[i]).get_count() as i16 * defender_eg;

            // increase opponent's danger score for friendly piece in opponents king zone
            mg_safety[opp] -=
                (king_zones[opp] & figure_movements[i]).get_count() as i16 * attacker_mg;
            eg_safety[opp] -=
                (king_zones[opp] & figure_movements[i]).get_count() as i16 * attacker_eg;
        }
        // println!("mg score: {mg_safety:?}");
        #[allow(clippy::cast_sign_loss)]
        (
            [
                mg_safety[0].signum()
                    * MG_KING_SAFETY_TABLE[mg_safety[0].abs().clamp(0, 99) as usize],
                mg_safety[1].signum()
                    * MG_KING_SAFETY_TABLE[mg_safety[1].abs().clamp(0, 99) as usize],
            ],
            [
                eg_safety[0].signum()
                    * EG_KING_SAFETY_TABLE[mg_safety[0].abs().clamp(0, 99) as usize],
                eg_safety[1].signum()
                    * EG_KING_SAFETY_TABLE[mg_safety[1].abs().clamp(0, 99) as usize],
            ],
        )
    }
}
