use crate::{board::Board, piece::{Piece, move_sliding_squares}, moves::{Move, MoveType}, base_types::{Color, Position, PieceType}, precompute::get_direction_index, STARTING_POS_FEN, square_table::{square_table_read, self}};

#[derive(Copy, Clone, Debug)]
pub struct GameState {
    pub white_can_castle_kingside: bool,
    pub white_can_castle_queenside: bool,
    pub black_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,
    pub en_passant_target: Option<Position>,
    pub captured_piece: Option<PieceType>,
}

pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub white_king_position: Position,
    pub black_king_position: Position,
    pub state: GameState,
    state_stack: Vec<GameState>,
    moves: Vec<Move>,
    pub enemy_attacks: u64,
    pub friendly_attacks: u64,
    pub king_pins: Vec<u64>,
    pub enemy_king_pins: Vec<u64>,
    pub king_check: u64, // We can only have one check at a time
    pub enemy_king_check: u64,
}

impl Default for Game {
    fn default() -> Self {
        let mut result = Game {
            board: Board::new(),
            turn: Color::White,
            white_king_position: Position::from((4, 0)),
            black_king_position: Position::from((4, 7)),
            state: GameState {
                white_can_castle_kingside: true,
                white_can_castle_queenside: true,
                black_can_castle_kingside: true,
                black_can_castle_queenside: true,
                en_passant_target: None,
                captured_piece: None,
            },
            state_stack: Vec::new(),
            moves: Vec::new(),
            enemy_attacks: 0,
            friendly_attacks: 0,
            king_pins: Vec::new(),
            enemy_king_pins: Vec::new(),
            king_check: 0,
            enemy_king_check: 0,
        };
        result.update_position();
        result
    }
}

impl Game {
    pub fn evaluate(&self) -> i32 {
        let mut friendly_score = 0;
        let mut enemy_score = 0;


        let friendly_king_pin_check = (self.king_pins.clone(), self.king_check);
        let enemy_king_pin_check = (self.enemy_king_pins.clone(), self.enemy_king_check);

        let own_attacked = self.enemy_attacks;
        let enemy_attacked = self.friendly_attacks;



        for piece in self.board.pieces.iter() {
            if let Some(piece) = piece {
                if piece.color == self.turn {

                    friendly_score += piece.piece_type.get_value();

                    if own_attacked & piece.position.bitboard() != 0 {
                        // High own capture score is not good
                        friendly_score -= piece.piece_type.get_value();
                    }
                } else {
                    enemy_score += piece.piece_type.get_value();

                    if enemy_attacked & piece.position.bitboard() != 0 {
                        enemy_score -= piece.piece_type.get_value();
                    }
                }
            }
        }


        friendly_score += if friendly_king_pin_check.1 != 0 { -100 } else { 0 };
        enemy_score += if enemy_king_pin_check.1 != 0 { -100 } else { 0 };


        friendly_score += self.evaluate_square_table(self.turn);
        enemy_score += self.evaluate_square_table(self.turn.opposite());

        //return score_all_values(count_diff, check_score, pin_score, capture_score);
        return friendly_score - enemy_score;
    }

    pub fn evaluate_square_table(&self, color : Color) -> i32 {
        let mut result = 0;
        let pieces = self.board.pieces.iter().filter(|p| p.is_some() && p.unwrap().color == color).map(|p| p.unwrap()).collect::<Vec<Piece>>();
        for piece in pieces {
            let square_table = match piece.piece_type {
                PieceType::Bishop => &square_table::ST_BISHOPS,
                PieceType::King => &square_table::ST_KING_MID,
                PieceType::Knight => &square_table::ST_KNIGHTS,
                PieceType::Pawn => &square_table::ST_PAWNS,
                PieceType::Queen => &square_table::ST_QUEENS,
                PieceType::Rook => &square_table::ST_ROOKS,
            };
            let position = piece.position;
            result += square_table_read(square_table, position, color);
        }
        result
    }

