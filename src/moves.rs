use crate::base_types::Position;


#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum MoveType {
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
    KnightPromotionCapture,
    BishopPromotionCapture,
    RookPromotionCapture,
    QueenPromotionCapture,
}

impl MoveType {
    fn is_castle(&self) -> bool {
        match self {
            MoveType::KingCastle => true,
            MoveType::QueenCastle => true,
            _ => false
        }
    }

    fn is_promotion(&self) -> bool {
        match self {
            MoveType::KnightPromotion => true,
            MoveType::BishopPromotion => true,
            MoveType::RookPromotion => true,
            MoveType::QueenPromotion => true,
            MoveType::KnightPromotionCapture => true,
            MoveType::BishopPromotionCapture => true,
            MoveType::RookPromotionCapture => true,
            MoveType::QueenPromotionCapture => true,
            _ => false
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub move_type: MoveType,
}

impl Move {
    pub const fn new(from : Position, to : Position) -> Move {
        Move {
            from,
            to,
            move_type: MoveType::Quite,
        }
    }

    pub fn to_string(&self) -> String {
        let mut fen: String = String::new();

        let position = self.from;
        let x = position.get_col();
        let y = position.get_row();

        fen.push((x as u8 + 'a' as u8) as char);
        fen.push((y as u8 + '1' as u8) as char);

        let position = self.to;
        let x = position.get_col();
        let y = position.get_row();

        fen.push((x as u8 + 'a' as u8) as char);
        fen.push((y as u8 + '1' as u8) as char);

        fen
    }

    pub fn from_string(string : &str) -> Move {
        let mut iter = string.chars();
        let from_x = iter.next().unwrap() as u8 - 'a' as u8;
        let from_y = iter.next().unwrap() as u8 - '1' as u8;
        let to_x = iter.next().unwrap() as u8 - 'a' as u8;
        let to_y = iter.next().unwrap() as u8 - '1' as u8;

        Move {
            from: Position::from((from_x, from_y)),
            to: Position::from((to_x, to_y)),
            move_type: MoveType::Quite,
        }
    }
}