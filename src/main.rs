use std::{io::{self, stdin, Read}, time::Instant};


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

impl Team {
    pub fn other(&self) -> Team {
        match self {
            Team::White => Team::Black,
            Team::Black => Team::White
        }
    }
}

mod casteling {
    pub const WHITE_KING_SIDE: u8 = 1;
    pub const WHITE_QUEEN_SIDE: u8 = 2;
    pub const BLACK_KING_SIDE: u8 = 4;
    pub const BLACK_QUEEN_SIDE: u8 = 8;

    pub const ALL: u8 = WHITE_KING_SIDE | WHITE_QUEEN_SIDE | BLACK_KING_SIDE | BLACK_QUEEN_SIDE;
}


#[derive(Default, Clone, Debug)]
pub struct GameStateData {
    en_passent: Option<u8>,
    castle_state: u8,
    captured_piece: Option<PieceType>,
}


pub struct GameState {
    board: Board,
    turn: Team, // True = white, false = black
    state: GameStateData,
    state_stack: Vec<GameStateData>,
    move_stack: Vec<Move>,
}

impl GameState {
    pub const fn new() -> GameState {
        GameState { board: Board::new(), turn: Team::White, state: GameStateData { en_passent: None, castle_state: casteling::ALL, captured_piece: None}, state_stack: Vec::new(), move_stack: Vec::new() }
    }

    pub fn from_fen(fen: &str) -> GameState {
        let mut board = Board::new();
        let mut en_passent = None;

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

        let mut castle_state = 0;
        if castle_fen.contains('K') { castle_state |= casteling::WHITE_KING_SIDE; }
        if castle_fen.contains('Q') { castle_state |= casteling::WHITE_QUEEN_SIDE; }
        if castle_fen.contains('k') { castle_state |= casteling::BLACK_KING_SIDE; }
        if castle_fen.contains('q') { castle_state |= casteling::BLACK_QUEEN_SIDE; }

        if en_passent_fen != "-" {
            en_passent = Some(position_from_xy(en_passent_fen.chars().nth(0).unwrap() as u8 - 'a' as u8, en_passent_fen.chars().nth(1).unwrap() as u8 - '1' as u8));
        }

        GameState {
            board,
            turn: if turn_fen == "w" { Team::White } else { Team::Black },
            state: GameStateData {
                en_passent,
                castle_state,
                captured_piece: None,
            },
            state_stack: Default::default(),
            move_stack: Default::default(),
        }
    }