    pub fn make_move(&mut self, mov : Move) -> bool {
        if !mov.is_valid() {
            println!("Move is not on the board!");
            return false;
        }

        let moving_piece = self.board.get_piece(mov.from);
        if moving_piece.is_none() {
            println!("No piece to move selected!");
            return false;
        }
        let moving_piece = moving_piece.unwrap();
        if moving_piece.color != self.turn {
            println!("Piece is not the right color!");
            return false;
        }
        let piece_possible_moves = self.get_possible_piece_moves(moving_piece);
        let current_found_move_opt = piece_possible_moves.into_iter().find(|m| {
            if mov.move_type.is_promotion() && m.to == mov.to { // Get right promotion piece
                return mov.move_type.get_promotion_piece() == m.move_type.get_promotion_piece();
            }
            m.to == mov.to
        });
        if current_found_move_opt.is_none() {
            println!("Move {:?} is not possible!", mov);
            return false;
        }


        // Valid move
        let current_found_move = current_found_move_opt.unwrap();

        self.state_stack.push(self.state);
        self.moves.push(current_found_move);


        // Update castling rights
        match moving_piece.piece_type {
            PieceType::King => {
                if self.turn == Color::White {
                    self.state.white_can_castle_kingside = false;
                    self.state.white_can_castle_queenside = false;
                } else {
                    self.state.black_can_castle_kingside = false;
                    self.state.black_can_castle_queenside = false;
                }
                match self.turn {
                    Color::White => self.white_king_position = mov.to,
                    Color::Black => self.black_king_position = mov.to,
                }
            },
            PieceType::Rook => {
                if mov.from == Position::from((0 as u8, 0 as u8)) {
                    self.state.white_can_castle_queenside = false;
                } else if mov.from == Position::from((7 as u8, 0 as u8)) {
                    self.state.white_can_castle_kingside = false;
                } else if mov.from == Position::from((0 as u8, 7 as u8)) {
                    self.state.black_can_castle_queenside = false;
                } else if mov.from == Position::from((7 as u8, 7 as u8)) {
                    self.state.black_can_castle_kingside = false;
                }
            },
            _ => {}
        }


        // Reset en passant target
        self.state.en_passant_target = None;


        match current_found_move.move_type {
            MoveType::DoublePawnPush => self.state.en_passant_target = Some(current_found_move.from.get_change(if let Color::White = self.turn { 8 } else { -8 })),
            MoveType::EnPassantCapture => self.board.remove_piece(current_found_move.to.get_change(if let Color::White = self.turn { -8 } else { 8 })),
            MoveType::Capture => {
                let piece_type = self.board.get_piece(current_found_move.to).unwrap().piece_type;
                if let PieceType::Rook = piece_type {
                    if current_found_move.to == Position::from((0 as u8, 0 as u8)) {
                        self.state.white_can_castle_queenside = false;
                    } else if current_found_move.to == Position::from((7 as u8, 0 as u8)) {
                        self.state.white_can_castle_kingside = false;
                    } else if current_found_move.to == Position::from((0 as u8, 7 as u8)) {
                        self.state.black_can_castle_queenside = false;
                    } else if current_found_move.to == Position::from((7 as u8, 7 as u8)) {
                        self.state.black_can_castle_kingside = false;
                    }
                }
                self.state.captured_piece = Some(piece_type);
            }
            MoveType::KingCastle => self.board.move_piece(current_found_move.to.get_change(1), current_found_move.to.get_change(-1)),
            MoveType::QueenCastle => self.board.move_piece(current_found_move.to.get_change(-2), current_found_move.to.get_change(1)),
            MoveType::BishopPromotion | MoveType::KnightPromotion | MoveType::QueenPromotion | MoveType::RookPromotion => {
                self.board.remove_piece(current_found_move.from);
                self.board.add_piece(Piece::new(self.turn, current_found_move.move_type.get_promotion_piece(), current_found_move.to));
            }
            MoveType::BishopPromotionCapture | MoveType::KnightPromotionCapture | MoveType::QueenPromotionCapture | MoveType::RookPromotionCapture => {
                self.state.captured_piece = Some(self.board.get_piece(current_found_move.to).unwrap().piece_type);
                self.board.remove_piece(current_found_move.from);
                self.board.remove_piece(current_found_move.to);
                self.board.add_piece(Piece::new(self.turn, current_found_move.move_type.get_promotion_piece(), current_found_move.to));
            },
            _ => {}
        }

        if !current_found_move.move_type.is_promotion() {
            self.board.move_piece(mov.from, mov.to);
        }
        self.turn = self.turn.opposite();

        self.update_position();
        return true;
    }

