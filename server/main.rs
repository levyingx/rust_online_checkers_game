use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

// client functions

fn display_local_board_(board: &mut [[char; 8]; 8]) -> io::Result<()> {
    for i in 0..8 {
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

// local functions

fn init_board() -> [[char; 8]; 8] {
    let mut board = [[' '; 8]; 8];
    for i in 0..8 { // checkerboard pattern
        for j in 0..8 {
            if i % 2 == 0 && j % 2 == 0 || i % 2 != 0 && j % 2 != 0 {
                board[i][j] = 'W';
            } else {
                board[i][j] = 'B';
            }
        }
    }

    for i in 0..3 { // init pieces
        for j in 0..8 {
            if i % 2 == 0 && j % 2 != 0 || i % 2 != 0 && j % 2 == 0 {
                board[i][j] = '2';
            }
        }
    }

    for i in 5..8 {
        for j in 0..8 {
            if i % 2 == 0 && j % 2 != 0 || i % 2 != 0 && j % 2 == 0 {
                board[i][j] = '1';
            }
        }
    }

    board
}

fn send_board(mut stream: &TcpStream, board: &[[char; 8]; 8]) -> std::io::Result<()> {
    let mut buffer = [0; 64];
    for i in 0..8 {
        for j in 0..8 {
            buffer[i * 8 + j] = board[i][j] as u8;
        }
    }
    stream.write_all(&buffer)?;
    Ok(())
}

fn send_data_to_client(mut stream: &TcpStream, data: &str) -> std::io::Result<()> {
    let data_bytes = data.as_bytes();
    stream.write_all(data_bytes)?;
    Ok(())
}

fn move_piece(board: &mut [[char; 8]; 8], start_x: usize, start_y: usize, end_x: usize, end_y: usize, player_piece: char) {
    board[end_x][end_y] = player_piece; // take piece from start and put it in end
    board[start_x][start_y] = if (start_x % 2 == 0 && start_y % 2 != 0) || (start_x % 2 != 0 && start_y % 2 == 0) {
        'B'
    } else {
        'W'
    };

    // capture piece
    if (end_x as isize - start_x as isize).abs() == 2 && (end_y as isize - start_y as isize).abs() == 2 {
        let capture_x = (start_x + end_x) / 2;
        let capture_y = (start_y + end_y) / 2;
        board[capture_x][capture_y] = 'B'; // Remove the captured piece
    }

    if player_piece == '1' && end_x == 0 { // crown piece
        board[end_x][end_y] = 'K'; 
    } else if player_piece == '2' && end_x == 7 {
        board[end_x][end_y] = 'Q'; 
    }
}

fn get_possible_moves(board: &[[char; 8]; 8], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let piece = board[x][y];

    let is_king = piece == 'K' || piece == 'Q';

    if piece == '1' || is_king {
        if x > 0 && y > 0 && board[x - 1][y - 1] == 'B' {
            moves.push((x - 1, y - 1)); // move up left
        }
        if x > 0 && y < 7 && board[x - 1][y + 1] == 'B' {
            moves.push((x - 1, y + 1)); // move up right
        }
        if x > 1 && y > 1 && board[x - 1][y - 1] == '2' && board[x - 2][y - 2] == 'B' {
            moves.push((x - 2, y - 2)); // move up & capture left
        }
        if x > 1 && y < 6 && board[x - 1][y + 1] == '2' && board[x - 2][y + 2] == 'B' {
            moves.push((x - 2, y + 2)); // move up & capture right
        }
        if is_king {
            if x < 7 && y > 0 && board[x + 1][y - 1] == 'B' {
                moves.push((x + 1, y - 1)); // move down left
            }
            if x < 7 && y < 7 && board[x + 1][y + 1] == 'B' {
                moves.push((x + 1, y + 1)); // move down right
            }
            if x < 6 && y > 1 && board[x + 1][y - 1] == '2' && board[x + 2][y - 2] == 'B' {
                moves.push((x + 2, y - 2)); // move down & capture left
            }
            if x < 6 && y < 6 && board[x + 1][y + 1] == '2' && board[x + 2][y + 2] == 'B' {
                moves.push((x + 2, y + 2)); // move down & capture right
            }
        }
    } else if piece == '2' || is_king {
        if x < 7 && y > 0 && board[x + 1][y - 1] == 'B' {
            moves.push((x + 1, y - 1)); // move down left
        }
        if x < 7 && y < 7 && board[x + 1][y + 1] == 'B' {
            moves.push((x + 1, y + 1)); // move down right
        }
        if x < 6 && y > 1 && board[x + 1][y - 1] == '1' && board[x + 2][y - 2] == 'B' {
            moves.push((x + 2, y - 2)); // move down & capture left
        }
        if x < 6 && y < 6 && board[x + 1][y + 1] == '1' && board[x + 2][y + 2] == 'B' {
            moves.push((x + 2, y + 2)); // move down & capture right
        }
        if is_king {
            if x > 0 && y > 0 && board[x - 1][y - 1] == 'B' {
                moves.push((x - 1, y - 1)); // move up left
            }
            if x > 0 && y < 7 && board[x - 1][y + 1] == 'B' {
                moves.push((x - 1, y + 1)); // move up right
            }
            if x > 1 && y > 1 && board[x - 1][y - 1] == '1' && board[x - 2][y - 2] == 'B' {
                moves.push((x - 2, y - 2)); // move up & capture left
            }
            if x > 1 && y < 6 && board[x - 1][y + 1] == '1' && board[x - 2][y + 2] == 'B' {
                moves.push((x - 2, y + 2)); // move up & capture right
            }
        }
    }
    moves
}

fn check_win(board: &[[char; 8]; 8]) -> Option<&str> {
    let mut player1_pieces = 0;
    let mut player2_pieces = 0;

    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == '1' || board[i][j] == 'K' {
                player1_pieces += 1;
            } else if board[i][j] == '2' || board[i][j] == 'Q' {
                player2_pieces += 1;
            }
        }
    }

    if player1_pieces == 0 {
        return Some("Player 2");
    } else if player2_pieces == 0 {
        return Some("Player 1");
    }
    None
}

