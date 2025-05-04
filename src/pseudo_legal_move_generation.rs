use crate::position_generation::*;
use crate::prelude::*;
use crate::types::encoded_move::move_flags;
#[inline(always)]
pub fn get_all_moves(moves: &mut Vec<EncodedMove>, board: &Board, color: Color) -> Bitboard {
    let mut moves_bitboard = Bitboard(0);
    //let mut moves: Vec<ChessMove> = Vec::with_capacity(256);

    moves_bitboard |= get_pawn_moves(board, color, moves);

    // Knight
    moves_bitboard |=
        get_moves_for_piece_type(board, Piece::Knight, color, moves, get_knight_positions);

    // Bishop
    moves_bitboard |=
        get_moves_for_piece_type(board, Piece::Bishop, color, moves, get_bishop_positions);

    // Rook
    moves_bitboard |=
        get_moves_for_piece_type(board, Piece::Rook, color, moves, get_rook_positions);

    // Queen
    moves_bitboard |=
        get_moves_for_piece_type(board, Piece::Queen, color, moves, get_queen_positions);

    // King
    let king_pos = board.get_king_pos(color);
    if king_pos != Position(0) {
        moves_bitboard |= get_king_moves(board, king_pos, color, moves, get_king_positions);
    }

    get_castle_moves(board, color, moves);
    get_en_passant_moves(board, color, moves);
    moves_bitboard
}

/// Calculate all moves for each instance of a piece type exept the king because it is unique and pawns because they are
#[inline(always)]
pub fn get_moves_for_piece_type(
    board: &Board,
    piece: Piece,
    color: Color,
    moves: &mut Vec<EncodedMove>,
    f: fn(board: &Board, pos: Position, color: Color) -> Bitboard,
) -> Bitboard {
    let mut piece_positions = board.get_positions_by_piece_color(color, piece);
    let mut target_positions = Bitboard(0);

    while piece_positions != Bitboard(0) {
        let current_pos = piece_positions.pop_lsb_position().unwrap();

        let mut target_positions_for_one_piece = f(board, current_pos, color);
        target_positions |= target_positions_for_one_piece;

        while target_positions_for_one_piece != Bitboard(0) {
            let target_pos = target_positions_for_one_piece.pop_lsb_position().unwrap();
            let mv = match target_pos.is_enemy(board, color) {
                true => EncodedMove::encode(current_pos, target_pos, move_flags::CAPTURE),
                false => EncodedMove::encode(current_pos, target_pos, move_flags::QUIET),
            };
            moves.push(mv);
        }
    }
    target_positions
}

#[inline(always)]
pub fn get_pawn_moves(board: &Board, color: Color, moves: &mut Vec<EncodedMove>) -> Bitboard {
    let mut target_positions = Bitboard(0);
    let mut pawn_positions = board.get_positions_by_piece_color(color, Piece::Pawn);

    while pawn_positions != Bitboard(0) {
        let current_pos = pawn_positions.pop_lsb_position().unwrap();

        let mut target_positions_for_one_piece = get_pawn_positions(board, current_pos, color);
        target_positions |= target_positions_for_one_piece;

        while target_positions_for_one_piece != Bitboard(0) {
            let target_pos = target_positions_for_one_piece.pop_lsb_position().unwrap();

            let cy = current_pos.to_y();
            let ty = target_pos.to_y();
            let is_double_move = if cy.abs_diff(ty) == 2 { true } else { false };
            let is_capture = target_pos.is_enemy(board, color);

            if ty == 0 || ty == 7 {
                let promotion_moves = match is_capture {
                    true => [
                        EncodedMove::encode(
                            current_pos,
                            target_pos,
                            move_flags::KNIGHT_PROMO_CAPTURE,
                        ),
                        EncodedMove::encode(
                            current_pos,
                            target_pos,
                            move_flags::BISHOP_PROMO_CAPTURE,
                        ),
                        EncodedMove::encode(
                            current_pos,
                            target_pos,
                            move_flags::ROOK_PROMO_CAPTURE,
                        ),
                        EncodedMove::encode(
                            current_pos,
                            target_pos,
                            move_flags::QUEEN_PROMO_CAPTURE,
                        ),
                    ],

                    false => [
                        EncodedMove::encode(current_pos, target_pos, move_flags::KNIGHT_PROMO),
                        EncodedMove::encode(current_pos, target_pos, move_flags::BISHOP_PROMO),
                        EncodedMove::encode(current_pos, target_pos, move_flags::ROOK_PROMO),
                        EncodedMove::encode(current_pos, target_pos, move_flags::QUEEN_PROMO),
                    ],
                };
                moves.extend_from_slice(&promotion_moves);
            } else {
                let mv = match (is_double_move, is_capture) {
                    (true, false) => {
                        EncodedMove::encode(current_pos, target_pos, move_flags::DOUBLE_MOVE)
                    }
                    (false, true) => {
                        EncodedMove::encode(current_pos, target_pos, move_flags::CAPTURE)
                    }
                    _ => EncodedMove::encode(current_pos, target_pos, move_flags::QUIET),
                };

                moves.push(mv);
            }
        }
    }
    target_positions
}

