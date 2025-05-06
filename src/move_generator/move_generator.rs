use std::pin;

use rayon::iter::MultiZip;

use crate::move_generator::blockers;
use crate::move_generator::lookup_tables::ray::{RAY_BETWEEN, RAY_DIRECTION};
use crate::move_generator::magic_array;
use crate::move_generator::magics;
use crate::move_generator::normal_targets;
use crate::move_generator::shifts;
use crate::move_generator::sliding_targets;
use crate::prelude::*;

use super::normal_targets::KNIGHT_TARGETS;

pub const fn get_rook_targets(pos: Square, occ: Bitboard) -> Bitboard {
    let blockers = Bitboard(blockers::ROOK[pos.0].0 & occ.0); // Blocked by other pieces or default blockers
    let magic = magics::ROOK[pos.0]; // use generated magic number
    let shift = shifts::ROOK[pos.0]; // use generated shift
    let offset = magic_array::ROOK[pos.0]; // use generated offset
    let hash_idx = (((blockers.0.wrapping_mul(magic)) >> shift) as usize) + offset; // calculate the hash_idx 
    magic_array::TABLE[hash_idx]
}

pub const fn get_bishop_targets(pos: Square, occ: Bitboard) -> Bitboard {
    let blockers = Bitboard(blockers::BISHOP[pos.0].0 & occ.0); // Blocked by other pieces or default blockers
    let magic = magics::BISHOP[pos.0]; // use generated magic number
    let shift = shifts::BISHOP[pos.0]; // use generated shift
    let offset = magic_array::BISHOP[pos.0]; // use generated offset
    let hash_idx = (((blockers.0.wrapping_mul(magic)) >> shift) as usize) + offset; // calculate the hash_idx 
    magic_array::TABLE[hash_idx]
}

impl Board {
    pub fn generate_all_moves(&self) -> Vec<EncodedMove> {
        let mut moves: Vec<EncodedMove> = Vec::new();
        if self.is_in_check() {
            println!("info: not in check");
            //self.generate_evasion_moves(&mut moves);
        } else {
            self.generate_capture_moves(&mut moves);
            //self.generate_quiet_moves(&mut moves);
        }
        moves
    }

    fn generate_evasion_moves(&self, moves: Vec<EncodedMove>) {
        let king_pos = self.get_king_bit(self.current_color);
    }

    pub fn generate_capture_moves(&self, moves: &mut Vec<EncodedMove>) {
        let current = self.current_color;
        let enemy_bb = self.get_enemy_pieces(current);
        let pin_masks = self.generate_pin_masks2();
        // for (idx, i) in pin_masks.iter().enumerate() {
        //     println!("{} {:?}", idx, self.pieces[idx]);
        //     println!("{:?}", i);
        // }
        println!(" OCCU {:?}", self.occupied);

        for bit in self
            .get_bitboard_by_piece_color(current, Piece::Knight)
            .iter_mut()
        {
            let from = bit.to_square();
            if self.pieces[from.i()] == Figure::WhiteKnight {
                let pin_mask = pin_masks[from.i()];
                println!("{:?}", pin_mask);
                if pin_mask != Bitboard(0) {
                    continue;
                }
                let potential_targets = KNIGHT_TARGETS[from.i()];
                let mut targets = potential_targets & enemy_bb;

                for to_bit in targets.iter_mut() {
                    let to = to_bit.to_square();

                    moves.push(EncodedMove::encode(from, to, MoveType::Capture));
                }
            }
        }
    }

    pub fn generate_pin_masks(&self) -> [Bitboard; 64] {
        let mut masks = [Bitboard(0); 64];
        let friendly_color = self.current_color;
        let enemy_color = !friendly_color;
        let king_bit = self.get_king_bit(friendly_color);
        let king_sq = king_bit.to_square();
        let occ = self.occupied;
        // queen is technically a rook and a bishop so its positon is added to both
        let enemy_rooks_queens = self.get_bitboard_by_piece_color(enemy_color, Piece::Rook)
            | self.get_bitboard_by_piece_color(enemy_color, Piece::Queen);
        let enemy_bishops_queens = self.get_bitboard_by_piece_color(enemy_color, Piece::Bishop)
            | self.get_bitboard_by_piece_color(enemy_color, Piece::Queen);

        for dir in 0..8usize {
            let ray_from_king = RAY_DIRECTION[king_sq.i()][dir];
            let mut pieces_on_ray = occ & ray_from_king;
            if pieces_on_ray.is_empty() {
                continue;
            }
            let pinned = pieces_on_ray.get_next_by_dir(dir);

            // only friendly piece can be pinned, so skip if enemy
            if pinned.is_enemy(self, friendly_color) {
                continue;
            }

            pieces_on_ray &= !pinned;

            if pieces_on_ray.is_empty() {
                continue;
            }

            let pinning = pieces_on_ray.get_next_by_dir(dir);
            let is_rook_dir = if dir % 2 == 0 { true } else { false };

            let is_pinner = if is_rook_dir {
                enemy_rooks_queens.is_position_set(pinning)
            } else {
                enemy_bishops_queens.is_position_set(pinning)
            };
            if !is_pinner {
                continue;
            }

            masks[pinned.to_square().i()] = ray_from_king;
        }
        masks
    }

