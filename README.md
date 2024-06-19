### Checkers rules:
- 8x8 grid board with black and white tiles;
- 12 black or white pieces for each player; 

#### Win:
- If one player's piece count goes to zero.

#### Piece state: 
- If a normal piece reaches the opposite side, it becomes "king" and has different move options.

#### Move:
- A player can only move pieces forward and diagonally. If the piece is "king", they can go forwards and backwards.

--- 

### Code requirements and logic:
0. Connect two players via TCP/IP and run `start_game()` once they're online;
1. The initialized board should look like this:
```
⬜🔴⬜🔴⬜🔴⬜🔴
🔴⬜🔴⬜🔴⬜🔴⬜
⬜🔴⬜⬛⬜🔴⬜🔴
⬛⬜⬛⬜⬛⬜⬛⬜
⬜⬛⬜⬛⬜⬛⬜⬛
⚪⬜⚪⬜⚪⬜⚪⬜
⬜⚪⬜⚪⬜⚪⬜⚪
⚪⬜⚪⬜⚪⬜⚪⬜
```

2. The board's state data should be stored in a compact data structure to be easily accessible and editable later.
3. 
4. The player can move their pieces by inputting two coordinates: where the piece is and where it goes.
5. The starting coordinate goes through a function named `check_player_move()`to see if the starting coordinate is pointing at the the player's piece. If so, check the end coordinate to see if the move is valid (inside bounds and tile = '⬛').
6. After the move is done, send the data online and update both screens;

7. 
--- 

### Multiplayer Game Structure:
- Send a request for a move and check if it's valid. If it's valid, update the game data and send it to the other player, updating their screen accordingly. If the game state changes or a piece becomes a "king", update the data and iterate again until the game ends;
- Maybe sending a data structure back and forth with information like:
  - Piece position on matrix;
  - Player turn;
  - Game state;
  - Piece hierarchy (normal or "king");

---

### Optional features:
- Randomly decide the game starter;
- Loser picks pieces' color;
- Count how many pieces are left in the board for each player;
- Update the screen by overwriting the previous frame;
- The second player's game should be upside down;
