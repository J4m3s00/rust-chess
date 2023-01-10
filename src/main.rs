use std::io;

/**
 * 
---------------------------------
| 56| 57| 58| 59| 60| 61| 62| 63|
---------------------------------
| 48| 49| 50| 51| 52| 53| 54| 55|
---------------------------------
| 40| 41| 42| 43| 44| 45| 46| 47|
---------------------------------
| 32| 33| 34| 35| 36| 37| 38| 39|
---------------------------------
| 24| 25| 26| 27| 28| 29| 30| 31|
---------------------------------
| 16| 17| 18| 19| 20| 21| 22| 23|
---------------------------------
| 8 | 9 | 10| 11| 12| 13| 14| 15|
---------------------------------
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
---------------------------------
 */

#[derive(Copy, Clone, PartialEq)]
enum Team {
    White,
    Black
}

pub struct GameState {
    board: Board,
    turn: Team, // True = white, false = black
    moves: Vec<Move>
}

pub static mut GAME_STATE: GameState = GameState { board: init_board(), turn: Team::White, moves: Vec::new() };

fn game_make_move()
{
    
}



#[derive(Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    // The piece can be white (true) or black (false)
    color: Team,
    // The piece can be positioned on the board
    position: u8
}


#[derive(Copy, Clone)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}


#[derive(Copy, Clone, Default)]
struct Square {
    // The square can be empty or contain a piece
    piece: Option<Piece>
}
// The chess board
struct Board {
    // The board is a 2D array of 64 squares
    squares: [Square; 8 * 8]
}

fn piece_get_char(piece: Piece) -> char {
    match piece.piece_type {
        PieceType::Pawn => if piece.color == Team::White { 'P' } else { 'p' },
        PieceType::Knight => if piece.color == Team::White { 'N' } else { 'n' },
        PieceType::Bishop => if piece.color == Team::White { 'B' } else { 'b' },
        PieceType::Rook => if piece.color == Team::White { 'R' } else { 'r' },
        PieceType::Queen => if piece.color == Team::White { 'Q' } else { 'q' },
        PieceType::King => if piece.color == Team::White { 'K' } else { 'k' },
    }
}

fn print_board(board: &Board) {
    // Print the board
    println!("---------------------------------");
    for i in 0 .. 8 { // We are looping wrong way around
        print!("|");
        for x in 0 .. 8 {
            let y = 7 - i;
            if let Some(piece) = board.squares[y * 8 + x].piece {
                // Print the piece. Use upper case for white and lower case for black
                print!(" {} ", piece_get_char(piece));
            } else {
                print!("   ");
            }
            print!("|");
        }

        println!("");
        println!("---------------------------------");
    }
}

const fn init_board() -> Board
{
    return Board { squares: ([
        Square {piece: Some(Piece { piece_type: PieceType::Rook, color: Team::White, position: 56 })}, Square {piece: Some(Piece { piece_type: PieceType::Knight, color: Team::White, position: 57 })}, Square {piece: Some(Piece { piece_type: PieceType::Bishop, color: Team::White, position: 58 })}, Square {piece: Some(Piece { piece_type: PieceType::Queen, color: Team::White, position: 59 })}, Square {piece: Some(Piece { piece_type: PieceType::King, color: Team::White, position: 60 })}, Square {piece: Some(Piece { piece_type: PieceType::Bishop, color: Team::White, position: 61 })}, Square {piece: Some(Piece { piece_type: PieceType::Knight, color: Team::White, position: 62 })}, Square {piece: Some(Piece { piece_type: PieceType::Rook, color: Team::White, position: 63 })},
        Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::White, position: 48 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::White, position: 49 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::White, position: 50 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::White, position: 51 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::White, position: 52 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::White, position: 53 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::White, position: 54 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::White, position: 55 })},
        Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None},
        Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None},
        Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None},
        Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None}, Square {piece: None}, Square { piece: None},
        Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::Black, position: 8 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::Black, position: 9 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::Black, position: 10 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::Black, position: 11 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::Black, position: 12 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::Black, position: 13 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::Black, position: 14 })}, Square {piece: Some(Piece { piece_type: PieceType::Pawn, color: Team::Black, position: 15 })},
        Square {piece: Some(Piece { piece_type: PieceType::Rook, color: Team::Black, position: 0 })}, Square {piece: Some(Piece { piece_type: PieceType::Knight, color: Team::Black, position: 1 })}, Square {piece: Some(Piece { piece_type: PieceType::Bishop, color: Team::Black, position: 2 })}, Square {piece: Some(Piece { piece_type: PieceType::Queen, color: Team::Black, position: 3 })}, Square {piece: Some(Piece { piece_type: PieceType::King, color: Team::Black, position: 4 })}, Square {piece: Some(Piece { piece_type: PieceType::Bishop, color: Team::Black, position: 5 })}, Square {piece: Some(Piece { piece_type: PieceType::Knight, color: Team::Black, position: 6 })}, Square {piece: Some(Piece { piece_type: PieceType::Rook, color: Team::Black, position: 7 })},
    ]) };
}

