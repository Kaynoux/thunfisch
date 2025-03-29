use crate::types::{Bit, Bitboard, Board, Color};
use crate::utils::{bit_from_x_y, is_axis_in_bounds, is_bit_set, is_pos_friendly};

// pub fn generate_pseudo_legal_pawn_moves(board: [[Field; 8]; 8], field: Field) -> Vec<Position> {
//     let mut pseudo_legal_positions: Vec<Position> = Vec::new();
//     let mut target_positions_1: Vec<Position> = Vec::new();
//     let mut target_positions_2: Vec<Position> = Vec::new();
//     let move_direction = match field.color {
//         Color::Black => -1,
//         Color::White => 1,
//         Color::None => panic!("Pawn provided is not valid"),
//     };

//     Adds potential positions
//     match (field.position, field.color) {
//         (Position { x: _, y: 1 }, Color::White) => {
//             target_positions_1.push(Position {
//                 x: field.position.x,
//                 y: field.position.y + move_direction,
//             });
//             target_positions_2.push(Position {
//                 x: field.position.x,
//                 y: field.position.y + (move_direction * 2),
//             })
//         }
//         (Position { x: _, y: 6 }, Color::Black) => {
//             target_positions_1.push(Position {
//                 x: field.position.x,
//                 y: field.position.y + move_direction,
//             });
//             target_positions_2.push(Position {
//                 x: field.position.x,
//                 y: field.position.y + (move_direction * 2),
//             })
//         }

//         (_, _) => target_positions_1.push(Position {
//             x: field.position.x,
//             y: field.position.y + move_direction,
//         }),
//     };

//     Adds psedo leglal position when moved by one field
//     for target_pos in target_positions_1.iter() {
//         Skip if out of bounds
//         if !target_pos.is_within_bounds() {
//             continue;
//         }

//         Skip if target field is not empty
//         let forward_field: Field = get_field(board, *target_pos);
//         if forward_field.color != Color::None {
//             continue;
//         }

//         pseudo_legal_positions.push(*target_pos);
//     }

//     Adds psedo leglal position when moved by two field
//     for target_pos in target_positions_2.iter() {
//         Skip if out of bounds
//         if !target_pos.is_within_bounds() {
//             continue;
//         }

//         Skip if target field is not empty
//         let forward_field: Field = get_field(board, *target_pos);
//         if forward_field.color != Color::None {
//             continue;
//         }

//         Skip if field in between is not free
//         if get_field(
//             board,
//             Position {
//                 x: target_pos.x,
//                 y: target_pos.y - move_direction,
//             },
//         )
//         .color
//             != Color::None
//         {
//             continue;
//         }

//         pseudo_legal_positions.push(*target_pos);
//     }

//     Adds possible strikes
//     let potential_strike_moves: [Move; 2] = [Move { x: 1, y: 1 }, Move { x: -1, y: 1 }];
//     for potential_strike_move in potential_strike_moves.iter() {
//         let target_pos = field.position
//             + Move {
//                 x: potential_strike_move.x,
//                 y: potential_strike_move.y * move_direction,
//             };

//         if !target_pos.is_within_bounds() {
//             continue;
//         }

//         let target_color = get_field(board, target_pos).color;
//         if target_color != field.color.opposite() {
//             continue;
//         }

//         pseudo_legal_positions.push(target_pos);
//     }

//     pseudo_legal_positions
// }

fn get_offset_moves(
    board: &Board,
    pos: usize,
    color: Color,
    move_offsets: &[(i32, i32)],
) -> Bitboard {
    let mut pseudo_legal_moves: Bitboard = 0u64;
    let pos_x = (pos % 8) as i32;
    let pos_y = (pos / 8) as i32;

    for (off_x, off_y) in move_offsets.iter() {
        let new_x = pos_x + off_x;
        let new_y = pos_y + off_y;

        // Skip if out of bounds
        if !is_axis_in_bounds(new_x) || !is_axis_in_bounds(new_y) {
            continue;
        }

        // Skip if friendly pice already there
        let new_pos: Bit = bit_from_x_y(new_x, new_y);
        if is_pos_friendly(board, new_pos, color) {
            continue;
        }

        // Add new pos to list of pseudo legal moves
        pseudo_legal_moves |= new_pos;
    }
    pseudo_legal_moves
}

pub fn get_pseudo_legal_king_moves(board: &Board, pos: usize, color: Color) -> Bitboard {
    static KING_MOVE_OFFSETS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    get_offset_moves(board, pos, color, &KING_MOVE_OFFSETS)
}

pub fn get_pseudo_legal_knight_moves(board: &Board, pos: usize, color: Color) -> Bitboard {
    static KNIGHT_MOVE_OFFSETS: [(i32, i32); 8] = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];
    get_offset_moves(board, pos, color, &KNIGHT_MOVE_OFFSETS)
}
