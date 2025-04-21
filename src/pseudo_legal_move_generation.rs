use crate::position_generation::*;
use crate::prelude::*;
pub fn get_all_moves(board: &Board, color: Color, moves: &mut Vec<ChessMove>) -> Bitboard {
    let mut moves_bitboard = Bitboard(0);

    // Pawn by 1
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.get_positions_by_piece_color(color, Piece::Pawn),
        color,
        moves,
        false,
        get_pawn_positions,
    );

    // Knight
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.get_positions_by_piece_color(color, Piece::Knight),
        color,
        moves,
        false,
        get_knight_positions,
    );

    // Bishop
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.get_positions_by_piece_color(color, Piece::Bishop),
        color,
        moves,
        false,
        get_bishop_positions,
    );

    // Rook
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.get_positions_by_piece_color(color, Piece::Rook),
        color,
        moves,
        false,
        get_rook_positions,
    );

    // Queen
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.get_positions_by_piece_color(color, Piece::Queen),
        color,
        moves,
        false,
        get_queen_positions,
    );

    // King
    let king_pos = board.get_king_pos(color);
    if king_pos != Position(0) {
        moves_bitboard |= get_king_moves(board, king_pos, color, moves, get_king_positions);
    }

    // Double pawn moves
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.get_positions_by_piece_color(color, Piece::Pawn),
        color,
        moves,
        true,
        get_pawn_double_positions,
    );

    get_castle_moves(board, color, moves);
    get_promotions_moves(board, color, moves);
    get_en_passant_moves(board, color, moves);
    moves_bitboard
}

/// Calculate all moves for each instance of a piece type exept the king because it is unique
pub fn get_moves_for_piece_type(
    board: &Board,
    mut piece_positions: Bitboard,
    color: Color,
    moves: &mut Vec<ChessMove>,
    is_double_move: bool,
    f: fn(board: &Board, pos: Position, color: Color) -> Bitboard,
) -> Bitboard {
    let mut target_positions = Bitboard(0);

    while piece_positions != Bitboard(0) {
        let current_pos = piece_positions.pop_lsb_position();

        let mut target_positions_for_one_piece = f(board, current_pos, color);
        target_positions |= target_positions_for_one_piece;

        while target_positions_for_one_piece != Bitboard(0) {
            let target_pos = target_positions_for_one_piece.pop_lsb_position();
            let (target_piece, _) = board.get_piece_and_color_at_position(target_pos);
            let is_capture = match target_piece {
                Piece::Empty => false,
                _ => true,
            };
            moves.push(ChessMove {
                from: current_pos,
                to: target_pos,
                is_capture: is_capture,
                is_double_move: is_double_move,
                is_promotion: false,
                is_en_passant: false,
                is_castle: false,
                promotion: Piece::Empty,
                captured: target_piece,
            });
        }
    }
    target_positions
}

/// Calculates all possible moves for the king which is the only unique piece
pub fn get_king_moves(
    board: &Board,
    current_pos: Position,
    color: Color,
    moves: &mut Vec<ChessMove>,
    f: fn(board: &Board, pos: Position, color: Color) -> Bitboard,
) -> Bitboard {
    let mut target_positions = f(board, current_pos, color);
    while target_positions != Bitboard(0) {
        let target_pos = target_positions.pop_lsb_position();
        let (target_piece, _) = board.get_piece_and_color_at_position(target_pos);
        let is_capture = match target_piece {
            Piece::Empty => false,
            _ => true,
        };
        moves.push(ChessMove {
            from: current_pos,
            to: target_pos,
            is_capture: is_capture,
            is_double_move: false,
            is_promotion: false,
            is_en_passant: false,
            is_castle: false,
            promotion: Piece::Empty,
            captured: target_piece,
        });
    }
    target_positions
}

