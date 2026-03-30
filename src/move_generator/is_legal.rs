use crate::{
    move_generator::{
        masks::{self, calculate_attackmask},
        normal_targets::{
            KING_TARGETS, KNIGHT_TARGETS, PAWN_ATTACK_TARGETS, pawn_quiet_double_target,
            pawn_quiet_single_target,
        },
        pinmask,
        sliding_targets::{get_bishop_targets, get_rook_targets},
    },
    prelude::*,
};

impl Board {
    /// Checks whether `mv` is legal on `self`.
    /// - [x] null moves
    /// - [x] no piece
    /// - [x] Piece pinned
    /// - [x] Checks
    /// - [x] Target square available
    /// - [ ] castling
    /// TODO: Collapse to gigantic branchless boolean statement ahahahaha
    pub fn is_legal(&self, mv: &DecodedMove) -> bool {
        let to = mv.to.to_bit();
        // Catch null moves
        if mv.from == mv.to {
            return false;
        }

        // is there even a piece?
        let from_figure = self.figures(mv.from);
        if from_figure == Figure::Empty {
            return false;
        }

        // Do we capture the king?
        if self.figures(mv.to).piece() == Piece::King {
            return false;
        }

        // Is the piece pinned?
        let (hv_pinmask, diag_pinmask) = pinmask::generate_pin_masks(self);
        let pinmask = hv_pinmask | diag_pinmask;


        // probably missing an edge case where we're double pinned and move from one pin line to another one
        if pinmask.is_position_set(mv.from.to_bit()) && !pinmask.is_position_set(to) {
            return false;
        }

        let (check_mask, check_counter) = masks::calc_check_mask(self);
        // Does the king have to move because of double check?
        if check_counter >= 2 && from_figure.piece() != Piece::King {
            return false;
        }
        // Does the move block the check?
        if from_figure.piece() != Piece::King && !check_mask.is_empty() && !check_mask.is_position_set(to) {
            return false;
        }

        // Is the piece generally allowed to move there?
        let opponents = self.color_bbs_without_king(!self.current_color());

        let can_move_there = match from_figure.piece() {
            Pawn => match mv.mv_type {
                MoveType::DoubleMove => {
                    pawn_quiet_double_target(mv.from.to_bit(), self.current_color()) == to
                        && self
                            .empty()
                            .is_position_set(pawn_quiet_single_target(mv.from.to_bit(), White))
                        && self
                            .empty()
                            .is_position_set(pawn_quiet_single_target(mv.from.to_bit(), Black))
                }
                MoveType::Quiet
                | MoveType::KnightPromo
                | MoveType::BishopPromo
                | MoveType::RookPromo
                | MoveType::QueenPromo => {
                    pawn_quiet_single_target(mv.from.to_bit(), self.current_color()) == to
                }
                MoveType::Capture
                | MoveType::KnightPromoCapture
                | MoveType::BishopPromoCapture
                | MoveType::RookPromoCapture
                | MoveType::QueenPromoCapture => {
                    PAWN_ATTACK_TARGETS[0][mv.from.i()].is_position_set(to)
                        && opponents.is_position_set(to)
                        || PAWN_ATTACK_TARGETS[1][mv.from.i()].is_position_set(to)
                            && opponents.is_position_set(to)
                },
                MoveType::EpCapture => true,
                _ => false,
            },
            Knight => KNIGHT_TARGETS[mv.from.i()].is_position_set(to),
            Bishop => (get_bishop_targets(mv.from, self.occupied()) & (opponents | self.empty()))
                .is_position_set(to),
            Rook => (get_rook_targets(mv.from, self.occupied()) & (opponents | self.empty()))
                .is_position_set(to),
            Queen => ((get_rook_targets(mv.from, self.occupied()) & (opponents | self.empty()))
                | (get_bishop_targets(mv.from, self.occupied() & (opponents | self.empty()))))
            .is_position_set(to),
            King => {
                KING_TARGETS[mv.from.i()].is_position_set(to)
                    && !calculate_attackmask(self, self.occupied(), !self.current_color(), None)
                        .is_position_set(to)
            }
            Empty => false,
        };

        if !can_move_there {
            return false;
        }

        true
    }
}
