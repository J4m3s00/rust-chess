
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {

    pub fn get_value(&self) -> i32 {
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 300,
            PieceType::Bishop => 300,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 10000,
        }
    }


    pub fn from_char(c: char) -> PieceType {
        match c.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => PieceType::Pawn,
        }
    }

    pub fn get_char(&self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
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

    pub fn to_string(&self) -> &str {
        match self {
            Color::White => "White",
            Color::Black => "Black",
        }
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct Position(u8);

impl Position {
    pub fn new(position: u8) -> Position {
        Position(position)
    }

    #[allow(dead_code)]
    pub fn invalid() -> Position {
        Position(u8::MAX)
    }

    pub fn get_row(&self) -> u8 {
        self.0 / 8
    }

    pub fn get_col(&self) -> u8 {
        self.0 % 8
    }

    pub fn bitboard(&self) -> u64 {
        1 << self.0
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