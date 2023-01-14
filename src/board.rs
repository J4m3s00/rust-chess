use std::io::{stdin, Read};

use crate::piece::{Piece};
use crate::base_types::{Position, Color, PieceType};

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
            println!("No piece at {}{}", from.to_string(),to.to_string());
            let _ = stdin().read(&mut [0u8]).unwrap();
            return;
        }
        let piece = self.get_piece(from).unwrap();
        self.pieces[from.index()] = None;
        self.pieces[to.index()] = Some(Piece::new(piece.color, piece.piece_type, to));
    }

    pub fn has_piece(&self, position: Position) -> bool {
        self.get_piece(position).is_some()
    }

    pub fn print_custom(&self, callback: &dyn Fn(Position) -> char) {
        println!("+---+---+---+---+---+---+---+---+");
        for i in 0..8{
            print!("|");
            for j in 0..8{
                print!(" {} |", callback(Position::from((j, 7 - i))));
            }
            println!(" {}", 8 - i);
            println!("+---+---+---+---+---+---+---+---+");
        }
        println!("  a   b   c   d   e   f   g   h  ");
    }

    pub fn print(&self) { 
        self.print_custom(&|position| -> char {
            let piece = self.get_piece(position);
            match piece {
                Some(piece) => {
                    piece.get_char()
                },
                None => ' ',
            }
        })
    }
}