fn board_move(board: &mut Board, from: u8, to: u8) {
    assert!(from < 64);
    assert!(to < 64);
    
    // Move a piece from one square to another
    board.squares[to as usize].piece = board.squares[from as usize].piece;
    if let Some(piece) = &mut board.squares[to as usize].piece {
        piece.position = to;
    }
    board.squares[from as usize].piece = None;
    println!("Moved from {:?} to {:?}", from, to);
}

fn board_get_piece(board: &Board, position: u8) -> Option<Piece> {
    assert!(position < 64);
    if let Some(piece) = &board.squares[position as usize].piece {
        return Some(piece.clone());
    }
    return None;
}

fn board_get_piece_coord(board: &Board, position: (u8, u8)) ->  Option<Piece> {
    assert!(position.0 < 8);
    assert!(position.1 < 8);
    let position = position.0 + position.1 * 8;

    return board_get_piece(board, position);
}

fn board_get_position_from_string(position: &str) -> u8 {
    let position_chars: Vec<char> = position.to_uppercase().chars().collect();
    let x = position_chars[0] as u8 - 'A' as u8;
    let y = position_chars[1] as u8 - '1' as u8;

    println!("Get Position {}: {}, {}", position, x, y);

    return x + y * 8;
}


#[derive(Default, Debug)]
enum MoveType {
    #[default] Quite,
    Capture,
    EnPassant,
    KingCastle,
    QueenCastle,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
}

#[derive(Default, Debug)]
struct Move {
    from: u8,
    to: u8,
    move_type: MoveType,
}

enum InputMessage {
    Move(Move),
    ShowMoves(u8),
    ShowBoard,
    Quit,
    None
}

fn get_input() -> InputMessage {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    if input == "quit" {
        return InputMessage::Quit;
    }
    if input.len() == 0 {
        return InputMessage::Quit;
    }

    let input: Vec<&str> = input.split(" ").collect();
    if input[0] == "m" {
        if input.len() != 3 {
            return InputMessage::None;
        }
        
        let from = board_get_position_from_string(input[1]);
        let to = board_get_position_from_string(input[2]);
        return InputMessage::Move(Move {from, to, ..Default::default()});
    } else if input[0] == "s" {
        if input.len() != 2 {
            return InputMessage::None;
        }
        let position = board_get_position_from_string(input[1]);
        return InputMessage::ShowMoves(position);
    } else if input[0] == "sb" {
        return InputMessage::ShowBoard;
    }
    return InputMessage::None;
}

fn piece_get_possible_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let mut moves = Vec::new();
    
    return moves;
}

fn print_possible_moves(board: &Board, piece: &Piece) {
    let moves = piece_get_possible_moves(board, piece);
    println!("Showing moves {:?}", moves);
    println!("---------------------------------");
    for i in 0..8 { // Rows
        // Loop 8 times
        print!("|");
        for j in 0..8 { // Columns
            // Loop through all possible moves
            //let y = 7 - i;
            let position = i * 8 + j;
            if piece.position == position {
                print!(" {} ", piece_get_char(*piece));
            } else {
                let mut found = false;
                for m in &moves {
                    // If the current move is the current position
                    if m.to == position {
                        // Print the move
                        print!(" X ");
                        found = true;
                        break;
                    }
                }
                if !found {
                    print!("   ");
                }
            }
            print!("|");
        }
        println!("");
        println!("---------------------------------");
    }
}

fn main() {
    let mut board = init_board();

    print_board(&board);
    loop {
        let input = get_input();
        match input {
            InputMessage::Move(m) => {
                board_move(&mut board, m.from, m.to);
                print_board(&board);
            },
            InputMessage::ShowMoves(pos) => {
                if let Some(piece) = board_get_piece(&board, pos) {
                    print_possible_moves(&board, &piece);
                }
            },
            InputMessage::ShowBoard => {
                print_board(&board);
            },
            InputMessage::None => { },
            InputMessage::Quit => {
                break;
            },
        }
    }
}