/// Calculates all possible moves for the king which is the only unique piece
#[inline(always)]
pub fn get_king_moves(
    board: &Board,
    current_pos: Position,
    color: Color,
    moves: &mut Vec<EncodedMove>,
    f: fn(board: &Board, pos: Position, color: Color) -> Bitboard,
) -> Bitboard {
    let mut target_positions = f(board, current_pos, color);
    while target_positions != Bitboard(0) {
        let target_pos = target_positions.pop_lsb_position().unwrap();

        match target_pos.is_enemy(board, color) {
            true => moves.push(EncodedMove::encode(
                current_pos,
                target_pos,
                move_flags::CAPTURE,
            )),
            false => moves.push(EncodedMove::encode(
                current_pos,
                target_pos,
                move_flags::QUIET,
            )),
        }
    }
    target_positions
}

pub fn get_castle_moves(board: &Board, color: Color, moves: &mut Vec<EncodedMove>) {
    match color {
        Color::White => {
            const MASK_WHITE_QUEEN_CASTLE: Bitboard = Bitboard(1u64 << 1 | 1u64 << 2 | 1u64 << 3);
            if board.white_queen_castle
                && board.get_empty_pieces() & MASK_WHITE_QUEEN_CASTLE == MASK_WHITE_QUEEN_CASTLE
            {
                let mv = EncodedMove::encode(
                    IndexPosition(4).to_position(),
                    IndexPosition(2).to_position(),
                    move_flags::QUEEN_CASTLE,
                );
                moves.push(mv);
            }
            const MASK_WHITE_KING_CASTLE: Bitboard = Bitboard(1u64 << 5 | 1u64 << 6);
            if board.white_king_castle
                && board.get_empty_pieces() & MASK_WHITE_KING_CASTLE == MASK_WHITE_KING_CASTLE
            {
                let mv = EncodedMove::encode(
                    IndexPosition(4).to_position(),
                    IndexPosition(6).to_position(),
                    move_flags::KING_CASTLE,
                );
                moves.push(mv);
            }
        }
        Color::Black => {
            const MASK_BLACK_KING_CASTLE: Bitboard = Bitboard(1u64 << 61 | 1u64 << 62);
            if board.black_king_castle
                && board.get_empty_pieces() & MASK_BLACK_KING_CASTLE == MASK_BLACK_KING_CASTLE
            {
                let mv = EncodedMove::encode(
                    IndexPosition(60).to_position(),
                    IndexPosition(62).to_position(),
                    move_flags::KING_CASTLE,
                );
                moves.push(mv);
            }
            const MASK_BLACK_QUEEN_CASTLE: Bitboard =
                Bitboard(1u64 << 57 | 1u64 << 58 | 1u64 << 59);
            if board.black_queen_castle
                && board.get_empty_pieces() & MASK_BLACK_QUEEN_CASTLE == MASK_BLACK_QUEEN_CASTLE
            {
                let mv = EncodedMove::encode(
                    IndexPosition(60).to_position(),
                    IndexPosition(58).to_position(),
                    move_flags::QUEEN_CASTLE,
                );
                moves.push(mv);
            }
        }
    }
}

#[inline(always)]
pub fn get_en_passant_moves(board: &Board, color: Color, moves: &mut Vec<EncodedMove>) {
    let ep_target = match board.ep_target {
        Some(pos) => pos,
        None => return,
    };

    match color {
        Color::White => {
            let position_left = ep_target.get_offset_pos(-1, -1);
            let position_right = ep_target.get_offset_pos(1, -1);
            if board.bbs[ColorPiece::WhitePawn as usize].is_position_set(position_left) {
                let mv = EncodedMove::encode(position_left, ep_target, move_flags::EP_CAPTURE);
                moves.push(mv);
            }
            if board.bbs[ColorPiece::WhitePawn as usize].is_position_set(position_right) {
                let mv = EncodedMove::encode(position_right, ep_target, move_flags::EP_CAPTURE);
                moves.push(mv);
            }
        }
        Color::Black => {
            let position_left = ep_target.get_offset_pos(-1, 1);
            let position_right = ep_target.get_offset_pos(1, 1);
            if board.bbs[ColorPiece::BlackPawn as usize].is_position_set(position_left) {
                let mv = EncodedMove::encode(position_left, ep_target, move_flags::EP_CAPTURE);
                moves.push(mv);
            }
            if board.bbs[ColorPiece::BlackPawn as usize].is_position_set(position_right) {
                let mv = EncodedMove::encode(position_right, ep_target, move_flags::EP_CAPTURE);
                moves.push(mv);
            }
        }
    }
}

#[inline(always)]
pub fn get_all_attacks(board: &Board, color: Color) -> Bitboard {
    let mut attacks = Bitboard(0);
    let mut positions = match color {
        Color::Black => board.black_pieces,
        Color::White => board.white_pieces,
    };
    while positions != Bitboard(0) {
        let current_pos = positions.pop_lsb_position().unwrap();
        let (piece, color) = board.get_piece_and_color_at_position(current_pos);
        attacks |= match piece {
            Piece::Pawn => get_pawn_attack_positions(board, current_pos, color),
            Piece::Knight => get_knight_positions(board, current_pos, color),
            Piece::Bishop => get_bishop_positions(board, current_pos, color),
            Piece::Rook => get_rook_positions(board, current_pos, color),
            Piece::Queen => get_queen_positions(board, current_pos, color),
            Piece::King => get_king_positions(board, current_pos, color),
            _ => Bitboard(0),
        };
    }
    attacks
}
