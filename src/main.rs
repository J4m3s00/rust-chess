use std::{io, fmt::write};

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

---------------------------------
| 21| 22| 23| 24| 25| 26| 27| 28|       
---------------------------------           BISHOP: 7, 9, -7, -9
| 13| 14| 15| 16| 17| 18| 19| 20|           ROOK: 1, 8, -1, -8
---------------------------------           QUEEN: 1, 8, -1, -8, 7, 9, -7, -9
| 5 | 6 | 7 | 8 | 9 | 10| 11| 12|           KNIGHT: 17, 10, 15, 6, 1, -6, -15, -10
---------------------------------           KING: 1, 8, -1, -8, 7, 9, -7, -9
| -3| -2| -1| 0 | 1 | 2 | 3 | 4 |
---------------------------------
|-11|-10| -9| -8| -7| -6| -5| -4|
---------------------------------
|-19|-18|-17|-16|-15|-14|-13|-12|
---------------------------------
|-27|-26|-25|-24|-23|-22|-21|-20|
---------------------------------
|-35|-34|-33|-32|-31|-30|-29|-28|
---------------------------------
 */

#[derive(Copy, Clone, PartialEq)]
pub enum Team {
    White,
    Black
}

mod Casteling {
    pub const WHITE_KING_SIDE: u8 = 1;
    pub const WHITE_QUEEN_SIDE: u8 = 2;
    pub const BLACK_KING_SIDE: u8 = 4;
    pub const BLACK_QUEEN_SIDE: u8 = 8;

    pub const ALL: u8 = WHITE_KING_SIDE | WHITE_QUEEN_SIDE | BLACK_KING_SIDE | BLACK_QUEEN_SIDE;
}



pub struct GameState {
    board: Board,
    turn: Team, // True = white, false = black
    en_passent: Option<u8>,
    castle_state: u8,
    moves: Vec<Move>
}

impl GameState {
    pub const fn new() -> GameState {
        GameState { board: Board::new(), turn: Team::White, en_passent: None, castle_state: Casteling::ALL, moves: Vec::new() }
    }

    pub fn from_fen(fen: &str) -> GameState {
        let mut board = Board::new();
        let mut turn = Team::White;
        let mut en_passent = None;
        let mut castle_state = Casteling::ALL;
        let mut moves = Vec::new();

        let mut fen_iter = fen.split(' ');
        let board_fen = fen_iter.next().unwrap();
        let turn_fen = fen_iter.next().unwrap();
        let castle_fen = fen_iter.next().unwrap();
        let en_passent_fen = fen_iter.next().unwrap();

        let mut board_fen_iter = board_fen.split('/');

        for y in 0..8 {
            let board_fen_row = board_fen_iter.next().unwrap();
            let mut x = 0;
            for c in board_fen_row.chars() {
                if c.is_digit(10) {
                    x += c.to_digit(10).unwrap() as usize;
                } else {
                    let piece_type = match c {
                        'P' => PieceType::Pawn,
                        'N' => PieceType::Knight,
                        'B' => PieceType::Bishop,
                        'R' => PieceType::Rook,
                        'Q' => PieceType::Queen,
                        'K' => PieceType::King,
                        'p' => PieceType::Pawn,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'r' => PieceType::Rook,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        _ => panic!("Invalid character in fen string")
                    };
                    let color = if c.is_ascii_uppercase() { Team::White } else { Team::Black };
                    let position = position_from_xy(x as u8, 7 - y as u8);
                    board.squares[position as usize] = Some(Piece { piece_type, color, position });
                    x += 1;
                }
            }
        }

        turn = if turn_fen == "w" { Team::White } else { Team::Black };

        castle_state = 0;
        if castle_fen.contains('K') { castle_state |= Casteling::WHITE_KING_SIDE; }
        if castle_fen.contains('Q') { castle_state |= Casteling::WHITE_QUEEN_SIDE; }
        if castle_fen.contains('k') { castle_state |= Casteling::BLACK_KING_SIDE; }
        if castle_fen.contains('q') { castle_state |= Casteling::BLACK_QUEEN_SIDE; }

        if en_passent_fen != "-" {
            en_passent = Some(position_from_xy(en_passent_fen.chars().nth(0).unwrap() as u8 - 'a' as u8, en_passent_fen.chars().nth(1).unwrap() as u8 - '1' as u8));
        }

        GameState {
            board,
            turn,
            en_passent,
            castle_state,
            moves
        }
    }

