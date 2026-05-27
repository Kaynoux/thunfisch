//! Tunable evaluation parameters for Thunfisch.
//!
//! All constants that affect evaluation are collected here so the engine can
//! pass a runtime parameter set into the evaluation logic.

use std::{fs, io, path::Path};

use ngalgebra::SVector;
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////// Weight Counting ///////////////////////////////////////////////////////

const N_PIECES: usize = 6;
const N_COLORS: usize = 2;
const N_SQUARES: usize = 8 * 8;
const N_KING_SAFETY_BUCKETS: usize = 10 * 10;
pub const N_TOTAL: usize = (N_PIECES * 2)
    + (N_COLORS * 2)
    + 1
    + (N_COLORS * 2)
    + (N_COLORS * N_PIECES * 3)
    + N_COLORS
    + 1
    + (N_SQUARES * 2)
    + (N_KING_SAFETY_BUCKETS * 2)
    + (N_PIECES * N_SQUARES * 2);
pub type WeightVector = SVector<f64, N_TOTAL>;

/// A runtime-configurable parameter set for the evaluator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TunableParams {
    pub mg_piece_values: [i32; N_PIECES],
    pub eg_piece_values: [i32; N_PIECES],

    pub rook_open_file_bonus: [i32; N_COLORS],
    pub king_open_file_penalty: [i32; N_COLORS],

    pub doubled_pawn_penalty: i32,
    pub isolated_pawn_penalty: [i16; N_COLORS],
    pub bishop_pair_bonus: [i16; N_COLORS],

    pub mobility_coefficients: [[i32; N_PIECES]; N_COLORS],
    pub piece_attack_values: [[i16; N_PIECES]; N_COLORS],
    pub piece_defend_values: [[i16; N_PIECES]; N_COLORS],
    pub pawn_shield_bonus: [i16; N_COLORS],

    pub initiative: i32,

    pub mg_passed_pawn_table: [i16; N_SQUARES],
    pub eg_passed_pawn_table: [i16; N_SQUARES],
    pub mg_king_safety_table: [i16; N_KING_SAFETY_BUCKETS],
    pub eg_king_safety_table: [i16; N_KING_SAFETY_BUCKETS],

    pub mg_base_position_table: [[i32; N_SQUARES]; N_PIECES],
    pub eg_base_position_table: [[i32; N_SQUARES]; N_PIECES],
}

impl Default for TunableParams {
    fn default() -> Self {
        Self {
            mg_piece_values: MG_PIECE_VALUES,
            eg_piece_values: EG_PIECE_VALUES,
            rook_open_file_bonus: ROOK_OPEN_FILE_BONUS,
            king_open_file_penalty: KING_OPEN_FILE_PENALTY,
            doubled_pawn_penalty: DOUBLED_PAWN_PENALTY,
            isolated_pawn_penalty: ISOLATED_PAWN_PENALTY,
            bishop_pair_bonus: BISHOP_PAIR_BONUS,
            mobility_coefficients: MOBILITY_COEFFICIENTS,
            piece_attack_values: PIECE_ATTACK_VALUES,
            piece_defend_values: PIECE_DEFEND_VALUES,
            pawn_shield_bonus: PAWN_SHIELD_BONUS,
            initiative: INITIATIVE,
            mg_passed_pawn_table: MG_PASSED_PAWN_TABLE,
            eg_passed_pawn_table: EG_PASSED_PAWN_TABLE,
            mg_king_safety_table: MG_KING_SAFETY_TABLE,
            eg_king_safety_table: EG_KING_SAFETY_TABLE,
            mg_base_position_table: MG_BASE_POSITION_TABLE,
            eg_base_position_table: EG_BASE_POSITION_TABLE,
        }
    }
}

