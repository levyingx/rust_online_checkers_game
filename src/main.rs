// emojis: ⚪🔴⬛⬜

use std::io;

fn init_board(board: &mut [[char; 8]; 8]) -> &[[char; 8]; 8] {
  for i in 0..8 {
    for j in 0..8 {
      if i % 2 == 0 && j % 2 == 0 || i % 2 != 0 && j % 2 != 0 {
        board[i][j] = '⬜';
      } else {
        board[i][j] = '⬛';
      }
    }
  }

  for i in 0..3 { // board red disc init
    for j in 0..8 {
      if i % 2 == 0 && j % 2 != 0 || i % 2 != 0 && j % 2 == 0 {
        board[i][j] = '🔴';
      } 
    }
  }

  for i in 5..8 { // board white discs init
    for j in 0..8 {
      if i % 2 == 0 && j % 2 != 0 || i % 2 != 0 && j % 2 == 0 {
        board[i][j] = '⚪';
      } 
    }
  }

  return board;
}

fn display_board(board: [[char; 8]; 8]) {
  for i in 0..8 {
    for j in 0..8 {
      print!("{}", board[i][j]);
    }
    println!();
  }
}

fn move_piece() {
  
}

fn main() {
  let mut board = [['⬛'; 8]; 8]; 
  let mut start_x = String::new();
  let mut start_y = String::new();
  let mut end_x = String::new();
  let mut end_y = String::new();
  
  init_board(&mut board); // init board
  display_board(board); // display board 

  println!("Start (x):");
  io::stdin().read_line(&mut start_x).unwrap();
  start_x = start_x.trim().parse().unwrap();
  println!("Start (y):");
  io::stdin().read_line(&mut start_y).unwrap();
  println!("End (x):");
  io::stdin().read_line(&mut end_x).unwrap();
  println!("End (y):");
  io::stdin().read_line(&mut end_y).unwrap();

}
