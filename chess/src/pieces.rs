#[derive(Copy)]
enum Figures {
    Pawn,
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
}

#[derive(Copy)]
enum Players {
    Player1,
    Player2,
}

#[derive(Copy)]
pub struct Pieces {
    kind: Figures,
    posx: u8,
    posy: u8,
    player: Players,
}