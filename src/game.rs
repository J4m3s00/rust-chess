use crate::{board::Board, piece::{Piece}, moves::{Move, MoveType}, base_types::{Color, Position, PieceType}};

pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub white_king_position: Position,
    pub black_king_position: Position,
    pub white_can_castle_kingside: bool,
    pub white_can_castle_queenside: bool,
    pub black_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,
    pub en_passant_target: Option<Position>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: Board::new(),
            turn: Color::White,
            white_king_position: Position::from((4, 0)),
            black_king_position: Position::from((4, 7)),
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
            en_passant_target: None,
        }
    }
}

impl Game {
    pub fn make_move(&mut self, mov : Move) {
        let moving_piece = self.board.get_piece(mov.from);
        if moving_piece.is_none() {
            println!("No piece to move selected!");
            return;
        }
        let moving_piece = moving_piece.unwrap();
        if moving_piece.color != self.turn {
            println!("Piece is not the right color!");
            return;
        }



        self.board.move_piece(mov.from, mov.to);
        self.turn = self.turn.opposite();
    }

    pub fn get_possible_team_moves(&self, c : Color) -> Vec<Move> {
        let mut moves : Vec<Move> = Vec::new();
        for piece in self.board.pieces {
            if let Some(piece) = piece {
                if piece.color == c {
                    moves.append(&mut self.get_possible_piece_moves(piece));
                }
            }
        }
        moves
    }

    pub fn get_possible_piece_moves(&self, piece : Piece) -> Vec<Move> {
        let mut moves : Vec<Move> = Vec::new();

        piece.move_all_directions(&mut|position : Position| -> bool {
            let piece_on_position = self.board.get_piece(position);
            if piece_on_position.is_some() {
                return false;
            }
            moves.push(Move {
                from: piece.position,
                to: position,
                move_type: MoveType::Quite
            });
            return true;
        });

        moves
    }

    pub fn from_fen(&mut self, fen: &str) {
        let mut board = Board::new();
        let mut en_passent = None;

        let mut fen_iter = fen.split(' ');
        let board_fen = fen_iter.next().unwrap();
        let turn_fen = fen_iter.next().unwrap();
        let castle_fen = fen_iter.next().unwrap();
        let en_passent_fen = fen_iter.next().unwrap();
        let mut white_king_position = Position::from((4, 0));
        let mut black_king_position = Position::from((4, 7));

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
                    let color = if c.is_ascii_uppercase() { Color::White } else { Color::Black };
                    let position = Position::from((x as u8, 7 - y as u8));
                    if let PieceType::King = piece_type {
                        if let Color::White = color {
                            white_king_position = position;
                        } else {
                            black_king_position = position;
                        }
                    }
                    board.add_piece(Piece::new(color, piece_type, position));
                    x += 1;
                }
            }
        }

        if en_passent_fen != "-" {
            en_passent = Some(Position::from(en_passent_fen.to_string()));
        }

        /*self.board = board;
        self.turn = if turn_fen == "w" { Color::White } else { Color::Black };
        self.white_can_castle_kingside = castle_fen.contains('K');
        self.white_can_castle_queenside = castle_fen.contains('Q');
        self.black_can_castle_kingside = castle_fen.contains('k');
        self.black_can_castle_queenside = castle_fen.contains('q');
        self.en_passant_target = en_passent;
        self.white_king_position = white_king_position;
        self.black_king_position = black_king_position;*/

        *self = Game {
            board,
            turn: if turn_fen == "w" { Color::White } else { Color::Black },
            white_can_castle_kingside: castle_fen.contains('K'),
            white_can_castle_queenside: castle_fen.contains('Q'),
            black_can_castle_kingside: castle_fen.contains('k'),
            black_can_castle_queenside: castle_fen.contains('q'),
            en_passant_target: en_passent,
            white_king_position,
            black_king_position,
        };
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for y in 0..8 {
            let mut empty_count = 0;
            for x in 0..8 {
                let position = Position::from((x, 7 - y));
                let square = self.board.get_piece(position);
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
                    fen.push(if let Color::White = piece.color { piece_char } else { piece_char.to_ascii_lowercase() });
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
        fen.push(if let Color::White = self.turn { 'w' } else { 'b' });
        fen.push(' ');
        
        let mut can_castle = false;

        if self.white_can_castle_kingside { can_castle = true; fen.push('K'); }
        if self.white_can_castle_queenside { can_castle = true; fen.push('Q'); }
        if self.black_can_castle_kingside { can_castle = true; fen.push('k'); }
        if self.black_can_castle_queenside { can_castle = true; fen.push('q'); }

        if !can_castle {
            fen.push('-');
        }

        fen.push(' ');
        if self.en_passant_target.is_none() {
            fen.push('-');
        } else {
            let position = self.en_passant_target.unwrap();
            let x = position.get_col();
            let y = position.get_row();
            fen.push((x as u8 + 'a' as u8) as char);
            fen.push((y as u8 + '1' as u8) as char);
        }

        fen.push_str(" 0 1");

        fen
    }
}