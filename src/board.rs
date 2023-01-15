use std::io::{stdin, Read};

use crate::piece::{Piece, move_sliding_squares};
use crate::base_types::{Position, Color, PieceType};
use crate::precompute::get_direction_index;

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



    pub fn get_king_pins(&self, team : Color) -> (Vec<u64>, u64) {
        let mut king_pins = Vec::new();
        let mut king_check = 0;
        for piece in self.pieces {
            if let Some(piece) = piece {
                // For performance we only check sliding pieces (queen, rook, bishop)
                if piece.color != team {
                    let mut own_piece_count = 0;
                    piece.move_all_directions(&mut|position, start_dir| -> bool {
                        // Reset piece count if we start new dir
                        if start_dir {
                            own_piece_count = 0;
                        }
                        if let Some(found_piece) = self.get_piece(position) {
                            if found_piece.color == team {
                                if let PieceType::King = found_piece.piece_type {
                                    // We found king. We need to go through the line to add all the pins

                                    let is_attacking_move = if let PieceType::Pawn = piece.piece_type {
                                        if piece.position.get_col() != position.get_col() {
                                            true
                                        } else {
                                            false
                                        }
                                    } else {
                                        true
                                    };
                                    
                                    // Add the piece position to the pins to check capture
                                    if own_piece_count == 0 && is_attacking_move {
                                        king_check |= 1 << piece.position.index();
                                    }
                                    
                                    if piece.is_sliding() {
                                        let mut pins = 1 << piece.position.index();

                                        let sliding_dir = get_direction_index(piece.position, position) as u8;
                                        move_sliding_squares(piece.position, (sliding_dir, sliding_dir + 1), &mut|pin_pos, _| -> bool {
                                            if pin_pos.index() != position.index() {
                                                if own_piece_count == 0 {
                                                    king_check |= 1 << pin_pos.index();
                                                }
                                                pins |= 1 << pin_pos.index();
                                                true
                                            } else {
                                                false                  
                                            }
                                        });

                                        if own_piece_count > 0 {
                                            king_pins.push(pins);
                                        }
                                    }
                                    return false;
                                } else { // Other piece of current color
                                    own_piece_count += 1;
                                    if own_piece_count > 1 {
                                        return false;
                                    }
                                    return true;
                                }
                            }
                            return false;
                        }
                        true
                    });
                }
            }
        }
        (king_pins, king_check)
    }

    pub fn get_enemy_attacks(&self, team : Color) -> u64 {
        let mut enemy_attacks = 0;
        for piece in self.pieces {
            if let Some(piece) = piece {
                if piece.color != team {
                       piece.move_all_directions(&mut|position, _| -> bool {
                        if let PieceType::Pawn = piece.piece_type {
                            if piece.position.get_col() != position.get_col() {
                                //println!("Pawn attack {}", position.index());
                                if position.is_valid() {
                                    enemy_attacks |= 1 << position.index();
                                }
                                return false;
                            }
                            return true;
                        }

                        // We want to continue if we find a king
                        if let Some(found_piece) = self.get_piece(position) {
                            if let PieceType::King = found_piece.piece_type {
                                if found_piece.color == team {
                                    enemy_attacks |= 1 << position.index();
                                    return true;
                                }
                            }
                        }


                        enemy_attacks |= 1 << position.index();
                        !self.has_piece(position)
                    });
                }
            }
        }

        enemy_attacks
    }
}