    pub fn generate_pin_masks2(&self) -> [Bitboard; 64] {
        let mut masks = [Bitboard(0); 64];
        let friendly_color = self.current_color;
        let enemy_color = !friendly_color;
        let king_bit = self.get_king_bit(friendly_color);
        let king_sq = king_bit.to_square();
        let occ = self.occupied;
        const NORTH: usize = 0;
        const NORTHEAST: usize = 1;
        const EAST: usize = 2;
        const SOUTHEAST: usize = 3;
        const SOUTH: usize = 4;
        const SOUTHWEST: usize = 5;
        const WEST: usize = 6;
        const NORTHWEST: usize = 7;
        let enemy_rooks_queens = self.get_bitboard_by_piece_color(enemy_color, Piece::Rook)
            | self.get_bitboard_by_piece_color(enemy_color, Piece::Queen);
        let enemy_bishops_queens = self.get_bitboard_by_piece_color(enemy_color, Piece::Bishop)
            | self.get_bitboard_by_piece_color(enemy_color, Piece::Queen);

        // Iteriere über die 8 Richtungen vom König aus
        let mut dir_idx: usize = 0;
        while dir_idx < 8 {
            // 1. Finde alle besetzten Felder auf dem Strahl vom König aus
            let ray = RAY_DIRECTION[king_bit.to_square().i()][dir_idx];
            let occupied_on_ray = ray & occ;

            // 2. Prüfe, ob mindestens 2 Figuren auf dem Strahl sind
            if occupied_on_ray.0.count_ones() >= 2 {
                // 3. Finde die Figur, die dem König am nächsten ist (potenziell gefesselt)
                //    LSB für positive Richtungen, MSB für negative Richtungen
                let first_blocker_sq = if dir_idx < 4 {
                    // N, NE, E, SE
                    Square(occupied_on_ray.0.trailing_zeros() as usize)
                } else {
                    // S, SW, W, NW
                    Square(63 - occupied_on_ray.0.leading_zeros() as usize)
                };

                // 4. Prüfe, ob diese nächste Figur zur 'pinned_side' gehört

                if occ.is_position_set(first_blocker_sq.to_bit()) {
                    // 5. Finde die Figur(en) *hinter* dem ersten Blocker
                    let behind_first = RAY_BETWEEN[first_blocker_sq.0][dir_idx] & occ;

                    // 6. Prüfe, ob genau EINE Figur dahinter ist UND ob diese ein relevanter gegnerischer Slider ist

                    // Mindestens eine Figur dahinter
                    // Finde die Figur, die dem ersten Blocker am nächsten ist (der Pinner)
                    let pinner_sq = if dir_idx < 4 {
                        Square(behind_first.0.trailing_zeros() as usize)
                    } else {
                        Square(63 - behind_first.0.leading_zeros() as usize)
                    };

                    // Prüfe, ob der Pinner ein relevanter gegnerischer Slider ist
                    let pinner_bb = pinner_sq.to_bitboard();
                    let is_relevant_pinner = match dir_idx {
                        NORTH | EAST | SOUTH | WEST => {
                            !((enemy_rooks_queens & pinner_bb).is_empty())
                        } // Orthogonal
                        NORTHEAST | SOUTHEAST | SOUTHWEST | NORTHWEST => {
                            !((enemy_bishops_queens & pinner_bb).is_empty())
                        } // Diagonal
                        _ => false, // Sollte nicht passieren
                    };

                    if is_relevant_pinner {
                        // Pin gefunden! Setze die Maske für die gefesselte Figur
                        // Erlaubt sind Züge ZWISCHEN König und Pinner, PLUS der Pinner selbst
                        masks[first_blocker_sq.0] =
                            RAY_BETWEEN[king_bit.to_square().i()][pinner_sq.i()] | pinner_bb;
                    }
                }
            }
            dir_idx += 1;
        }
        masks
    }
}
