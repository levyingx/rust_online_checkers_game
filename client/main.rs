use std::io::{self, Read, Write};
use std::net::TcpStream;

fn display_turn(stream: &mut TcpStream) -> io::Result<String> {
    let mut turn_buffer = [0; 1024]; // read and display player turn
    if let Ok(bytes_read) = stream.read(&mut turn_buffer) {
        if bytes_read > 0 {
            let turn_str = String::from_utf8_lossy(&turn_buffer[..bytes_read]).trim().to_string();
            
            let turn = turn_str.clone(); // clone string to return it
            if turn == "Player 1" {
                println!("\x1b[33mYour\x1b[0m turn\n");
            } else {
                println!("\x1b[33m{}\x1b[0m's turn\n", turn);
            }
            return Ok(turn); // return turn
        }
    }
    Ok(String::new()) 
}

fn display_board(stream: &mut TcpStream, board: &mut [[char; 8]; 8]) -> io::Result<()> {
    let mut buffer = [0; 64]; // board data from server
    match stream.read(&mut buffer) { // read board from server
        Ok(_) => {
            for i in 0..8 {
                for j in 0..8 {
                    board[i][j] = buffer[i * 8 + j] as char; // fill board
                }
            }
        },
        Err(e) => return Err(e),
    }
    for i in 0..8 { // convert char to emoji
        for j in 0..8 {
            let piece = match board[i][j] {
                'B' => '⬛',
                'W' => '⬜',
                '1' => '⚪',
                '2' => '🔴',
                'K' => '🔘',
                'Q' => '🟠',
                _ => '🟦', // error tile
            };
            print!("{}", piece);
        }
        println!();
    }
    Ok(())
}

fn read_move_start_xy() -> (usize, usize) {
    let mut piece_move = String::new();
   
    println!("Insert start_x: ");
    io::stdin().read_line(&mut piece_move).expect("Failed to read move");
    let start_x = piece_move.trim().parse().expect("Failed to parse move");
    piece_move.clear();
    println!("Insert start_y: ");
    io::stdin().read_line(&mut piece_move).expect("Failed to read move");
    let start_y = piece_move.trim().parse().expect("Failed to parse move");
    piece_move.clear();
    return (start_x, start_y);
}

fn read_move_end_xy() -> (usize, usize) {
    let mut piece_move = String::new();
   
    println!("Insert end_x: ");
    io::stdin().read_line(&mut piece_move).expect("Failed to read move");
    let end_x = piece_move.trim().parse().expect("Failed to parse move");
    piece_move.clear();
    println!("Insert end_y: ");
    io::stdin().read_line(&mut piece_move).expect("Failed to read move");
    let end_y = piece_move.trim().parse().expect("Failed to parse move");
    piece_move.clear();
    return (end_x, end_y);
}

fn main() -> io::Result<()> {
    const ADDRESS: &str = "127.0.0.1:8080";
    let mut board = [[' '; 8]; 8];

    match TcpStream::connect(ADDRESS) {
        Ok(mut stream) => { // connect to server
            loop {
                display_board(&mut stream, &mut board)?; // read and display board
                let turn = display_turn(&mut stream)?; // show player turn and capture it
                if turn == "Player 1" {    
                    let start_moves = read_move_start_xy(); // read and send start_x and start_y first
                    let move_str = format!("{},{}", start_moves.0, start_moves.1); 
                    stream.write_all(move_str.as_bytes()).expect("Failed to send start move"); 
                    
                    let end_moves = read_move_end_xy(); // and then read and send end_x and end_y 
                    let move_str = format!("{},{}", end_moves.0, end_moves.1); 
                    stream.write_all(move_str.as_bytes()).expect("Failed to send end move");
                } else {
                    // wait for player 2 to move
                    println!("Waiting for player 2 to move...");
                }
            }
        },
        Err(_) => {
            println!("Couldn't connect to the server");
        }
    }
    Ok(())
}