fn main() -> std::io::Result<()> {
    const ADDRESS: &str = "127.0.0.1:8080";
    let listener = TcpListener::bind(ADDRESS)?;

    let mut board = init_board();
    let mut turn = "Player 1";
    let mut client_start_input = [0; 1024];
    let mut client_end_input = [0; 1024];
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;

    for stream in listener.incoming() { // accept incoming connections
        match stream { 
            Ok(mut stream) => {
                loop {
                    display_local_board_(&mut board)?; // display board here
                    send_board(&stream, &board)?; // send board to client
                    if let Some(winner) = check_win(&board) { // check win
                        println!("{} wins!", winner);
                        send_data_to_client(&stream, winner)?; // tell client someone won
                        break;
                    }

                    if turn == "Player 1" {
                        println!("\x1b[33mYour\x1b[0m turn\n");
                    } else {
                        println!("\x1b[33mPlayer 2\x1b[0m's turn\n");
                    } // display turn here
                     
                    send_data_to_client(&stream, turn)?; // send turn to client

                    if turn == "Player 1" { // read from client first  
                        match stream.read(&mut client_start_input) { // read start_x and start_y from client
                            Ok(bytes_read) => {
                                let input_str = String::from_utf8_lossy(&client_start_input[..bytes_read]).trim().to_string();
                                let parts: Vec<&str> = input_str.split(',').collect();

                                if parts.len() == 2 {
                                    start_x = parts[0].parse().unwrap_or(0);
                                    start_y = parts[1].parse().unwrap_or(0);
                                    possible_moves = get_possible_moves(&mut board, start_x, start_y); // get possible moves for start_x and start_y
                                } else {
                                    eprintln!("Error: Invalid move format");
                                    send_data_to_client(&stream, "Invalid move format")?;
                                    continue;
                                }
                            }
                            Err(_) => {
                                eprintln!("\x1b[31mUnable to read from client\x1b[0m");
                            }
                        }
                        
                        match stream.read(&mut client_end_input) {
                            Ok(bytes_read) => {
                                let input_str = String::from_utf8_lossy(&client_end_input[..bytes_read]).trim().to_string();
                                let parts: Vec<&str> = input_str.split(',').collect();
                                let end_x = parts[0].parse().unwrap_or(0);
                                let end_y = parts[1].parse().unwrap_or(0);
                                let player_piece = board[start_x][start_y];

                                if possible_moves.contains(&(end_x, end_y)) {
                                    println!("\x1b[32m{} sent end_x and end_y\x1b[0m", turn);
                                    move_piece(&mut board, start_x, start_y, end_x, end_y, player_piece);
                                    println!("\x1b[32m{} made a move\x1b[0m", turn);
                                    turn = "Player 2";
                                    
                                } else {
                                    eprintln!("Error: Not a possible move");
                                    continue;
                                }
                            }
                            Err(_) => {
                                eprintln!("\x1b[31mUnable to read from client\x1b[0m");
                            }
                        }
                    } else { // local stuff (player 2) 
                        let start_moves = read_move_start_xy(); // read and send start_x and start_y first 
                        let player_piece = board[start_moves.0][start_moves.1];
                        
                        possible_moves = get_possible_moves(&mut board, start_moves.0, start_moves.1); // get possible moves 
                        println!("Possible moves: {:?}", possible_moves);  
                        println!("\x1b[32m{} sent start_x and start_y\x1b[0m", turn);
                        
                        let end_moves = read_move_end_xy(); // read end_x and end_y 

                        if possible_moves.contains(&(end_moves.0, end_moves.1)) {
                            println!("\x1b[32m{} sent end_x and end_y\x1b[0m", turn);
                            move_piece(&mut board, start_moves.0, start_moves.1, end_moves.0, end_moves.1, player_piece);
                            println!("\x1b[32m{} made a move\x1b[0m", turn);
                            turn = "Player 1";
                        } else {
                            eprintln!("Error: Not a possible move");
                            continue;
                        }
                    }
                }
            }
            Err(_) => {
                eprintln!("\x1b[31mFailed to establish connection\x1b[0m");
            }
        }
    }
    Ok(())
}