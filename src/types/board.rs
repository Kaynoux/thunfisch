use crate::{
    prelude::*,
    types::unmake_info::UnmakeInfo,
    utils::zobrist::{self, generate_castling_hash},
};
/// Each piece type gets its own 64bits where
#[derive(Clone)]
pub struct Board {
    color_bbs: [Bitboard; 2],
    occupied: Bitboard,
    figure_bbs: [Bitboard; 13],
    figures: [Figure; 64],
    black_king_castle: bool,
    black_queen_castle: bool,
    white_queen_castle: bool,
    white_king_castle: bool,
    ep_target: Option<Bit>,
    current_color: Color,
    halfmove_clock: usize,
    total_halfmove_counter: usize,
    unmake_info_stack: Vec<UnmakeInfo>,
    hash: u64,
}

impl Board {
    pub const EMPTY: Board = Board {
        color_bbs: [Bitboard(0); 2],
        occupied: Bitboard(0),
        figure_bbs: [Bitboard(0); 13],
        figures: [Figure::Empty; 64],
        black_king_castle: true,
        black_queen_castle: true,
        white_queen_castle: true,
        white_king_castle: true,
        ep_target: None,
        current_color: Color::White,
        halfmove_clock: 0,
        total_halfmove_counter: 0,
        unmake_info_stack: Vec::new(),
        hash: 0,
    };

    #[inline(always)]
    pub fn color_bbs(&self, color: Color) -> Bitboard {
        self.color_bbs[color as usize]
    }

    #[inline(always)]
    pub fn color_bbs_without_king(&self, color: Color) -> Bitboard {
        self.color_bbs(color) & !self.king(color)
    }

    #[inline(always)]
    pub fn empty(&self) -> Bitboard {
        self.figure_bbs[Figure::Empty as usize]
    }

    pub fn occupied(&self) -> Bitboard {
        self.occupied
    }

    pub fn current_color(&self) -> Color {
        self.current_color
    }

    pub fn figures(&self, square: Square) -> Figure {
        self.figures[square]
    }

    pub fn all_figures(&self) -> [Figure; 64] {
        self.figures
    }

    pub fn ep_target(&self) -> Option<Bit> {
        self.ep_target
    }

    pub fn white_queen_castle(&self) -> bool {
        self.white_queen_castle
    }

    pub fn white_king_castle(&self) -> bool {
        self.white_king_castle
    }

    pub fn black_queen_castle(&self) -> bool {
        self.black_queen_castle
    }

    pub fn black_king_castle(&self) -> bool {
        self.black_king_castle
    }

    pub fn halfmove_clock(&self) -> usize {
        self.halfmove_clock
    }

    pub fn total_halfmove_counter(&self) -> usize {
        self.total_halfmove_counter
    }

    pub fn set_halfmove_clock(&mut self, clock: usize) {
        self.halfmove_clock = clock
    }

    pub fn set_total_halfmove_counter(&mut self, counter: usize) {
        self.total_halfmove_counter = counter
    }

    pub fn set_ep_target(&mut self, target: Option<Bit>) {
        self.ep_target = target;
    }

    /// For move unmaking
    pub fn set_castling_rights(
        &mut self,
        white_queen: bool,
        white_king: bool,
        black_queen: bool,
        black_king: bool,
    ) {
        self.hash ^= zobrist::generate_castling_hash(self);

        self.white_queen_castle = white_queen;
        self.white_king_castle = white_king;
        self.black_queen_castle = black_queen;
        self.black_king_castle = black_king;

        self.hash ^= zobrist::generate_castling_hash(self);
    }

    #[inline(always)]
    pub fn piece_and_color_at_position(&self, pos: Bit) -> (Piece, Color) {
        self.figures[pos.to_square()].piece_and_color()
    }

    #[inline(always)]
    pub fn piece_at_position(&self, pos: Square) -> Piece {
        match self.figures[pos.0] {
            Figure::Empty => Empty,
            Figure::WhitePawn => Pawn,
            Figure::BlackPawn => Pawn,
            Figure::WhiteKnight => Knight,
            Figure::BlackKnight => Knight,
            Figure::WhiteBishop => Bishop,
            Figure::BlackBishop => Bishop,
            Figure::WhiteRook => Rook,
            Figure::BlackRook => Rook,
            Figure::WhiteQueen => Queen,
            Figure::BlackQueen => Queen,
            Figure::WhiteKing => King,
            Figure::BlackKing => King,
        }
    }

    #[inline(always)]
    pub const fn figure_bb(&self, color: Color, piece: Piece) -> Bitboard {
        self.figure_bbs[Figure::from_piece_and_color(piece, color) as usize]
    }

    #[inline(always)]
    pub fn king(&self, color: Color) -> Bit {
        match color {
            Black => Bit(self.figure_bbs[Figure::BlackKing as usize].0),
            White => Bit(self.figure_bbs[Figure::WhiteKing as usize].0),
        }
    }

    pub fn toggle_current_color(&mut self) {
        self.current_color = !self.current_color;
        self.hash ^= zobrist::white_move_key()
    }

    pub fn toggle(&mut self, color: Color, piece: Piece, square: Square) {
        let figure = Figure::from_piece_and_color(piece, color);
        self.color_bbs[color as usize].toggle(square);
        self.color_bbs[Figure::Empty as usize].toggle(square);
        self.figure_bbs[figure as usize].toggle(square);
        self.figures[figure as usize] = match self.figures[figure as usize] {
            Figure::Empty => figure,
            _ => Figure::Empty,
        };

        self.hash ^= zobrist::piece_key(color, piece, square)
    }

    pub fn push_unmake_info_stack(&mut self, info: UnmakeInfo) {
        self.unmake_info_stack.push(info);
    }

    pub fn pop_unmake_info_stack(&mut self) -> Option<UnmakeInfo> {
        self.unmake_info_stack.pop()
    }

    pub fn update_castling(
        &mut self,
        color: Color,
        from: Piece,
        mv: &DecodedMove,
        captured: Piece,
    ) {
        if captured != Piece::Rook && from != Piece::Rook && from != Piece::King {
            return; // move not relevant for castling
        }

        self.hash ^= zobrist::generate_castling_hash(self); // remove old castling hash

        const WHITE_ROOK_QUEEN_POS: Square = Square(0);
        const WHITE_ROOK_KING_POS: Square = Square(7);
        const BLACK_ROOK_KING_POS: Square = Square(63);
        const BLACK_ROOK_QUEEN_POS: Square = Square(56);
        match from {
            Piece::King => match color {
                White => {
                    self.white_queen_castle = false;
                    self.white_king_castle = false
                }
                Black => {
                    self.black_queen_castle = false;
                    self.black_king_castle = false
                }
            },
            Piece::Rook => match mv.from {
                WHITE_ROOK_QUEEN_POS => self.white_queen_castle = false,
                WHITE_ROOK_KING_POS => self.white_king_castle = false,
                BLACK_ROOK_QUEEN_POS => self.black_queen_castle = false,
                BLACK_ROOK_KING_POS => self.black_king_castle = false,
                _ => {}
            },
            _ => (),
        }

        if captured == Piece::Rook {
            match mv.to {
                WHITE_ROOK_QUEEN_POS => self.white_queen_castle = false,
                WHITE_ROOK_KING_POS => self.white_king_castle = false,
                BLACK_ROOK_QUEEN_POS => self.black_queen_castle = false,
                BLACK_ROOK_KING_POS => self.black_king_castle = false,
                _ => {}
            }
        }

        self.hash ^= zobrist::generate_castling_hash(self); // add new castling hash
    }
}
