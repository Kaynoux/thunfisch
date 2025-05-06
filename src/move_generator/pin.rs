use super::*;
use const_for::*;

/// This gets a pin mask between the king and the sliding piece if relevant
/// Use (king_sq * 2048) + (slider_sq * 256) + (pexed occupancies along the axis wanted)
pub const PIN_MASKS: [u64; 16384] = generate_pin_masks();

const fn generate_pin_masks() -> [u64; 16384] {
    let mut masks: [u64; 16384] = [0; 16384];

    const_for!(king_pos in 0..8 => {
        const_for!(square in 0..8 => {
            const_for!(occ in 0..256 => {
                let mut mask = 0;

                let mut found_either: bool = false;
                let mut between = 0;

                if occ & 1 << square == 0 {
                    continue
                }

                let start = if king_pos < square {
                    king_pos
                } else {
                    0
                };

                const_for!(i in start..8 => {
                    if i == king_pos {
                        // Found king
                        if found_either {
                            break;
                        }

                        found_either = true;
                    } else if i == square {
                        // Found slider
                        mask |= 1 << i;

                        if found_either {
                            break;
                        }

                        found_either = true;
                    } else if occ & 1 << i != 0 && found_either {
                        // Found piece between
                        mask |= 1 << i;
                        between += 1;
                    } else if found_either {
                        // Found empty between
                        mask |= 1 << i;
                    }
                });

                if between == 1 {
                    masks[2048*king_pos   +   256*square   +    occ] = mask
                }
            });
        });
    });

    masks
}
