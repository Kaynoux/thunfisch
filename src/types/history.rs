use crate::{
    prelude::*,
    settings::*,
};

// don't think that should be tweaked tbh so I'll not put it in settings.rs
const MAX_HISTORY_VALUE: i16 = i16::MAX;

/// vectors are two-dimensional arrays indexed by [from_square][to_square]
pub struct HistoryBoard([[i16; 64]; 64], [[i16; 64]; 64]);

impl HistoryBoard {
    /// Update the history value for `mv` at `depth` for `color` by `bonus`.
    /// The bonus should be calculated by either history_bonus for bonuses, and
    /// history_maluse for history maluse punishments.
    pub fn update_history(&mut self, color: Color, mv: DecodedMove, depth: usize, bonus: i16) {
        let fro = mv.from.0;
        let to = mv.to.0;
        // let bonus = history_bonus(depth);
        match color {
            White => self.0[fro][to] = gravity(self.0[fro][to], bonus),
            Black => self.1[fro][to] = gravity(self.0[fro][to], bonus),
        }
    }


    /// TODO: The Relative History paper suggests this "aging" of histories,
    /// however I have not found such a practice in both the CPW and viridithas so
    /// it may not be necessary
    pub fn age_histories(&mut self) {
        self.0
            .iter_mut()
            .for_each(|inner| inner.iter_mut().for_each(|h| *h = *h / 2));
        self.1
            .iter_mut()
            .for_each(|inner| inner.iter_mut().for_each(|h| *h = *h / 2));
    }

    pub fn reset_histories(&mut self) {
        // SAFETY: Both arrays are fixed-size [i32; 64] x 64, so raw zeroing is fine
        // The pointers are valid and properly aligned. Should be faster than iteration.
        #[allow(clippy::ptr_as_ptr)]
        unsafe {
            std::ptr::write_bytes(self.0.as_mut_ptr() as *mut i32, 0, 64 * 64);
            std::ptr::write_bytes(self.1.as_mut_ptr() as *mut i32, 0, 64 * 64);
        }
    }
}

#[inline]
fn gravity(val: i16, bonus: i16) -> i16 {
    i16::clamp(
        val + bonus - val * bonus.abs() / MAX_HISTORY_VALUE,
        -MAX_HISTORY_VALUE,
        MAX_HISTORY_VALUE,
    )
}

#[inline]
pub fn history_bonus(depth: usize) -> i16 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    i16::min(
        HISTORY_BONUS_MUL * depth as i16 + HISTORY_BONUS_OFFS,
        HISTORY_BONUS_MAX,
    )
}

#[inline]
pub fn history_maluse(depth: usize) -> i16 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    -i16::min(
        HISTORY_MALUSE_MUL * depth as i16 + HISTORY_MALUSE_OFFS,
        HISTORY_MALUSE_MAX,
    )
}