pub fn get_castle_moves(board: &Board, color: Color, moves: &mut Vec<ChessMove>) {
    match color {
        Color::White => {
            let mask_white_left = Bitboard(1u64 << 1 | 1u64 << 2 | 1u64 << 3);
            if board.white_castle_left && board.empty_pieces & mask_white_left == mask_white_left {
                let mv = ChessMove {
                    from: Position::from_idx(4),
                    to: Position::from_idx(2),
                    is_capture: false,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: false,
                    is_castle: true,
                    promotion: Piece::Empty,
                    captured: Piece::Empty,
                };
                moves.push(mv);
            }
            let mask_white_right = Bitboard(1u64 << 5 | 1u64 << 6);
            if board.white_castle_right && board.empty_pieces & mask_white_right == mask_white_right
            {
                let mv = ChessMove {
                    from: Position::from_idx(4),
                    to: Position::from_idx(6),
                    is_capture: false,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: false,
                    is_castle: true,
                    promotion: Piece::Empty,
                    captured: Piece::Empty,
                };
                moves.push(mv);
            }
        }
        Color::Black => {
            let mask_black_left = Bitboard(1u64 << 57 | 1u64 << 58 | 1u64 << 59);
            if board.black_castle_left && board.empty_pieces & mask_black_left == mask_black_left {
                let mv = ChessMove {
                    from: Position::from_idx(60),
                    to: Position::from_idx(58),
                    is_capture: false,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: false,
                    is_castle: true,
                    promotion: Piece::Empty,
                    captured: Piece::Empty,
                };
                moves.push(mv);
            }
            let mask_black_right = Bitboard(1u64 << 61 | 1u64 << 62);
            if board.black_castle_right && board.empty_pieces & mask_black_right == mask_black_right
            {
                let mv = ChessMove {
                    from: Position::from_idx(60),
                    to: Position::from_idx(62),
                    is_capture: false,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: false,
                    is_castle: true,
                    promotion: Piece::Empty,
                    captured: Piece::Empty,
                };
                moves.push(mv);
            }
        }
    }
}

pub fn get_promotions_moves(board: &Board, color: Color, moves: &mut Vec<ChessMove>) {
    let (mut piece_positions, y_limit, move_direction) = match color {
        Color::Black => (board.black_pawns, 1, -1),
        Color::White => (board.white_pawns, 6, 1),
    };

    while piece_positions != Bitboard(0) {
        let current_pos = piece_positions.pop_lsb_position();
        // Skip piece if not at y limit
        if current_pos.to_xy().1 != y_limit {
            continue;
        }
        let mv = ChessMove {
            from: current_pos,
            to: current_pos.get_offset_pos(0, move_direction),
            is_capture: false,
            is_double_move: false,
            is_promotion: true,
            is_en_passant: false,
            is_castle: false,
            promotion: Piece::Queen,
            captured: Piece::Empty,
        };
        moves.push(mv);
    }
}

pub fn get_en_passant_moves(board: &Board, color: Color, moves: &mut Vec<ChessMove>) {
    let ep_target = match board.en_passant_target {
        Some(pos) => pos,
        None => return,
    };

    match color {
        Color::White => {
            let position_left = ep_target.get_offset_pos(-1, -1);
            let position_right = ep_target.get_offset_pos(1, -1);
            if board.white_pawns.is_position_set(position_left) {
                let mv = ChessMove {
                    from: position_left,
                    to: ep_target,
                    is_capture: true,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: true,
                    is_castle: false,
                    promotion: Piece::Empty,
                    captured: Piece::Pawn,
                };
                moves.push(mv);
            }
            if board.white_pawns.is_position_set(position_right) {
                let mv = ChessMove {
                    from: position_right,
                    to: ep_target,
                    is_capture: true,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: true,
                    is_castle: false,
                    promotion: Piece::Empty,
                    captured: Piece::Pawn,
                };
                moves.push(mv);
            }
        }
        Color::Black => {
            let position_left = ep_target.get_offset_pos(-1, 1);
            let position_right = ep_target.get_offset_pos(1, 1);
            if board.black_pawns.is_position_set(position_left) {
                let mv = ChessMove {
                    from: position_left,
                    to: ep_target,
                    is_capture: true,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: true,
                    is_castle: false,
                    promotion: Piece::Empty,
                    captured: Piece::Pawn,
                };
                moves.push(mv);
            }
            if board.black_pawns.is_position_set(position_right) {
                let mv = ChessMove {
                    from: position_right,
                    to: ep_target,
                    is_capture: true,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: true,
                    is_castle: false,
                    promotion: Piece::Empty,
                    captured: Piece::Pawn,
                };
                moves.push(mv);
            }
        }
    }
}
