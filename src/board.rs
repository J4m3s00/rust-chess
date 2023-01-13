use crate::piece::{Piece, Position, PieceType, Color};

pub struct Board {
    pub pieces: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [None; 64],
        }
    }

    pub fn get_piece(&self, position: Position) -> Option<Piece> {
        if position.is_valid() {
            self.pieces[position.index()]
        } else {
            None
        }
    }

    pub fn add_piece(&mut self, piece: Piece) {
        self.pieces[piece.position.index()] = Some(piece);
    }

    pub fn remove_piece(&mut self, position: Position) {
        self.pieces[position.index()] = None;
    }

    pub fn move_piece(&mut self, from: Position, to: Position) {
        if !self.has_piece(from) {
            panic!("No piece at position");
        }
        let piece = self.get_piece(from).unwrap();
        self.pieces[from.index()] = None;
        self.pieces[to.index()] = Some(Piece::new(piece.color, piece.piece_type, to));
    }

    pub fn has_piece(&self, position: Position) -> bool {
        self.get_piece(position).is_some()
    }

    pub fn print_custom(&self, callback: &dyn Fn(Option<Piece>) -> char) {
        println!("---------------------------------");
        for i in 0..8{
            print!("|");
            for j in 0..8{
                print!(" {} |", callback(self.get_piece(Position::from((j, 7 - i)))));
            }
            println!("");
            println!("---------------------------------");
        }
    }

    pub fn print(&self) { 
        self.print_custom(&|piece: Option<Piece>| -> char {
            match piece {
                Some(piece) => {
                    match piece.piece_type {
                        PieceType::Pawn => if let Color::White = piece.color { 'P' } else { 'p' },
                        PieceType::Rook => if let Color::White = piece.color { 'R' } else { 'r' },
                        PieceType::Knight => if let Color::White = piece.color { 'N' } else { 'n' },
                        PieceType::Bishop => if let Color::White = piece.color { 'B' } else { 'b' },
                        PieceType::Queen => if let Color::White = piece.color { 'Q' } else { 'q' },
                        PieceType::King => if let Color::White = piece.color { 'K' } else { 'k' },
                    }
                },
                None => ' ',
            }
        })
    }
}