    pub fn unmake_move(&mut self) {
        if self.state_stack.len() == 0 || self.moves.len() == 0 {
            println!("No moves to unmake!");
            return;
        }

        let last_move = self.moves.pop().unwrap();

        if let Some(move_piece) = self.board.get_piece(last_move.to) {
            if let PieceType::King = move_piece.piece_type {
                match self.turn {
                    Color::White => self.black_king_position = last_move.from,
                    Color::Black => self.white_king_position = last_move.from,
                }
            }
        }
        
        match last_move.move_type {
            MoveType::BishopPromotion | MoveType::KnightPromotion | MoveType::QueenPromotion | MoveType::RookPromotion => {
                self.board.add_piece(Piece::new(self.turn.opposite(), PieceType::Pawn, last_move.from));
                self.board.remove_piece(last_move.to);
            },
            MoveType::EnPassantCapture => {
                let capture_pos = if let Color::White = self.turn { 8 } else { -8 };

                self.board.move_piece(last_move.to, last_move.from);
                self.board.add_piece(Piece::new(self.turn, PieceType::Pawn, last_move.to.get_change(capture_pos)));
            },
            MoveType::QueenCastle => {
                self.board.move_piece(last_move.to, last_move.from);
                self.board.move_piece(last_move.to.get_change(1), last_move.to.get_change(-2));
            },
            MoveType::KingCastle => {
                self.board.move_piece(last_move.to, last_move.from);
                self.board.move_piece(last_move.to.get_change(-1), last_move.to.get_change(1));
            },
            MoveType::Capture => {
                let capture_type = if let Some(capture_type) = self.state.captured_piece {
                    capture_type
                } else {
                    println!("No capture type! {:?}", self.state);
                    PieceType::Pawn
                };
                self.board.move_piece(last_move.to, last_move.from);
                self.board.add_piece(Piece::new(self.turn, capture_type, last_move.to));
            },
            MoveType::BishopPromotionCapture | MoveType::KnightPromotionCapture | MoveType::QueenPromotionCapture | MoveType::RookPromotionCapture => {
                self.board.add_piece(Piece::new(self.turn.opposite(), PieceType::Pawn, last_move.from));
                self.board.remove_piece(last_move.to);
                self.board.add_piece(Piece::new(self.turn, self.state.captured_piece.unwrap(), last_move.to));
            }
            _ => {
                self.board.move_piece(last_move.to, last_move.from);
            }
        }        


        self.state = self.state_stack.pop().unwrap();
        self.turn = self.turn.opposite();
    
        self.update_position();
    }

    pub fn get_possible_team_moves(&self, c : Color) -> Vec<Move> {
        let mut moves : Vec<Move> = Vec::with_capacity(256);
        for piece in self.board.pieces {
            if let Some(piece) = piece {
                if piece.color == c {
                    moves.append(&mut self.get_possible_piece_moves(piece));
                }
            }
        }
        moves
    }

