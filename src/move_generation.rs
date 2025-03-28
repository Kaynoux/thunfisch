use crate::types::{Color, Field, Move, Position};
use crate::utils::get_field;

pub fn generate_pseudo_legal_pawn_moves(board: [[Field; 8]; 8], field: Field) -> Vec<Position> {
    let mut pseudo_legal_positions: Vec<Position> = Vec::new();
    let mut target_positions_1: Vec<Position> = Vec::new();
    let mut target_positions_2: Vec<Position> = Vec::new();
    let move_direction = match field.color {
        Color::Black => -1,
        Color::White => 1,
        Color::None => panic!("Pawn provided is not valid"),
    };

    //Adds potential positions
    match (field.position, field.color) {
        (Position { x: _, y: 1 }, Color::White) => {
            target_positions_1.push(Position {
                x: field.position.x,
                y: field.position.y + move_direction,
            });
            target_positions_2.push(Position {
                x: field.position.x,
                y: field.position.y + (move_direction * 2),
            })
        }
        (Position { x: _, y: 6 }, Color::Black) => {
            target_positions_1.push(Position {
                x: field.position.x,
                y: field.position.y + move_direction,
            });
            target_positions_2.push(Position {
                x: field.position.x,
                y: field.position.y + (move_direction * 2),
            })
        }

        (_, _) => target_positions_1.push(Position {
            x: field.position.x,
            y: field.position.y + move_direction,
        }),
    };

    // Adds psedo leglal position when moved by one field
    for target_pos in target_positions_1.iter() {
        // Skip if out of bounds
        if !target_pos.is_within_bounds() {
            continue;
        }

        // Skip if target field is not empty
        let forward_field: Field = get_field(board, *target_pos);
        if forward_field.color != Color::None {
            continue;
        }

        pseudo_legal_positions.push(*target_pos);
    }

    // Adds psedo leglal position when moved by two field
    for target_pos in target_positions_2.iter() {
        // Skip if out of bounds
        if !target_pos.is_within_bounds() {
            continue;
        }

        // Skip if target field is not empty
        let forward_field: Field = get_field(board, *target_pos);
        if forward_field.color != Color::None {
            continue;
        }

        // Skip if field in between is not free
        if get_field(
            board,
            Position {
                x: target_pos.x,
                y: target_pos.y - move_direction,
            },
        )
        .color
            != Color::None
        {
            continue;
        }

        pseudo_legal_positions.push(*target_pos);
    }

    // Adds possible strikes
    let potential_strike_moves: [Move; 2] = [Move { x: 1, y: 1 }, Move { x: -1, y: 1 }];
    for potential_strike_move in potential_strike_moves.iter() {
        let target_pos = field.position
            + Move {
                x: potential_strike_move.x,
                y: potential_strike_move.y * move_direction,
            };

        if !target_pos.is_within_bounds() {
            continue;
        }

        let target_color = get_field(board, target_pos).color;
        if target_color != field.color.opposite() {
            continue;
        }

        pseudo_legal_positions.push(target_pos);
    }

    pseudo_legal_positions
}

pub fn generate_pseudo_legal_king_moves(board: [[Field; 8]; 8], field: Field) -> Vec<Position> {
    let potential_moves: [Move; 8] = [
        Move { x: -1, y: -1 },
        Move { x: -1, y: 0 },
        Move { x: -1, y: 1 },
        Move { x: 0, y: -1 },
        Move { x: 0, y: 1 },
        Move { x: 1, y: -1 },
        Move { x: 1, y: 0 },
        Move { x: 1, y: 1 },
    ];
    generate_pseudo_legal_king_or_knight_moves(board, field, potential_moves)
}

pub fn generate_pseudo_legal_knight_moves(board: [[Field; 8]; 8], field: Field) -> Vec<Position> {
    let potential_moves: [Move; 8] = [
        Move { x: -2, y: -1 },
        Move { x: -2, y: 1 },
        Move { x: -1, y: -2 },
        Move { x: -1, y: 2 },
        Move { x: 1, y: -2 },
        Move { x: 1, y: 2 },
        Move { x: 2, y: -1 },
        Move { x: 2, y: 1 },
    ];
    generate_pseudo_legal_king_or_knight_moves(board, field, potential_moves)
}

pub fn generate_pseudo_legal_king_or_knight_moves(
    board: [[Field; 8]; 8],
    field: Field,
    potential_moves: [Move; 8],
) -> Vec<Position> {
    let mut pseudo_legal_positions: Vec<Position> = Vec::new();

    for potenial_move in potential_moves.iter() {
        let target_pos = field.position + *potenial_move;

        // Check if target out of bounds
        if !target_pos.is_within_bounds() {
            continue;
        }

        // Skip if target field is friendly
        let forward_field: Field = get_field(board, target_pos);
        if forward_field.color == field.color {
            continue;
        }

        pseudo_legal_positions.push(target_pos);
    }

    pseudo_legal_positions
}