    pub fn make_move(&mut self, m: Move) {
        let square_to_move = self.board.squares[m.from as usize];
        if let Some(mut square) = square_to_move {
            // Check if valid move
            let moves = square.get_possible_moves(self);
            if moves.iter().find(|fm| { fm.to == m.to }).is_some() {
                // It is a valid move
                square.position = m.to;

                self.board.squares[m.from as usize] = None;
                self.board.squares[m.to as usize] = Some(square);
                return;
            }
        }

        println!("Could not make the move!");
    }

    pub fn get_possible_moves(&self, team : Team) -> Vec<Move> {
        let mut moves : Vec<Move> = Vec::new();
        for square in self.board.squares {
            if let Some(square) = square {
                if square.color == team {
                    let mut piece_moves = square.get_possible_moves(self);
                    moves.append(&mut piece_moves);
                }
            }
        }
        return moves;
    }
}

#[derive(Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    // The piece can be white (true) or black (false)
    color: Team,
    // The piece can be positioned on the board
    position: u8
}

impl Piece {
    fn get_char(&self) -> char {
        match self.piece_type {
            PieceType::Pawn => if self.color == Team::White { 'P' } else { 'p' },
            PieceType::Knight => if self.color == Team::White { 'N' } else { 'n' },
            PieceType::Bishop => if self.color == Team::White { 'B' } else { 'b' },
            PieceType::Rook => if self.color == Team::White { 'R' } else { 'r' },
            PieceType::Queen => if self.color == Team::White { 'Q' } else { 'q' },
            PieceType::King => if self.color == Team::White { 'K' } else { 'k' },
        }
    }


