use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Figure {
    Pawn,
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
}

#[derive(Copy, Clone, Debug)]
pub enum Player {
    Player1,
    Player2,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    kind: Figure,
    posx: usize,
    posy: usize,
    player: Player,
}

pub fn new_piece(p: Figure, x: usize, y: usize, pl: Player) -> Piece {
    let piece = Piece {
        kind: p,
        posx: x,
        posy: y,
        player: pl,
    };

    piece
}

pub fn set_piece(mut piece: Piece, newfigure: Figure) -> Option<Piece> {
    piece.kind = newfigure;
    Some(piece)
}