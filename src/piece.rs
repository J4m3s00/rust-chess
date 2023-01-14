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

    pub fn move_all_directions(&self, callback : &mut dyn FnMut(Position, bool) -> bool) {
        match self.piece_type {
            PieceType::Pawn => {
                let change = if let Color::White = self.color { 8 } else { -8 };
                let double_push_row = if let Color::White = self.color { 1 } else { 6 };

                if self.position.get_col() != 0 {
                    callback(self.position.get_change(change - 1), true);
                }
                if self.position.get_col() != 7 {
                    callback(self.position.get_change(change + 1), true);
                }

                if callback(self.position.get_change(change), true) &&
                    self.position.get_row() == double_push_row {
                    callback(self.position.get_change(change*2), false);
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
                        callback(pos, true);
                    }
                }
            },
            PieceType::Bishop => {
                move_sliding_squares(self.position, (4, 8), callback);
            },
            PieceType::Rook => {
                move_sliding_squares(self.position, (0, 4), callback);
            },
            PieceType::Queen => {
                move_sliding_squares(self.position,(0, 8), callback);
            },
            PieceType::King => {
                let x = self.position.get_col() as i8;
                let y = self.position.get_row() as i8;

                let positions = [
                    Position::from((x + 1, y)),
                    Position::from((x + 1, y + 1)),
                    Position::from((x, y + 1)),
                    Position::from((x - 1, y + 1)),
                    Position::from((x - 1, y)),
                    Position::from((x - 1, y - 1)),
                    Position::from((x, y - 1)),
                    Position::from((x + 1, y - 1))
                ];

                for pos in positions {
                    if pos.is_valid() {
                        callback(pos, true);
                    }
                }

                // Castling
                if self.position.get_row() == 0 {
                    if self.position.get_col() == 4 {
                        callback(Position::from((2, 0)), true);
                        callback(Position::from((6, 0)), true);
                    }
                } else if self.position.get_row() == 7 {
                    if self.position.get_col() == 4 {
                        callback(Position::from((2, 7)), true);
                        callback(Position::from((6, 7)), true);
                    }
                }
            },
        }
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

    pub fn is_sliding(&self) -> bool {
        match self.piece_type {
            PieceType::Pawn => false,
            PieceType::Rook => true,
            PieceType::Knight => false,
            PieceType::Bishop => true,
            PieceType::Queen => true,
            PieceType::King => false,
        }
    }
}

/**
 * returns a vector of all squares that a piece can move to. 
 * The start_dir and end_dir are used to determine which directions to check.
 * 0 - 4 are the 4 cardinal directions, 4 - 8 are the 4 diagonal directions.
 */
pub fn move_sliding_squares(position: Position, start_end_dir : (u8, u8), callback : &mut dyn FnMut(Position, bool) -> bool) {
    for i in start_end_dir.0 .. start_end_dir.1 {
        let mut current_position = position;
        let direction_offset = DIRECTION_OFFSETS[i as usize];
        let num_square_to_edge = NUM_SQUARES_TO_EDGE[position.index()][i as usize];

        for j in 0 .. num_square_to_edge {
            current_position = current_position.get_change(direction_offset);
            if current_position.is_valid() {
                if !callback(current_position, j == 0) {
                    break;
                }
            }
        }
    }
}