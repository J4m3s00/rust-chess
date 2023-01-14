
#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq)]
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

    pub fn to_string(&self) -> String {
        let mut fen = String::new();
        let x = self.get_col();
        let y = self.get_row();

        fen.push((x as u8 + 'a' as u8) as char);
        fen.push((y as u8 + '1' as u8) as char);

        return fen;
    }
}

impl From<(u8, u8)> for Position {
    fn from((col, row): (u8, u8)) -> Self {
        if col > 7 || row > 7 {
            return Position(u8::MAX);
        }
        Position(row * 8 + col)
    }
}

impl From<(i8, i8)> for Position {
    fn from((col, row) : (i8, i8)) -> Self {
        if col < 0 || col > 7 || row < 0 || row > 7 {
            return Position(u8::MAX);
        }
        Position::from((col as u8, row as u8))
    }
}

impl From<(i32, i32)> for Position {
    fn from((col, row) : (i32, i32)) -> Self {
        if col < 0 || col > 7 || row < 0 || row > 7 {
            return Position(u8::MAX);
        }
        Position::from((col as u8, row as u8))
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