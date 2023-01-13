use crate::{precompute::{NUM_SQUARES_TO_EDGE, DIRECTION_OFFSETS}, base_types::{Color, PieceType, Position}, moves::{Move, MoveType}};


#[derive(Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub position: Position,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType, position: Position) -> Piece {
        Piece {
            color,
            piece_type,
            position,
        }
    }

    pub fn move_all_directions(&self, callback : &mut dyn FnMut(Position) -> bool) {
        match self.piece_type {
            PieceType::Pawn => {
                if let Color::White = self.color  {
                    callback(self.position.get_change(8));
                    if self.position.get_row() == 1 {
                        callback(self.position.get_change(16));
                    }
                } else {
                    callback(self.position.get_change(-8));
                    if self.position.get_row() == 6 {
                        callback(self.position.get_change(-16));
                    }
                }
            },
            PieceType::Knight => {
                let x = self.position.get_col() as i8;
                let y = self.position.get_row() as i8;


                let positions = [
                    Position::from((x + 1, y + 2)),
                    Position::from((x + 1, y - 2)),
                    Position::from((x + 2, y + 1)),
                    Position::from((x + 2, y - 1)),
                    Position::from((x - 1, y + 2)),
                    Position::from((x - 1, y - 2)),
                    Position::from((x - 2, y + 1)),
                    Position::from((x - 2, y - 1))
                ];

                for pos in positions {
                    if pos.is_valid() {
                        callback(pos);
                    }
                }
            },
            PieceType::Bishop => {
                move_sliding_squares(self.position, 4, 8, callback);
            },
            PieceType::Rook => {
                move_sliding_squares(self.position, 0, 4, callback);
            },
            PieceType::Queen => {
                move_sliding_squares(self.position, 0, 8, callback);
            },
            PieceType::King => {},
        }
    }

    pub fn get_all_viewing_squares(&self) -> Vec<Position> {
        let mut squares = Vec::new();
        match self.piece_type {
            PieceType::Pawn => {
                if let Color::White = self.color  {
                    squares.push(self.position.get_change(8));
                    if self.position.get_row() == 1 {
                        squares.push(self.position.get_change(16));
                    }
                } else {
                    squares.push(self.position.get_change(-8));
                    if self.position.get_row() == 6 {
                        squares.push(self.position.get_change(-16));
                    }
                }
            }
            PieceType::Knight => {
                
            }
            PieceType::Bishop => {
                
            }
            PieceType::Rook => {
                
            }
            PieceType::Queen => {
                
            }
            PieceType::King => {
                
            }
        }

        squares
    }

    pub fn get_char(&self) -> char {
        match self.piece_type {
            PieceType::Pawn => if let Color::White = self.color { 'P' } else { 'p' },
            PieceType::Rook => if let Color::White = self.color { 'R' } else { 'r' },
            PieceType::Knight => if let Color::White = self.color { 'N' } else { 'n' },
            PieceType::Bishop => if let Color::White = self.color { 'B' } else { 'b' },
            PieceType::Queen => if let Color::White = self.color { 'Q' } else { 'q' },
            PieceType::King => if let Color::White = self.color { 'K' } else { 'k' },
        }
    }
}

/**
 * returns a vector of all squares that a piece can move to. 
 * The start_dir and end_dir are used to determine which directions to check.
 * 0 - 4 are the 4 cardinal directions, 4 - 8 are the 4 diagonal directions.
 */
fn move_sliding_squares(position: Position, start_dir: u8, end_dir: u8, callback : &mut dyn FnMut(Position) -> bool) {
    for i in start_dir .. end_dir {
        let mut current_position = position;
        let direction_offset = DIRECTION_OFFSETS[i as usize];
        let num_square_to_edge = NUM_SQUARES_TO_EDGE[position.index()][i as usize];
        println!("Direction of {i} = {direction_offset} and num square = {num_square_to_edge}");

        for _ in 0 .. num_square_to_edge {
            current_position = current_position.get_change(direction_offset);
            if current_position.is_valid() {
                if !callback(current_position) {
                    break;
                }
            }
        }
    }
}