use crate::precompute::{NUM_SQUARES_TO_EDGE, DIRECTION_OFFSETS};

#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct Position(u8);

impl Position {
    pub fn new(position: u8) -> Position {
        Position(position)
    }

    pub fn get_row(&self) -> u8 {
        self.0 / 8
    }

    pub fn get_col(&self) -> u8 {
        self.0 % 8
    }

    pub fn get_change(&self, change: i8) -> Position {
        Position((self.0 as i8 + change) as u8)
    }

    pub fn is_valid(&self) -> bool {
        self.0 < 64
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<(u8, u8)> for Position {
    fn from((col, row): (u8, u8)) -> Self {
        Position(row * 8 + col)
    }
}

impl From<String> for Position {
    fn from(str: String) -> Self {
        // Validating input
        if str.len() != 2 {
            panic!("Position String is not 2 characters long");
        }

        let col = str.chars().nth(0).unwrap() as u8 - 'a' as u8;
        let row = str.chars().nth(1).unwrap() as u8 - '1' as u8;

        // Validate values
        if col > 7 || row > 7 {
            panic!("Invalid position string");
        }

        Position::from((col, row))
    }
}

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
                squares.append(&mut get_sliding_squares(self.position, 4, 8));
            }
            PieceType::Rook => {
                squares.append(&mut get_sliding_squares(self.position, 0, 4));
            }
            PieceType::Queen => {
                squares.append(&mut get_sliding_squares(self.position, 0, 8));
            }
            PieceType::King => {
                
            }
        }

        squares
    }
}

/**
 * returns a vector of all squares that a piece can move to. 
 * The start_dir and end_dir are used to determine which directions to check.
 * 0 - 4 are the 4 cardinal directions, 4 - 8 are the 4 diagonal directions.
 */
fn get_sliding_squares(position: Position, start_dir: u8, end_dir: u8) -> Vec<Position> {
    let mut result : Vec<Position> = Vec::new();
    for i in start_dir .. end_dir {
        for j in 0 .. NUM_SQUARES_TO_EDGE[position.index()][i as usize] {
            let square = position.get_change(DIRECTION_OFFSETS[i as usize]);
            if square.is_valid() {
                result.push(square);
            }
        }
    }
    result
}