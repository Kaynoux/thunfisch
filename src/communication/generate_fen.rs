use crate::prelude::*;
impl Board {
    pub fn generate_fen(&self) -> String {
        let mut fen = String::new();
        for y in (0..=7).rev() {
            let mut empty_counter = 0;
            for x in 0..=7 {
                let (piece, color) = self.piece_and_color_at_position(Bit::from_xy(x, y));
                if piece == Empty && x == 7 {
                    empty_counter += 1;
                    fen.push_str(&empty_counter.to_string());
                    empty_counter = 0;
                } else if piece == Empty {
                    empty_counter += 1;
                } else {
                    if empty_counter != 0 {
                        fen.push_str(&empty_counter.to_string());
                        empty_counter = 0;
                    }

                    let c = piece.to_fin_char(color);
                    fen.push(c);
                }
            }
            fen.push('/');
        }
        // Remove last /
        fen.pop();
        fen.push(' ');
        let color_char = match self.current_color() {
            White => 'w',
            Black => 'b',
        };
        fen.push(color_char);
        fen.push(' ');
        if self.white_king_castle {
            fen.push('K');
        }
        if self.white_queen_castle {
            fen.push('Q');
        }
        if self.black_queen_castle {
            fen.push('k');
        }
        if self.black_king_castle {
            fen.push('q');
        }

        if !self.black_king_castle
            && !self.black_queen_castle
            && !self.white_queen_castle
            && !self.white_king_castle
        {
            fen.push('-');
        }
        fen.push(' ');

        if let Some(ep) = self.ep_target {
            fen.push_str(&ep.to_coords());
        } else {
            fen.push('-');
        };
        fen.push(' ');

        fen.push_str(&self.halfmove_clock.to_string());
        fen.push(' ');

        fen.push_str(&self.total_halfmove_counter.to_string());

        fen
    }
}