    fn get_possible_moves(&self, game_state: &GameState) -> Vec<Move> {
        let mut moves = Vec::new();
        let pos = position_to_xy(self.position);

        match self.piece_type {
            PieceType::Pawn => {
                let first_move = if self.color == Team::White { pos.1 == 1 } else { pos.1 == 6 };
                let next_one_y = if self.color == Team::White { pos.1 + 1 } else { pos.1 - 1 };
                let next_two_y = if self.color == Team::White { pos.1 + 2 } else { pos.1 - 2 };

                if game_state.board.get_piece(position_from_xy(pos.0, next_one_y)).is_none() {
                    moves.push(Move::new(self.position, position_from_xy(pos.0, next_one_y)));
                    if first_move && game_state.board.get_piece(position_from_xy(pos.0, next_two_y)).is_none()
                    {
                        moves.push(Move::new(self.position, position_from_xy(pos.0, next_two_y)));
                    }
                }
            },
            PieceType::Knight => {
                let move_index = [(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)];
                for m in move_index.iter() {
                    let new_pos = (pos.0 as i8 + m.0, pos.1 as i8 + m.1);
                    if position_xy_inside_s(new_pos.0, new_pos.1) {
                        let piece = game_state.board.get_piece(position_from_xy(new_pos.0 as u8, new_pos.1 as u8));
                        if piece.is_none() || piece.unwrap().color != self.color {
                            moves.push(Move::new(self.position, position_from_xy(new_pos.0 as u8, new_pos.1 as u8)));
                        }
                    }
                }
            },
            PieceType::Bishop => {
                for i in 0..4 {
                    let current_offset = match i {
                        0 => (1, 1),
                        1 => (1, -1),
                        2 => (-1, 1),
                        3 => (-1, -1),
                        _ => (0, 0)
                    };
                    let mut pos = (pos.0 as i8, pos.1 as i8);
                    loop {
                        pos = (pos.0 as i8 + current_offset.0, pos.1 as i8 + current_offset.1);
                        if position_xy_inside_s(pos.0, pos.1) {
                            let piece = game_state.board.get_piece(position_from_xy(pos.0 as u8, pos.1 as u8));
                            if piece.is_none() || piece.unwrap().color != self.color {
                                moves.push(Move::new(self.position, position_from_xy(pos.0 as u8, pos.1 as u8)));
                            }
                            if piece.is_some() {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            },
            PieceType::Rook => {
                for i in 0..4 {
                    let current_offset = match i {
                        0 => (1, 0),
                        1 => (-1, 0),
                        2 => (0, 1),
                        3 => (0, -1),
                        _ => (0, 0)
                    };
                    let mut pos = (pos.0 as i8, pos.1 as i8);
                    loop {
                        pos = (pos.0 as i8 + current_offset.0, pos.1 as i8 + current_offset.1);
                        if position_xy_inside_s(pos.0, pos.1) {
                            let piece = game_state.board.get_piece(position_from_xy(pos.0 as u8, pos.1 as u8));
                            if piece.is_none() || piece.unwrap().color != self.color {
                                moves.push(Move::new(self.position, position_from_xy(pos.0 as u8, pos.1 as u8)));
                            }
                            if piece.is_some() {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            },
            PieceType::Queen => {
                for i in 0..8 {
                    let current_offset = match i {
                        0 => (1, 1),
                        1 => (1, -1),
                        2 => (-1, 1),
                        3 => (-1, -1),
                        4 => (1, 0),
                        5 => (-1, 0),
                        6 => (0, 1),
                        7 => (0, -1),
                        _ => (0, 0)
                    };
                    let mut pos = (pos.0 as i8, pos.1 as i8);
                    loop {
                        pos = (pos.0 as i8 + current_offset.0, pos.1 as i8 + current_offset.1);
                        if position_xy_inside_s(pos.0, pos.1) {
                            let piece = game_state.board.get_piece(position_from_xy(pos.0 as u8, pos.1 as u8));
                            if piece.is_none() || piece.unwrap().color != self.color {
                                moves.push(Move::new(self.position, position_from_xy(pos.0 as u8, pos.1 as u8)));
                            }
                            if piece.is_some() {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            },
            PieceType::King => {
                for i in 0..8 {
                    let current_offset = match i {
                        0 => (1, 1),
                        1 => (1, -1),
                        2 => (-1, 1),
                        3 => (-1, -1),
                        4 => (1, 0),
                        5 => (-1, 0),
                        6 => (0, 1),
                        7 => (0, -1),
                        _ => (0, 0)
                    };
                    let pos = (pos.0 as i8 + current_offset.0, pos.1 as i8 + current_offset.1);
                    if position_xy_inside_s(pos.0, pos.1) {
                        let piece = game_state.board.get_piece(position_from_xy(pos.0 as u8, pos.1 as u8));
                        if piece.is_none() || piece.unwrap().color != self.color {
                            moves.push(Move::new(self.position, position_from_xy(pos.0 as u8, pos.1 as u8)));
                        }
                    }
                }
            },
        }

        return moves;
    }
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


#[derive(Copy, Clone)]
// The chess board
struct Board {
    // The board is a 2D array of 64 squares
    squares: [Option<Piece>; 8 * 8]
}

impl Board {

    pub const fn new() -> Board {
        return Board {
            squares: [None; 8 * 8]
        };
    }
    
    fn get_piece(&self, position: u8) -> Option<Piece> {
        assert!(position < 64);
        if let Some(piece) = &self.squares[position as usize] {
            return Some(piece.clone());
        }
        return None;
    }

    fn print_custom(&self, square_callback: &dyn Fn(u8, Option<Piece>) -> char) {
        println!("---------------------------------");
        for i in 0 .. 8 {
            print!("|");
            for x in 0 .. 8 {
                let y = 7 - i;
                print!(" {} ", square_callback(position_from_xy(x as u8, y as u8), self.squares[y * 8 + x]));
                print!("|");
            }
    
            println!("");
            println!("---------------------------------");
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "---------------------------------")?;
        for i in 0 .. 8 { // We are looping wrong way around
            write!(f, "|")?;
            for x in 0 .. 8 {
                let y = 7 - i;
                if let Some(piece) = self.squares[y * 8 + x] {
                    // write the piece. Use upper case for white and lower case for black
                    write!(f, " {} ", piece.get_char())?;
                } else {
                    write!(f, "   ")?;
                }
                write!(f, "|")?;
            }
    
            writeln!(f, "")?;
            writeln!(f, "---------------------------------")?;
        }
        return Ok(());
    }
}


fn position_from_xy(x: u8, y: u8) -> u8 {
    assert!(x < 8);
    assert!(y < 8);
    return x + y * 8;
}

fn position_to_xy(position: u8) -> (u8, u8) {
    assert!(position < 64);
    return (position % 8, position / 8);
}

fn position_xy_inside_s(x: i8, y: i8) -> bool {
    return x >= 0 && x < 8 && y >= 0 && y < 8;
}

fn position_xy_inside_u(x: u8, y: u8) -> bool {
    return x < 8 && y < 8;
}


fn get_position_from_string(position: &str) -> u8 {
    assert!(position.len() == 2);
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
pub struct Move {
    from: u8,
    to: u8,
    move_type: MoveType,
}

impl Move {
    pub fn new(from: u8, to: u8) -> Move {
        return Move {from, to, ..Default::default()};
    }
}

enum InputMessage {
    Move(Move),
    ShowMoves(u8),
    ShowTeam(Team),
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
        
        let from = get_position_from_string(input[1]);
        let to = get_position_from_string(input[2]);
        return InputMessage::Move(Move {from, to, ..Default::default()});
    } else if input[0] == "s" {
        if input.len() != 2 {
            return InputMessage::ShowBoard;
        }
        if input[1].to_uppercase() == "BLACK" || input[1].to_uppercase() == "WHITE" {
            let team = if input[1].to_uppercase() == "BLACK" { Team::Black } else { Team::White };
            return InputMessage::ShowTeam(team);
        } else {
            let position = get_position_from_string(input[1]);
            return InputMessage::ShowMoves(position);
        }
    }
    return InputMessage::None;
}

fn print_possible_moves(board: &Board, moves: &Vec<Move>) {
    println!("Showing moves {:?}", moves);
    let custom_print = |pos: u8, square: Option<Piece>|{
        for m in moves {
            // If the current move is the current position
            if m.to ==  pos {
                // Print the move
                return 'X';
            }
        }
        if let Some(piece) = square {
            return piece.get_char();
        }
        
        return ' ';
    };
    board.print_custom(&custom_print);
}

fn main() {
    let mut game_state = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    

    println!("{}", game_state.board);

    loop {
        let input = get_input();
        match input {
            InputMessage::Move(m) => {
                game_state.make_move(m);
                println!("{}", game_state.board);
            },
            InputMessage::ShowMoves(pos) => {
                if let Some(piece) = game_state.board.get_piece(pos) {
                    let moves = piece.get_possible_moves(&game_state);
                    print_possible_moves(&game_state.board, &moves);
                }
            },
            InputMessage::ShowTeam(team) => {
                let moves = game_state.get_possible_moves(team);
                print_possible_moves(&game_state.board, &moves);
            },
            InputMessage::ShowBoard => {
                println!("{}", game_state.board);
            },
            InputMessage::None => { },
            InputMessage::Quit => {
                break;
            },
        }
    }
}