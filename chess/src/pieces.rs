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

pub fn remove_piece(mut arraycontent: Option<Piece>) -> Option<Piece> {
    arraycontent = None;
    arraycontent
}

pub fn validate_move(figure: Figure,orx: usize, ory: usize, x: usize, y: usize) -> bool {
    if (x < 0) || (x > 7) || (y < 0) || (y > 7) {
        println!("coordinates out of board range");
        return false;
    }
    
    let difx = orx.abs_diff(x);
    let dify = ory.abs_diff(y);
    
    // check possible moves one piece after another
    // Pawn
    match figure {
        Figure::Pawn=> println!("test"),
        Figure::Rook => if (difx > 0 && dify == 0) || (difx == 0 && dify > 0) {
            return true;
        } else {
            return false;
        },
        Figure::Knight => if (difx == 2 && dify == 1) || (difx == 1 && dify == 2) {
            return true;
        } else {
            return false;
        },
        Figure::Bishop => if difx == dify {
            return true;
        } else {
            return false;
        },
        Figure::Queen => if (difx == dify) || (difx > 0 && dify == 0) || (difx == 0 && dify > 0) {
            return true;
        } else {
            return false;
        },
        Figure::King => if difx <= 1 && dify <= 1 {
            return true;
        } else {
            return false;
        },
    }

    false
}