use std::io;

fn display_board(board: [[char; 8]; 8]) {
  for i in 0..8 {
    for j in 0..8 {
      print!("{}", board[i][j]);
    }
    println!();
  }
}

fn move_piece(mut board: [[char; 8]; 8]) {

}

fn main() {
  let mut board = [['⬛'; 8]; 8];

  for i in 0..8 {
    for j in 0..8 {
      if i % 2 == 0 && j % 2 == 0 || i % 2 != 0 && j % 2 != 0 {
        board[i][j] = '⬜';
      }
    }
  }

  for i in 0..3 {
    for j in 0..8 {
      if i % 2 == 0 && j % 2 != 0 || i % 2 != 0 && j % 2 == 0 {
        board[i][j] = '🔴';
      }
    }
  }

  for i in 5..8 {
    for j in 0..8 {
      if i % 2 == 0 && j % 2 != 0 || i % 2 != 0 && j % 2 == 0 {
        board[i][j] = '⚪';
      }
    }
  }

  // display the board
  // prompt the user to enter the move
  // move the piece
  // send the updated board data to the other screen
}