impl TunableParams {
    /// Read tunable parameters from a JSON file.
    pub fn read_from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let file_params: TunableParamsFile = serde_json::from_str(&contents)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        file_params.try_into()
    }

    /// Write tunable parameters to a JSON file.
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let json = serde_json::to_string(&TunableParamsFile::from(self))
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        fs::write(path, json)
    }

    /// Convert the parameters into the optimizer's fixed-size vector.
    pub fn to_weight_vector(&self) -> WeightVector {
        self.into()
    }

    /// Reconstruct parameters from the optimizer's fixed-size vector.
    pub fn from_weight_vector(values: WeightVector) -> Self {
        values.into()
    }
}

impl From<&TunableParams> for WeightVector {
    fn from(params: &TunableParams) -> Self {
        let mut values = Vec::with_capacity(N_TOTAL);

        values.extend(params.mg_piece_values.iter().copied().map(f64::from));
        values.extend(params.eg_piece_values.iter().copied().map(f64::from));

        values.extend(params.rook_open_file_bonus.iter().copied().map(f64::from));
        values.extend(params.king_open_file_penalty.iter().copied().map(f64::from));

        values.push(f64::from(params.doubled_pawn_penalty));
        values.extend(params.isolated_pawn_penalty.iter().copied().map(f64::from));
        values.extend(params.bishop_pair_bonus.iter().copied().map(f64::from));

        for row in params.mobility_coefficients {
            values.extend(row.iter().copied().map(f64::from));
        }
        for row in params.piece_attack_values {
            values.extend(row.iter().copied().map(f64::from));
        }
        for row in params.piece_defend_values {
            values.extend(row.iter().copied().map(f64::from));
        }

        values.extend(params.pawn_shield_bonus.iter().copied().map(f64::from));
        values.push(f64::from(params.initiative));

        values.extend(params.mg_passed_pawn_table.iter().copied().map(f64::from));
        values.extend(params.eg_passed_pawn_table.iter().copied().map(f64::from));
        values.extend(params.mg_king_safety_table.iter().copied().map(f64::from));
        values.extend(params.eg_king_safety_table.iter().copied().map(f64::from));

        for row in params.mg_base_position_table {
            values.extend(row.iter().copied().map(f64::from));
        }
        for row in params.eg_base_position_table {
            values.extend(row.iter().copied().map(f64::from));
        }

        SVector::from_row_slice(&values)
    }
}

impl From<TunableParams> for WeightVector {
    fn from(params: TunableParams) -> Self {
        SVector::from(&params)
    }
}

