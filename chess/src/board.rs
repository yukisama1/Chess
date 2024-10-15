use crate::pieces;

pub fn build_board() -> [[Option<pieces::Piece>; 8]; 8] {
      let mut board = [[None; 8]; 8];

      board
} 

pub fn add_all_pieces(mut board: [[Option<pieces::Piece>; 8]; 8]) -> [[Option<pieces::Piece>; 8]; 8] {
      let count = 0;
      for y in 0..8 {
            for x in 0..8 {
                  if y > 1 && y < 6 {
                        continue;
                  } else if y <= 1 {
                        board[y][x] = Some(pieces::new_piece(pieces::Figure::Pawn, x, y, pieces::Player::Player1));
                  } else if y >= 6 {
                        board[y][x] = Some(pieces::new_piece(pieces::Figure::Pawn, x, y, pieces::Player::Player2));
                  }
            }
      }

      // board[0][0] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Rook);
      // board[0][1] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Knight);
      // board[0][2] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Bishop);
      // board[0][3] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Queen);
      // board[0][4] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::King);
      // board[0][5] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Bishop);
      // board[0][6] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Knight);
      // board[0][7] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Rook);
      // board[7][0] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Rook);
      // board[7][1] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Knight);
      // board[7][2] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Bishop);
      // board[7][3] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Queen);
      // board[7][4] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::King);
      // board[7][5] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Bishop);
      // board[7][6] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Knight);
      // board[7][7] = pieces::set_piece(board[0][0].unwrap(), pieces::Figure::Rook);

      board
}