    pub fn get_possible_piece_moves(&self, piece : Piece) -> Vec<Move> {
        let mut moves : Vec<Move> = Vec::with_capacity(32);

        piece.move_all_directions(&mut|move_to_position, _| -> bool {
            let piece_on_position = self.board.get_piece(move_to_position);

            if let Some(piece_on_position) = piece_on_position {
                if piece_on_position.color == piece.color {
                    return false;
                }
            }

            // Check if we dont go out of pins
            if self.king_pins.len() > 0 {
                for pins in self.king_pins.iter() {
                    if pins & piece.position.bitboard() != 0 { // We are in a pin
                        if pins & move_to_position.bitboard() == 0 { // We are not in the direction of the pin
                            return true;
                        }
                    }
                }
            }

            // Check king movement
            if let PieceType::King = piece.piece_type {
                let last_rank = if let Color::White = piece.color { 0 } else { 7 };
                
                // Check castling
                if piece.position.get_row() == last_rank && piece.position.get_col() == 4 {
                    let is_castle = move_to_position.get_col() == 6 || move_to_position.get_col() == 2;
                    if is_castle && self.king_check != 0 {
                        return true;
                    }

                    if move_to_position.get_col() == 2 { // Queen side castle
                        if match piece.color { Color::White => !self.state.white_can_castle_queenside, Color::Black => !self.state.black_can_castle_queenside } {
                            return true;
                        }
                        if  self.is_position_attacked(piece.position.get_change(-1), piece.color) || 
                            self.is_position_attacked(piece.position.get_change(-2), piece.color) {
                            return true;
                        } 
                        // If something is in the way return
                        if  self.board.get_piece(piece.position.get_change(-1)).is_some() || 
                            self.board.get_piece(piece.position.get_change(-2)).is_some() ||
                            self.board.get_piece(piece.position.get_change(-3)).is_some(){
                            return true;
                        }
                        moves.push(Move {
                            from: piece.position,
                            to: move_to_position,
                            move_type: MoveType::QueenCastle
                        });
                        return true;
                    } else if move_to_position.get_col() == 6 { // King side castle
                        if match piece.color { Color::White => !self.state.white_can_castle_kingside, Color::Black => !self.state.black_can_castle_kingside } {
                            return true;
                        }

                        if self.is_position_attacked(piece.position.get_change(1), piece.color) || 
                            self.is_position_attacked(piece.position.get_change(2), piece.color) {
                            return true;
                        }
                        // If something is in the way return
                        if self.board.get_piece(piece.position.get_change(1)).is_some() || 
                            self.board.get_piece(piece.position.get_change(2)).is_some() {
                            return true;
                        }
                        moves.push(Move {
                            from: piece.position,
                            to: move_to_position,
                            move_type: MoveType::KingCastle
                        });
                        return true;
                    }
                }

                if self.is_position_attacked(move_to_position, piece.color) {
                    return true;
                }
            } else {
                // Check if we are in check and need to block
                if self.king_check != 0 {
                    // Check if the move is en passant capture of the checking piece
                    if let Some(en_passant_target) = self.state.en_passant_target {
                        if let PieceType::Pawn = piece.piece_type {
                            if move_to_position == en_passant_target {
                                let en_passant_pawn_position = match self.turn { Color::White => en_passant_target.get_change(-8), Color::Black => en_passant_target.get_change(8) };

                                if self.king_check & en_passant_pawn_position.bitboard() != 0 {
                                    moves.push(Move {
                                        from: piece.position,
                                        to: move_to_position,
                                        move_type: MoveType::EnPassantCapture
                                    });
                                    return true;
                                }
                            }
                        }
                    }
                    // We have a current check
                    if self.king_check & move_to_position.bitboard() == 0 {
                        // We are not in the direction of the check
                        return !self.board.has_piece(move_to_position);
                    }
                }
            }


            // All the pawn movement checking is done here
            if let PieceType::Pawn = piece.piece_type {
                // Check for pawn promotion
                if (piece.color == Color::White && move_to_position.get_row() == 7) || (piece.color == Color::Black && move_to_position.get_row() == 0) {
                    let is_capture_move = piece.position.get_col() != move_to_position.get_col();

                    if let Some(capture_piece) = self.board.get_piece(move_to_position) {
                        if is_capture_move {
                            if capture_piece.color == self.turn {
                                return true;
                            }
                        } else {
                            return true;
                        }
                    } else if is_capture_move {
                        return true;
                    }

                    moves.push(Move {
                        from: piece.position,
                        to: move_to_position,
                        move_type: if is_capture_move { MoveType::QueenPromotionCapture } else { MoveType::QueenPromotion }
                    });
                    moves.push(Move {
                        from: piece.position,
                        to: move_to_position,
                        move_type: if is_capture_move { MoveType::RookPromotionCapture } else { MoveType::RookPromotion }
                    });
                    moves.push(Move {
                        from: piece.position,
                        to: move_to_position,
                        move_type: if is_capture_move { MoveType::BishopPromotionCapture } else { MoveType::BishopPromotion }
                    });
                    moves.push(Move {
                        from: piece.position,
                        to: move_to_position,
                        move_type: if is_capture_move { MoveType::KnightPromotionCapture } else { MoveType::KnightPromotion }
                    });
                    return true;
                }
                // Pawn capture
                if piece.position.get_col() != move_to_position.get_col() {
                    if let Some(en_passant_target) = self.state.en_passant_target {
                        // Check if king is in check after en passant
                        if en_passant_target == move_to_position {
                            let en_passant_pawn_position = match self.turn { Color::White => en_passant_target.get_change(-8), Color::Black => en_passant_target.get_change(8) };
                            let friendly_king_position = match self.turn { Color::White => self.white_king_position, Color::Black => self.black_king_position };
                            let mut check_after_en_passant = false;
    
                            //println!("Checking after en passent: {:?} {:?} {:?} {:?}", piece.position.to_string(), move_to_position.to_string(), en_passant_pawn_position.to_string(), friendly_king_position.to_string());
                            
                            
                            let row_dif = en_passant_pawn_position.get_row() as i8 - friendly_king_position.get_row() as i8;
    
                            if row_dif == 0 {
                                let dir_index = get_direction_index(friendly_king_position, en_passant_pawn_position) as u8;
                                //println!("Checking row: {:?}", dir_index);
                                move_sliding_squares(friendly_king_position, (dir_index, dir_index + 1), &mut |pos, _| {
                                    if let Some(maybe_attacking_piece) = self.board.get_piece(pos) {
                                        // Ignore en passent and own pawn
                                        if maybe_attacking_piece.position == en_passant_pawn_position || maybe_attacking_piece.position == piece.position {
                                            return true;
                                        }
                                        if maybe_attacking_piece.color != self.turn {
                                            if let PieceType::Queen | PieceType::Rook = maybe_attacking_piece.piece_type {
                                                check_after_en_passant = true;
                                            }
                                        }
                                        return false;
                                    }
                                    return true;
                                });
                            } 
    
                            if check_after_en_passant {
                                return true;
                            }
    
    
                        
                            moves.push(Move {
                                from: piece.position,
                                to: move_to_position,
                                move_type: MoveType::EnPassantCapture
                            });
                            return true;
                        }
                    }
                    if self.board.get_piece(move_to_position).is_some() && self.board.get_piece(move_to_position).unwrap().color != piece.color {
                        moves.push(Move {
                            from: piece.position,
                            to: move_to_position,
                            move_type: MoveType::Capture
                        });
                        return true;
                    }
                    return true;
                } else {
                    // Moving forward
                    if self.board.has_piece(move_to_position) {
                        return false;
                    }
                    // if double pawn push
                    let move_type = if (piece.color == Color::White && piece.position.get_row() == 1 && move_to_position.get_row() == 3) || (piece.color == Color::Black && piece.position.get_row() == 6 && move_to_position.get_row() == 4) {
                        MoveType::DoublePawnPush
                    } else {MoveType::Quite};

                    moves.push(Move {
                        from: piece.position,
                        to: move_to_position,
                        move_type
                    });
                    return true;
                }
            }

            let piece_on_position = self.board.get_piece(move_to_position);
            if piece_on_position.is_some() {
                if piece_on_position.unwrap().color != piece.color {
                    moves.push(Move {
                        from: piece.position,
                        to: move_to_position,
                        move_type: MoveType::Capture
                    });
                }
                return false;
            }

            moves.push(Move {
                from: piece.position,
                to: move_to_position,
                move_type: MoveType::Quite
            });
            return true;
        });

        moves
    }