impl From<WeightVector> for TunableParams {
    fn from(values: WeightVector) -> Self {
        let values = values.as_slice();
        let mut index = 0;

        fn read_f64(values: &[f64], index: &mut usize) -> f64 {
            let value = values[*index];
            *index += 1;
            value
        }

        fn read_i32(values: &[f64], index: &mut usize) -> i32 {
            read_f64(values, index) as i32
        }

        fn read_i16(values: &[f64], index: &mut usize) -> i16 {
            read_f64(values, index) as i16
        }

        fn read_i32_array<const N: usize>(values: &[f64], index: &mut usize) -> [i32; N] {
            std::array::from_fn(|_| read_i32(values, index))
        }

        fn read_i16_array<const N: usize>(values: &[f64], index: &mut usize) -> [i16; N] {
            std::array::from_fn(|_| read_i16(values, index))
        }

        fn read_i32_table<const ROWS: usize, const COLS: usize>(
            values: &[f64],
            index: &mut usize,
        ) -> [[i32; COLS]; ROWS] {
            std::array::from_fn(|_| read_i32_array::<COLS>(values, index))
        }

        fn read_i16_table<const ROWS: usize, const COLS: usize>(
            values: &[f64],
            index: &mut usize,
        ) -> [[i16; COLS]; ROWS] {
            std::array::from_fn(|_| read_i16_array::<COLS>(values, index))
        }

        Self {
            mg_piece_values: read_i32_array::<N_PIECES>(values, &mut index),
            eg_piece_values: read_i32_array::<N_PIECES>(values, &mut index),
            rook_open_file_bonus: read_i32_array::<N_COLORS>(values, &mut index),
            king_open_file_penalty: read_i32_array::<N_COLORS>(values, &mut index),
            doubled_pawn_penalty: read_i32(values, &mut index),
            isolated_pawn_penalty: read_i16_array::<N_COLORS>(values, &mut index),
            bishop_pair_bonus: read_i16_array::<N_COLORS>(values, &mut index),
            mobility_coefficients: read_i32_table::<N_COLORS, N_PIECES>(values, &mut index),
            piece_attack_values: read_i16_table::<N_COLORS, N_PIECES>(values, &mut index),
            piece_defend_values: read_i16_table::<N_COLORS, N_PIECES>(values, &mut index),
            pawn_shield_bonus: read_i16_array::<N_COLORS>(values, &mut index),
            initiative: read_i32(values, &mut index),
            mg_passed_pawn_table: read_i16_array::<N_SQUARES>(values, &mut index),
            eg_passed_pawn_table: read_i16_array::<N_SQUARES>(values, &mut index),
            mg_king_safety_table: read_i16_array::<N_KING_SAFETY_BUCKETS>(values, &mut index),
            eg_king_safety_table: read_i16_array::<N_KING_SAFETY_BUCKETS>(values, &mut index),
            mg_base_position_table: read_i32_table::<N_PIECES, N_SQUARES>(values, &mut index),
            eg_base_position_table: read_i32_table::<N_PIECES, N_SQUARES>(values, &mut index),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////// HELPERS FOR SERIALIZATION ////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TunableParamsFile {
    mg_piece_values: [i32; N_PIECES],
    eg_piece_values: [i32; N_PIECES],

    rook_open_file_bonus: [i32; N_COLORS],
    king_open_file_penalty: [i32; N_COLORS],

    doubled_pawn_penalty: i32,
    isolated_pawn_penalty: [i16; N_COLORS],
    bishop_pair_bonus: [i16; N_COLORS],

    mobility_coefficients: [[i32; N_PIECES]; N_COLORS],
    piece_attack_values: [[i16; N_PIECES]; N_COLORS],
    piece_defend_values: [[i16; N_PIECES]; N_COLORS],
    pawn_shield_bonus: [i16; N_COLORS],

    initiative: i32,

    mg_passed_pawn_table: Vec<i16>,
    eg_passed_pawn_table: Vec<i16>,
    mg_king_safety_table: Vec<i16>,
    eg_king_safety_table: Vec<i16>,

    mg_base_position_table: Vec<Vec<i32>>,
    eg_base_position_table: Vec<Vec<i32>>,
}

impl From<&TunableParams> for TunableParamsFile {
    fn from(params: &TunableParams) -> Self {
        Self {
            mg_piece_values: params.mg_piece_values,
            eg_piece_values: params.eg_piece_values,
            rook_open_file_bonus: params.rook_open_file_bonus,
            king_open_file_penalty: params.king_open_file_penalty,
            doubled_pawn_penalty: params.doubled_pawn_penalty,
            isolated_pawn_penalty: params.isolated_pawn_penalty,
            bishop_pair_bonus: params.bishop_pair_bonus,
            mobility_coefficients: params.mobility_coefficients,
            piece_attack_values: params.piece_attack_values,
            piece_defend_values: params.piece_defend_values,
            pawn_shield_bonus: params.pawn_shield_bonus,
            initiative: params.initiative,
            mg_passed_pawn_table: params.mg_passed_pawn_table.to_vec(),
            eg_passed_pawn_table: params.eg_passed_pawn_table.to_vec(),
            mg_king_safety_table: params.mg_king_safety_table.to_vec(),
            eg_king_safety_table: params.eg_king_safety_table.to_vec(),
            mg_base_position_table: params
                .mg_base_position_table
                .iter()
                .map(|row| row.to_vec())
                .collect(),
            eg_base_position_table: params
                .eg_base_position_table
                .iter()
                .map(|row| row.to_vec())
                .collect(),
        }
    }
}

impl TryFrom<TunableParamsFile> for TunableParams {
    type Error = io::Error;

    fn try_from(file: TunableParamsFile) -> Result<Self, Self::Error> {
        Ok(Self {
            mg_piece_values: file.mg_piece_values,
            eg_piece_values: file.eg_piece_values,
            rook_open_file_bonus: file.rook_open_file_bonus,
            king_open_file_penalty: file.king_open_file_penalty,
            doubled_pawn_penalty: file.doubled_pawn_penalty,
            isolated_pawn_penalty: file.isolated_pawn_penalty,
            bishop_pair_bonus: file.bishop_pair_bonus,
            mobility_coefficients: file.mobility_coefficients,
            piece_attack_values: file.piece_attack_values,
            piece_defend_values: file.piece_defend_values,
            pawn_shield_bonus: file.pawn_shield_bonus,
            initiative: file.initiative,
            mg_passed_pawn_table: to_array_64(file.mg_passed_pawn_table, "mg_passed_pawn_table")?,
            eg_passed_pawn_table: to_array_64(file.eg_passed_pawn_table, "eg_passed_pawn_table")?,
            mg_king_safety_table: to_array_100(file.mg_king_safety_table, "mg_king_safety_table")?,
            eg_king_safety_table: to_array_100(file.eg_king_safety_table, "eg_king_safety_table")?,
            mg_base_position_table: to_table_6x64(file.mg_base_position_table, "mg_base_position_table")?,
            eg_base_position_table: to_table_6x64(file.eg_base_position_table, "eg_base_position_table")?,
        })
    }
}

fn to_array_64(values: Vec<i16>, field_name: &str) -> io::Result<[i16; N_SQUARES]> {
    values
        .try_into()
        .map_err(|_| invalid_length(field_name, N_SQUARES))
}

fn to_array_100(values: Vec<i16>, field_name: &str) -> io::Result<[i16; N_KING_SAFETY_BUCKETS]> {
    values
        .try_into()
        .map_err(|_| invalid_length(field_name, N_KING_SAFETY_BUCKETS))
}

fn to_table_6x64(values: Vec<Vec<i32>>, field_name: &str) -> io::Result<[[i32; N_SQUARES]; N_PIECES]> {
    let rows: Vec<[i32; N_SQUARES]> = values
        .into_iter()
        .map(|row| row.try_into().map_err(|_| invalid_length(field_name, N_SQUARES)))
        .collect::<io::Result<Vec<_>>>()?;

    rows.try_into().map_err(|_| invalid_length(field_name, N_PIECES))
}

fn invalid_length(field_name: &str, expected: usize) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("field `{field_name}` has the wrong length, expected {expected}"),
    )
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////// Default Constants /////////////////////////////////////////////////////

// [pawn, knight, bishop, rook, queen, king]
pub const MG_PIECE_VALUES: [i32; N_PIECES] = [82, 337, 365, 477, 1025, 0];
pub const EG_PIECE_VALUES: [i32; N_PIECES] = [94, 281, 297, 512, 936, 0];

// rooks on open files are a rather weak positional idea so this should be kept pretty low
// Additionally, Endgames typically have a lot of open files, so there's no benefit to occupying one (hence 0 EG score)
// format: [MG, EG]
pub const ROOK_OPEN_FILE_BONUS: [i32; N_COLORS] = [25, 0];
pub const KING_OPEN_FILE_PENALTY: [i32; N_COLORS] = [-25, 0];

// TODO: probably interpolation of these values between MG and EG makes sense
// a doubled pawn should be worth about half a pawn
// For now we just linearly scale this; may be worth tho looking at punishing tripled pawns harder than doubled pawns
pub const DOUBLED_PAWN_PENALTY: i32 = -10;

// A pawn is isolated if it has no pawns on the file next to it
// Generally isolated pawns are bad as they require pieces to defend and thus are easy targets
pub const ISOLATED_PAWN_PENALTY: [i16; N_COLORS] = [-23, -13];

// Some say having the bishop pair is a slight advantage because having only one bishop essentially makes half the board unreachable
// Personally I'm indifferent to the bishop pair but it's an easy implementation and may gain a little bit
pub const BISHOP_PAIR_BONUS: [i16; N_COLORS] = [15, 25];

pub const MOBILITY_COEFFICIENTS: [[i32; N_PIECES]; N_COLORS] = [[0, 5, 3, 2, 1, 0], [0, 5, 3, 4, 1, 0]];

pub const PIECE_ATTACK_VALUES: [[i16; N_PIECES]; N_COLORS] = [[0, 2, 2, 3, 5, 0], [0, 2, 2, 3, 5, 0]];
pub const PIECE_DEFEND_VALUES: [[i16; N_PIECES]; N_COLORS] = [[0, 1, 1, 2, 4, 0], [0, 1, 1, 2, 3, 0]];

// danger points for every pawn in the pawn shield in front of the king
pub const PAWN_SHIELD_BONUS: [i16; N_COLORS] = [2, 0];

// bonus for the side to move
pub const INITIATIVE: i32 = 15;



/// Values here are vaguely inspired in their Shape by Fatalii
/// However I've changed them to only give bonusses and never subtract from the eval
/// let's see how this does haha
#[rustfmt::skip]
pub const MG_PASSED_PAWN_TABLE: [i16; N_SQUARES] = [
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
pub const EG_PASSED_PAWN_TABLE: [i16; N_SQUARES] = [
         0,    0,    0,    0,    0,    0,    0,    0,
         0,    0,    0,    0,    0,    0,    0,    0,
        90,   85,   80,   70,   70,   80,   85,   90,
        65,   60,   55,   45,   45,   55,   60,   65,
        40,   35,   30,   20,   20,   30,   35,   40,
        30,   25,   20,   10,   10,   20,   25,   30,
        25,   20,   15,    5,    5,   15,   20,   25,
         0,    0,    0,    0,    0,    0,    0,    0,
];

pub const MG_KING_SAFETY_TABLE: [i16; N_KING_SAFETY_BUCKETS] = [
    0, 0, 1, 2, 3, 5, 7, 9, 12, 15, 18, 22, 26, 30, 35, 39, 44, 50, 56, 62, 68, 75, 82, 85, 89, 97,
    105, 113, 122, 131, 140, 150, 169, 180, 191, 202, 213, 225, 237, 248, 260, 272, 283, 295, 307,
    319, 330, 342, 354, 366, 377, 389, 401, 412, 424, 436, 448, 459, 471, 483, 494, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
];
pub const EG_KING_SAFETY_TABLE: [i16; N_KING_SAFETY_BUCKETS] = [
    0, 0, 0, 1, 2, 3, 4, 6, 8, 10, 12, 14, 17, 20, 23, 26, 29, 33, 37, 41, 45, 50, 54, 56, 59, 64,
    70, 75, 81, 87, 93, 100, 112, 120, 127, 134, 142, 150, 158, 165, 173, 181, 188, 196, 204, 212,
    220, 228, 236, 244, 251, 259, 267, 274, 282, 290, 298, 306, 314, 322, 329, 333, 333, 333, 333,
    333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333,
    333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333, 333,
];

/// Middlegame-Piece-Square Tables Base Values used for calculation of the real eval values
pub const MG_BASE_POSITION_TABLE: [[i32; N_SQUARES]; N_PIECES] = [
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
pub const EG_BASE_POSITION_TABLE: [[i32; N_SQUARES]; N_PIECES] = [
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

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////// TESTS & INFO PRINTS ///////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{eval::evaluation::evaluate, training_data::TrainingSample};
    use std::{fs, path::Path, time::{Instant, SystemTime, UNIX_EPOCH}};
    use thunfisch::types::board::Board;

    fn next_u64(state: &mut u64) -> u64 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        *state
    }

    fn random_i32(state: &mut u64, magnitude: i32) -> i32 {
        (next_u64(state) % (2 * magnitude as u64 + 1)) as i32 - magnitude
    }

    fn random_i16(state: &mut u64, magnitude: i16) -> i16 {
        (next_u64(state) % (2 * magnitude as u64 + 1)) as i16 - magnitude
    }

    fn random_i32_array<const N: usize>(state: &mut u64, magnitude: i32) -> [i32; N] {
        std::array::from_fn(|_| random_i32(state, magnitude))
    }

    fn random_i16_array<const N: usize>(state: &mut u64, magnitude: i16) -> [i16; N] {
        std::array::from_fn(|_| random_i16(state, magnitude))
    }

    fn random_params(state: &mut u64) -> TunableParams {
        TunableParams {
            mg_piece_values: random_i32_array::<N_PIECES>(state, 100_000),
            eg_piece_values: random_i32_array::<N_PIECES>(state, 100_000),
            rook_open_file_bonus: random_i32_array::<N_COLORS>(state, 5_000),
            king_open_file_penalty: random_i32_array::<N_COLORS>(state, 5_000),
            doubled_pawn_penalty: random_i32(state, 5_000),
            isolated_pawn_penalty: random_i16_array::<N_COLORS>(state, 5_000),
            bishop_pair_bonus: random_i16_array::<N_COLORS>(state, 5_000),
            mobility_coefficients: std::array::from_fn(|_| random_i32_array::<N_PIECES>(state, 100)),
            piece_attack_values: std::array::from_fn(|_| random_i16_array::<N_PIECES>(state, 100)),
            piece_defend_values: std::array::from_fn(|_| random_i16_array::<N_PIECES>(state, 100)),
            pawn_shield_bonus: random_i16_array::<N_COLORS>(state, 100),
            initiative: random_i32(state, 5_000),
            mg_passed_pawn_table: random_i16_array::<N_SQUARES>(state, 200),
            eg_passed_pawn_table: random_i16_array::<N_SQUARES>(state, 200),
            mg_king_safety_table: random_i16_array::<N_KING_SAFETY_BUCKETS>(state, 500),
            eg_king_safety_table: random_i16_array::<N_KING_SAFETY_BUCKETS>(state, 500),
            mg_base_position_table: std::array::from_fn(|_| random_i32_array::<N_SQUARES>(state, 2_000)),
            eg_base_position_table: std::array::from_fn(|_| random_i32_array::<N_SQUARES>(state, 2_000)),
        }
    }

    #[test]
    fn default_params_round_trip_json() {
        let params = TunableParams::default();
        let unique_suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is before UNIX_EPOCH")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("tunable_params_round_trip_{unique_suffix}.json"));

        params
            .write_to_file(&path)
            .expect("default tunable params should be written successfully");

        let reloaded = TunableParams::read_from_file(&path)
            .expect("default tunable params should be read successfully");

        assert_eq!(params, reloaded);

        fs::remove_file(&path).expect("temporary tunable params file should be removable");
    }

    #[test]
    fn svector_round_trip_is_isomorph() {
        let mut state = 0x5eede67_u64;
        let params = random_params(&mut state);

        let vector = params.to_weight_vector();
        let reconstructed = TunableParams::from(vector);

        assert_eq!(params, reconstructed);
        assert_eq!(vector, reconstructed.to_weight_vector());
    }

    #[test]
    fn svector_round_trip_preserves_evaluation() {
        let params = TunableParams::default();
        let round_tripped = TunableParams::from(params.to_weight_vector());

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("_zurichess_short.epd");
        let training_data = TrainingSample::read_epd_file(&path)
            .expect("training positions should be readable");

        for sample in training_data.iter().take(10) {
            let board = Board::new(&sample.fen);
            let before = evaluate(&board, &params);
            let after = evaluate(&board, &round_tripped);

            assert_eq!(before, after, "evaluation changed for FEN {}", sample.fen);
        }
    }

    #[test]
    fn benchmark_conversion() {
        use std::hint::black_box;
        let mut state = 0x5eede67_u64;
        let params = (0..10_000).map(|_| random_params(&mut state));

        let start = Instant::now();
        for p in params {
            black_box(p.to_weight_vector());
        }
        println!("time: {:?}", start.elapsed() / 10_000);
    }

    #[test]
    fn print_n_total() {
        println!("{N_TOTAL}");
    }
}
