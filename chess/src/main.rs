mod board;
mod pieces;

fn main () {
    let mut chessboard= board::build_board();
    chessboard = board::add_all_pieces(chessboard);
    print!("{:#?}", chessboard);
}