    pub fn is_position_attacked(&self, position : Position, color : Color) -> bool {
        if color != self.turn {
            println!("Cant evaluate if position is attacked if its not the turn of the color");
            return false;
        }
        if self.enemy_attacks & position.bitboard() != 0 {
            return true;
        }
        false
    }

    fn update_position(&mut self) {
        self.update_attacks();
        self.update_king_pins();
    }

    fn update_king_pins(&mut self) {
        (self.king_pins, self.king_check) = self.board.get_king_pins(self.turn);
        (self.enemy_king_pins, self.enemy_king_check) = self.board.get_king_pins(self.turn.opposite());
    }

    fn update_attacks(&mut self) {
        self.enemy_attacks = self.board.get_enemy_attacks(self.turn);
        self.friendly_attacks = self.board.get_enemy_attacks(self.turn.opposite());
    }


    pub fn from_fen(fen: &str) -> Self {
        let mut game = Game::default();
        let mut board = Board::new();
        let mut en_passent = None;

        let mut fen_iter = fen.split(' ');
        let board_fen = fen_iter.next().unwrap();
        let turn_fen = fen_iter.next().unwrap();
        let castle_fen = fen_iter.next().unwrap();
        let en_passent_fen = fen_iter.next().unwrap();
        let mut white_king_position = Position::from((4, 0));
        let mut black_king_position = Position::from((4, 7));

        let mut board_fen_iter = board_fen.split('/');

        for y in 0..8 {
            let board_fen_row = board_fen_iter.next().unwrap();
            let mut x = 0;
            for c in board_fen_row.chars() {
                if c.is_digit(10) {
                    x += c.to_digit(10).unwrap() as usize;
                } else {
                    let piece_type = match c {
                        'P' => PieceType::Pawn,
                        'N' => PieceType::Knight,
                        'B' => PieceType::Bishop,
                        'R' => PieceType::Rook,
                        'Q' => PieceType::Queen,
                        'K' => PieceType::King,
                        'p' => PieceType::Pawn,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'r' => PieceType::Rook,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        _ => panic!("Invalid character in fen string")
                    };
                    let color = if c.is_ascii_uppercase() { Color::White } else { Color::Black };
                    let position = Position::from((x as u8, 7 - y as u8));
                    if let PieceType::King = piece_type {
                        if let Color::White = color {
                            white_king_position = position;
                        } else {
                            black_king_position = position;
                        }
                    }
                    board.add_piece(Piece::new(color, piece_type, position));
                    x += 1;
                }
            }
        }

        if en_passent_fen != "-" {
            en_passent = Some(Position::from(en_passent_fen.to_string()));
        }

        game.board = board;
        game.turn = if turn_fen == "w" { Color::White } else { Color::Black };
        game.state.white_can_castle_kingside = castle_fen.contains('K');
        game.state.white_can_castle_queenside = castle_fen.contains('Q');
        game.state.black_can_castle_kingside = castle_fen.contains('k');
        game.state.black_can_castle_queenside = castle_fen.contains('q');
        game.state.en_passant_target = en_passent;
        game.white_king_position = white_king_position;
        game.black_king_position = black_king_position;

        game.update_position();

        game
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for y in 0..8 {
            let mut empty_count = 0;
            for x in 0..8 {
                let position = Position::from((x, 7 - y));
                let square = self.board.get_piece(position);
                if square.is_none() {
                    empty_count += 1;
                } else {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    let piece = square.unwrap();
                    let piece_char = match piece.piece_type {
                        PieceType::Pawn => 'P',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Rook => 'R',
                        PieceType::Queen => 'Q',
                        PieceType::King => 'K',
                    };
                    fen.push(if let Color::White = piece.color { piece_char } else { piece_char.to_ascii_lowercase() });
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            if y < 7 {
                fen.push('/');
            }
        }
        fen.push(' ');
        fen.push(if let Color::White = self.turn { 'w' } else { 'b' });
        fen.push(' ');
        
        let mut can_castle = false;

        if self.state.white_can_castle_kingside { can_castle = true; fen.push('K'); }
        if self.state.white_can_castle_queenside { can_castle = true; fen.push('Q'); }
        if self.state.black_can_castle_kingside { can_castle = true; fen.push('k'); }
        if self.state.black_can_castle_queenside { can_castle = true; fen.push('q'); }

        if !can_castle {
            fen.push('-');
        }

        fen.push(' ');
        if self.state.en_passant_target.is_none() {
            fen.push('-');
        } else {
            let position = self.state.en_passant_target.unwrap();
            let x = position.get_col();
            let y = position.get_row();
            fen.push((x as u8 + 'a' as u8) as char);
            fen.push((y as u8 + '1' as u8) as char);
        }

        fen.push_str(" 0 1");

        fen
    }

    fn move_to_pgn(&self, mov : Move) -> String {
        let mut pgn = String::new();
        let piece = self.board.get_piece(mov.from).unwrap();
        let piece_type = piece.piece_type;
        let mut is_capture = false;
        if let Some(_) = self.board.get_piece(mov.to) {
            is_capture = true;
        }
        if let PieceType::Pawn = piece_type {
            if is_capture {
                pgn.push((mov.from.get_col() as u8 + 'a' as u8) as char);
            }
            pgn.push((mov.to.get_col() as u8 + 'a' as u8) as char);
            pgn.push((mov.to.get_row() as u8 + '1' as u8) as char);
        } else {
            pgn.push(piece_type.get_char());
            if is_capture {
                pgn.push((mov.from.get_col() as u8 + 'a' as u8) as char);
            }
            pgn.push((mov.to.get_col() as u8 + 'a' as u8) as char);
            pgn.push((mov.to.get_row() as u8 + '1' as u8) as char);
        }
        pgn
    }

    pub fn fide_to_move(&self, fide : &str) -> Move {
        Move::invalid()
    }

    pub fn to_pgn(&self) -> String {
        todo!("Not Working");
        let mut pgn = String::new();
        let mut move_number = 1;
        for (i, m) in self.moves.iter().enumerate() {
            if i % 2 == 0 {
                pgn.push_str(&move_number.to_string());
                pgn.push('.');
                move_number += 1;
            }
            pgn.push_str(&self.move_to_pgn(*m));
            pgn.push(' ');
        }
        pgn
    }

    pub fn from_pgn(pgn : &str)-> Game {
        todo!("Not Working");
        let mut game = Game::from_fen(STARTING_POS_FEN);
        // Get the move list of the pgn string
        let moves = pgn.split(' ').collect::<Vec<&str>>();
        let mut in_comment = false;
        for mov in moves {
            if mov.len() == 0 {
                continue;
            }
            if mov.starts_with('{') {
                in_comment = true;
            }
            if in_comment {
                if mov.ends_with('}') {
                    in_comment = false;
                }
                continue;
            }
            let mut mov = mov.to_string();
            if mov.contains('.') {
                let dot_pos = mov.find('.').unwrap();
                mov = mov.chars().skip(dot_pos + 1).collect::<String>();
                if mov.is_empty() {
                    continue;
                }
            }
            println!("Move string \"{}\"", mov);
            //let mov = game.fide_to_move(&mov);
            //game.make_move(mov);
        }
        game
    }

}