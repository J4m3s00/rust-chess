use crate::base_types::{Position, PieceType};


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

    pub fn is_promotion(&self) -> bool {
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

    pub fn is_promotion_capture(&self) -> bool {
        match self {
            MoveType::KnightPromotionCapture => true,
            MoveType::BishopPromotionCapture => true,
            MoveType::RookPromotionCapture => true,
            MoveType::QueenPromotionCapture => true,
            _ => false
        }
    }

    pub fn get_promotion_piece(&self) -> PieceType {
        match self {
            MoveType::KnightPromotion => PieceType::Knight,
            MoveType::BishopPromotion => PieceType::Bishop,
            MoveType::RookPromotion => PieceType::Rook,
            MoveType::QueenPromotion => PieceType::Queen,
            MoveType::KnightPromotionCapture => PieceType::Knight,
            MoveType::BishopPromotionCapture => PieceType::Bishop,
            MoveType::RookPromotionCapture => PieceType::Rook,
            MoveType::QueenPromotionCapture => PieceType::Queen,
            _ => {println!("Error: MoveType::get_promotion_piece() called on non-promotion move"); PieceType::Pawn}
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
        return self.from.to_string() + &self.to.to_string();
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