    pub fn to_fen_str(&self) -> String {
        let mut fen = String::new();
        for y in 0..8 {
            let mut empty_count = 0;
            for x in 0..8 {
                let position = position_from_xy(x, 7 - y);
                let square = self.board.squares[position as usize];
                if square.is_none() {
                    empty_count += 1;
                } else {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    let piece = square.unwrap();
                    let piece_char = match piece.piece_type {
                        PieceType::Pawn => 'P',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Rook => 'R',
                        PieceType::Queen => 'Q',
                        PieceType::King => 'K',
                    };
                    fen.push(if piece.color == Team::White { piece_char } else { piece_char.to_ascii_lowercase() });
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            if y < 7 {
                fen.push('/');
            }
        }
        fen.push(' ');
        fen.push(if self.turn == Team::White { 'w' } else { 'b' });
        fen.push(' ');
        if self.state.castle_state == 0 {
            fen.push('-');
        } else {
            if self.state.castle_state & casteling::WHITE_KING_SIDE != 0 { fen.push('K'); }
            if self.state.castle_state & casteling::WHITE_QUEEN_SIDE != 0 { fen.push('Q'); }
            if self.state.castle_state & casteling::BLACK_KING_SIDE != 0 { fen.push('k'); }
            if self.state.castle_state & casteling::BLACK_QUEEN_SIDE != 0 { fen.push('q'); }
        }
        fen.push(' ');
        if self.state.en_passent.is_none() {
            fen.push('-');
        } else {
            let position = self.state.en_passent.unwrap();
            let x = position % 8;
            let y = position / 8;
            fen.push((x as u8 + 'a' as u8) as char);
            fen.push((y as u8 + '1' as u8) as char);
        }

        fen.push_str(" 0 1");

        fen
    }

    pub fn make_move(&mut self, m: Move) {
        let square_to_move = self.board.squares[m.from as usize];
        if square_to_move.is_none() {
            println!("No piece to move!");
            return;
        }
        let piece_to_move = square_to_move.unwrap();
        if piece_to_move.color != self.turn { 
            println!("The piece you want to move is not the correct team!");
            return;
        }
        let piece_possible_moves = piece_to_move.get_possible_moves(self);
        let current_found_move_opt = piece_possible_moves.into_iter().find(|fm| {fm.to == m.to});
        if current_found_move_opt.is_none() {
            println!("The move is not possible with current position on the board! {:?}\nFen:{}", m, self.to_fen_str());
            let _ = stdin().read(&mut [0u8]).unwrap();
            return;
        }

        // We have a valid move!
        self.state_stack.push(self.state.clone());
        self.move_stack.push(m);

        let current_pos = position_to_xy(piece_to_move.position);
        let current_move = current_found_move_opt.unwrap();


        

        // Update casteling state
        let mut to_remove_castle_state = 0;
        match piece_to_move.piece_type {
            PieceType::King => {
                match piece_to_move.color {
                    Team::White => { to_remove_castle_state = casteling::WHITE_KING_SIDE | casteling::WHITE_QUEEN_SIDE;},
                    Team::Black => { to_remove_castle_state = casteling::BLACK_KING_SIDE | casteling::BLACK_QUEEN_SIDE;}
                }
            },
            PieceType::Rook => {
                let king_side = current_pos.0 == 7;
                to_remove_castle_state = if king_side {
                    if let Team::White = piece_to_move.color {
                        casteling::WHITE_KING_SIDE
                    } else {
                        casteling::BLACK_KING_SIDE
                    }
                } else {
                    if let Team::White = piece_to_move.color {
                        casteling::WHITE_QUEEN_SIDE
                    } else {
                        casteling::BLACK_QUEEN_SIDE
                    }
                }
            },
            _ => ()
        }
        self.state.castle_state -= to_remove_castle_state;

        let casteling_rook = if let MoveType::KingCastle = current_move.move_type {
            if let Team::White = piece_to_move.color {
                self.board.get_piece(7)
            } else {
                self.board.get_piece(63)
            }
        } else if let MoveType::QueenCastle = current_move.move_type {
            if let Team::White = piece_to_move.color {
                self.board.get_piece(0)
            } else {
                self.board.get_piece(56)
            }
        } else { None };

        if casteling_rook.is_some() && current_move.move_type.is_castle() {
            let rook_pos = get_rook_castle_position(piece_to_move.color, MoveType::KingCastle == current_move.move_type);
            self.board.move_piece(Move::new(casteling_rook.unwrap().position, rook_pos));
        }

        if let MoveType::EnPassantCapture = current_move.move_type {
            let capture_pos = if let Team::White = piece_to_move.color { current_move.to - 8 } else { current_move.to + 8 };
            self.board.squares[capture_pos as usize] = None;
        }

        if let MoveType::Capture = current_move.move_type {
            if let Some(captured) = self.board.squares[current_move.to as usize] {
                self.state.captured_piece = Some(captured.piece_type);
            }
        }

        if current_move.move_type == MoveType::DoublePawnPush {
            self.state.en_passent = if piece_to_move.color == Team::White { Some(current_move.from + 8) } else { Some(current_move.from - 8) };
        } else {
            self.state.en_passent = None;
        }

        self.board.move_piece(current_move);

        self.turn = self.turn.other();
    }

    pub fn unmake_move(&mut self, current_move: Move) {
        let square_to_move = self.board.squares[current_move.to as usize];
        if square_to_move.is_none() {
            println!("No piece to move!");
            return;
        }
        let piece_to_move = square_to_move.unwrap();
        if piece_to_move.color == self.turn { 
            println!("The piece you want to move is not the correct team!");
            return;
        }
        
        
        // We have a valid move!
        let current_state = self.state.clone();

        
        match current_move.move_type {
            MoveType::BishopPromotion | MoveType::KnightPromotion | MoveType::QueenPromotion | MoveType::RookPromotion => {
                self.board.squares[current_move.from as usize] = Some(Piece::new(piece_to_move.color, PieceType::Pawn, current_move.from));
                self.board.squares[current_move.to as usize] = None;
            },
            MoveType::EnPassantCapture => {
                let capture_pos = if let Team::White = piece_to_move.color { current_move.to - 8 } else { current_move.to + 8 };
                self.board.squares[capture_pos as usize] = Some(Piece::new(piece_to_move.color.other(), PieceType::Pawn, capture_pos));
                self.board.squares[current_move.from as usize] = Some(Piece {position: current_move.from, ..piece_to_move});
                self.board.squares[current_move.to as usize] = None;
            },
            MoveType::Capture => {
                let capture_type = if let Some(capture_type) = current_state.captured_piece {
                    capture_type
                } else {
                    println!("No capture type! {:?}", current_state);
                    PieceType::Pawn
                };
                self.board.squares[current_move.from as usize] = Some(Piece {position: current_move.from, ..piece_to_move});
                self.board.squares[current_move.to as usize] = Some(Piece::new(piece_to_move.color.other(), capture_type, current_move.to));
            }
            _ => {
                self.board.squares[current_move.from as usize] = Some(Piece {position: current_move.from, ..piece_to_move});
                self.board.squares[current_move.to as usize] = None;
            }
        }
        

        self.state = self.state_stack.pop().unwrap();
        self.turn = self.turn.other();

        self.move_stack.pop();
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



    pub fn run_test(&mut self, depth : u8, debug : bool) -> usize {
        if depth == 0 {
            return 1;
        }

        let moves = self.get_possible_moves(self.turn);

        let mut count = 0;
        for m in moves.iter() {
            self.make_move(*m);
            if debug {
                println!("{}", self.board);
            }
            count += self.run_test(depth - 1, debug);
            self.unmake_move(*m);
        }
        return count;
    }
}

fn get_rook_castle_position(team: Team, king_side : bool) -> u8 {
    if let Team::White = team {
        if king_side {
            5
        } else {
            3
        }
    } else {
        if king_side {
            61
        } else {
            59
        }
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
    const fn new(color: Team, piece_type: PieceType, position: u8) -> Piece {
        Piece {
            piece_type,
            color,
            position
        }
    }

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
                let next_is_last = if self.color == Team::White { pos.1 == 6 } else { pos.1 == 1 };

                // Moving forward
                if game_state.board.get_piece(position_from_xy(pos.0, next_one_y)).is_none() {
                    if next_is_last {
                        // We add all the promotion moves to the list for now
                        moves.push(Move {
                            from: self.position,
                            to: position_from_xy(pos.0, next_one_y),
                            move_type: MoveType::RookPromotion
                        });
                        moves.push(Move {
                            from: self.position,
                            to: position_from_xy(pos.0, next_one_y),
                            move_type: MoveType::QueenPromotion
                        });
                        moves.push(Move {
                            from: self.position,
                            to: position_from_xy(pos.0, next_one_y),
                            move_type: MoveType::BishopPromotion
                        });
                        moves.push(Move {
                            from: self.position,
                            to: position_from_xy(pos.0, next_one_y),
                            move_type: MoveType::KnightPromotion
                        });
                    } else {
                        moves.push(Move {
                            from: self.position,
                            to: position_from_xy(pos.0, next_one_y),
                            move_type: MoveType::Quite
                        });
                    }
                    if first_move && game_state.board.get_piece(position_from_xy(pos.0, next_two_y)).is_none()
                    {
                        moves.push(Move {
                            from: self.position,
                            to: position_from_xy(pos.0, next_two_y),
                            move_type: MoveType::DoublePawnPush
                        });
                    }
                }
                // Capture

                if pos.0 < 7 {
                    let capture_pos = position_from_xy(pos.0 + 1, next_one_y);
                    let is_enpassant = if let Some(en_passent) = game_state.state.en_passent {
                        en_passent == capture_pos
                    } else {
                        false
                    };

                    if let Some(to_capture) = game_state.board.get_piece(capture_pos) {
                        if to_capture.color != self.color {
                            moves.push(Move {
                                from: self.position,
                                to: capture_pos,
                                move_type: MoveType::Capture
                            })
                        }
                    }
                    else if is_enpassant {
                        moves.push(Move {
                            from: self.position,
                            to: capture_pos,
                            move_type: MoveType::EnPassantCapture
                        });
                    }
                }

                if pos.0 > 0 {
                    let capture_pos = position_from_xy(pos.0 - 1, next_one_y);
                    let is_enpassant = if let Some(en_passent) = game_state.state.en_passent {
                        en_passent == capture_pos
                    } else {
                        false
                    };

                    if let Some(to_capture) = game_state.board.get_piece(capture_pos) {
                        if to_capture.color != self.color {
                            
                            moves.push(Move {
                                from: self.position,
                                to: capture_pos,
                                move_type: MoveType::Capture
                            });
                        }
                    } else if is_enpassant {
                        moves.push(Move {
                            from: self.position,
                            to: capture_pos,
                            move_type: MoveType::EnPassantCapture
                        });
                    }
                }
            },
            PieceType::Knight => {
                let move_index = [(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)];
                for m in move_index.iter() {
                    let new_pos = (pos.0 as i8 + m.0, pos.1 as i8 + m.1);
                    if position_xy_inside_s(new_pos.0, new_pos.1) {
                        let piece = game_state.board.get_piece(position_from_xy(new_pos.0 as u8, new_pos.1 as u8));
                        let capture_piece = piece.is_some() && piece.unwrap().color != self.color;
                        if piece.is_none() || piece.unwrap().color != self.color {
                            moves.push(Move {
                                from: self.position,
                                to: position_from_xy(new_pos.0 as u8, new_pos.1 as u8),
                                move_type: if capture_piece { MoveType::Capture } else { MoveType::Quite }
                            });
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
                            let capture_piece = piece.is_some() && piece.unwrap().color != self.color;

                            if piece.is_none() || piece.unwrap().color != self.color {
                                moves.push(Move {
                                    from: self.position,
                                    to: position_from_xy(pos.0 as u8, pos.1 as u8),
                                    move_type: if capture_piece { MoveType::Capture } else { MoveType::Quite }
                                });
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
                            let capture_piece = piece.is_some() && piece.unwrap().color != self.color;

                            if piece.is_none() || piece.unwrap().color != self.color {
                                moves.push(Move {
                                    from: self.position,
                                    to: position_from_xy(pos.0 as u8, pos.1 as u8),
                                    move_type: if capture_piece { MoveType::Capture } else { MoveType::Quite }
                                });
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
                            let capture_piece = piece.is_some() && piece.unwrap().color != self.color;

                            if piece.is_none() || piece.unwrap().color != self.color {
                                moves.push(Move {
                                    from: self.position,
                                    to: position_from_xy(pos.0 as u8, pos.1 as u8),
                                    move_type: if capture_piece { MoveType::Capture } else { MoveType::Quite }
                                });
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
                        let capture_piece = piece.is_some() && piece.unwrap().color != self.color;

                        if piece.is_none() || piece.unwrap().color != self.color {
                            moves.push(Move {
                                from: self.position,
                                to: position_from_xy(pos.0 as u8, pos.1 as u8),
                                move_type: if capture_piece { MoveType::Capture } else { MoveType::Quite }
                            });
                        }
                    }
                }
            
                // Check casteling
                match self.color {
                    Team::White => {
                        if game_state.state.castle_state & casteling::WHITE_KING_SIDE != 0 {
                            if game_state.board.is_free(5) && 
                                game_state.board.is_free(6) {
                                // The way is free to castle
                                moves.push(Move {
                                    from: self.position,
                                    to: 6,
                                    move_type: MoveType::KingCastle
                                });
                            }
                        }
                        if game_state.state.castle_state & casteling::WHITE_QUEEN_SIDE != 0 {
                            if game_state.board.is_free(1) && 
                                game_state.board.is_free(2) &&
                                game_state.board.is_free(3) {
                                    moves.push(Move {
                                        from: self.position,
                                        to: 2,
                                        move_type: MoveType::QueenCastle
                                    });
                            }
                        }
                    },
                    Team::Black => {
                        if game_state.state.castle_state & casteling::BLACK_KING_SIDE != 0 {
                            if game_state.board.is_free(61) &&
                                game_state.board.is_free(62) 
                            {
                                moves.push(Move {
                                    from: self.position,
                                    to: 62,
                                    move_type: MoveType::KingCastle
                                });
                            }
                        }
                        if game_state.state.castle_state & casteling::BLACK_QUEEN_SIDE != 0 {
                            if game_state.board.is_free(59) &&
                                game_state.board.is_free(58) && 
                                game_state.board.is_free(57) 
                            {
                                moves.push(Move {
                                    from: self.position,
                                    to: 58,
                                    move_type: MoveType::QueenCastle
                                });  
                            }
                        }
                    },
                }
            },
        }

        return moves;
    }
}


#[derive(Copy, Clone, Debug)]
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

    fn move_piece(&mut self, _move: Move) {
        if let Some(piece_to_move) = self.get_piece(_move.from) {
            self.squares[_move.to as usize] = Some(Piece {
                position: _move.to,
                ..piece_to_move
            });
            self.squares[_move.from as usize] = None;
        }

    }

    fn is_ocupied(&self, position: u8) -> bool {
        return self.get_piece(position).is_some();
    }

    fn is_free(&self, position: u8) -> bool {
        return !self.is_ocupied(position);
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


#[derive(Default, Debug, Copy, Clone, PartialEq)]
enum MoveType {
    #[default] Quite,
    Capture,
    DoublePawnPush,
    EnPassantCapture,
    KingCastle,
    QueenCastle,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
}

impl MoveType {
    fn is_castle(&self) -> bool {
        match self {
            MoveType::KingCastle => true,
            MoveType::QueenCastle => true,
            _ => false
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Move {
    from: u8,
    to: u8,
    move_type: MoveType,
}

impl Move {
    pub const fn new(from : u8, to : u8) -> Move {
        Move {
            from,
            to,
            move_type: MoveType::Quite,
        }
    }
}

enum InputMessage {
    Move(Move),
    UndoMove,
    ShowMoves(u8),
    ShowTeam(Team),
    ShowBoard,
    LoadFen(String),
    RunTest(u8, bool),
    ShowFen,
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

    let args: Vec<&str> = input.split(" ").collect();
    if args[0] == "m" {
        if args.len() != 3 {
            return InputMessage::None;
        }
        
        let from = get_position_from_string(args[1]);
        let to = get_position_from_string(args[2]);
        return InputMessage::Move(Move::new(from, to));
    } else if args[0] == "s" {
        if args.len() != 2 {
            return InputMessage::ShowBoard;
        }
        if args[1].to_uppercase() == "BLACK" || args[1].to_uppercase() == "WHITE" {
            let team = if args[1].to_uppercase() == "BLACK" { Team::Black } else { Team::White };
            return InputMessage::ShowTeam(team);
        } else {
            let position = get_position_from_string(args[1]);
            return InputMessage::ShowMoves(position);
        }
    } else if args[0] == "fen" {
        if args.len() == 1 {
            return InputMessage::ShowFen;
        }
        return InputMessage::LoadFen(input[4..].to_string());
    } else if args[0] == "um" {
        return InputMessage::UndoMove;
    } else if args[0] == "rt" {
        if args.len() < 2 {
            return InputMessage::None;
        }
        let test_number = args[1].parse::<u8>().unwrap();
        let debug = if args.len() == 3 {args[2].parse::<bool>().unwrap()} else { false };
        return InputMessage::RunTest(test_number, debug);
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
                if m.move_type == MoveType::Capture {
                    return 'X';
                }

                match m.move_type {
                    MoveType::Capture => return 'X',
                    MoveType::EnPassantCapture => return 'E',
                    _ => return '+'
                }
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
            InputMessage::UndoMove => {
                if let Some(m) = game_state.move_stack.last() {
                    game_state.unmake_move(*m);
                    println!("{}", game_state.board);
                }
            },
            InputMessage::ShowMoves(pos) => {
                if let Some(piece) = game_state.board.get_piece(pos) {
                    let moves = piece.get_possible_moves(&game_state);
                    print_possible_moves(&game_state.board, &moves);
                } else {
                    println!("Could not find piece at position!");
                }
            },
            InputMessage::ShowTeam(team) => {
                let moves = game_state.get_possible_moves(team);
                print_possible_moves(&game_state.board, &moves);
            },
            InputMessage::ShowBoard => {
                println!("{}", game_state.board);
            },
            InputMessage::LoadFen(str) => {
                game_state = GameState::from_fen(&str);
                println!("{}", game_state.board);
            },
            InputMessage::ShowFen => {
                println!("{}", game_state.to_fen_str());
            },
            InputMessage::RunTest(depth, debug) => {
                println!("Running test with depth {}", depth);
                println!("Starting fen: {}", game_state.to_fen_str());
                println!("------------------");
                println!("Result: {}", game_state.run_test(depth, debug));

            }
            InputMessage::None => { },
            InputMessage::Quit => {
                break;
            },
        }
    }
}