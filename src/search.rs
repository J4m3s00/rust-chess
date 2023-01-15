use crate::{moves::Move, game::Game};


pub struct SearchSettings {
    pub depth: u8,
    pub move_order: bool,
    pub show_log: bool,

    /**
     * Advanced settings. Should stay at default unless you know what you are doing.
     * They mess with the evaluation and move ordering functions.
     */
    pub move_on_attacked_penalty: i32,  // Penalty for moving on a square that is attacked by the opponent
    pub capture_multiplier: i32,        // Multiplier for captures
    pub castle_reword: i32,             // Reword for castling
    pub promotion_bonus: i32            // Bonus for promoting a pawn
}

impl Default for SearchSettings {
    fn default() -> Self {
        SearchSettings {
            depth: 4,
            move_order: true,
            show_log: false,
            move_on_attacked_penalty: 200,
            capture_multiplier: 10,
            castle_reword: 50,
            promotion_bonus: 25
        }
    }
}


pub struct Search<'a> {
    pub best_move: Move,
    pub moves_searched: u64,
    pub settings: SearchSettings,
    game: &'a mut Game
}

impl<'a> Search<'a> {
    pub fn new(game : &'a mut Game) -> Search {
        Search {
            best_move: Move::invalid(),
            moves_searched: 0,
            settings: Default::default(),
            game
        }
    }

    pub fn start(&mut self) -> Move {
        if self.settings.show_log {
            println!("---------------------------------");
            println!("Starting best move search!"); 
            println!("Team = {}", self.game.turn.to_string());
            println!("Depth = {}", self.settings.depth);
            println!("Move order enabled = {}", self.settings.move_order);
            if self.settings.move_order {
                println!("Move order settings:");
                println!("Move on attacked penalty = {}", self.settings.move_on_attacked_penalty);
                println!("Capture multiplier = {}", self.settings.capture_multiplier);
                println!("Castle reword = {}", self.settings.castle_reword);
                println!("Promotion bonus = {}", self.settings.promotion_bonus);
            }
            println!("Running Search...");
        }
        
        let start = std::time::Instant::now();
        self.search(0, self.settings.depth, -1000000, 1000000);

        if self.settings.show_log {
            println!("Searched {} moves in {}ms.", self.moves_searched, start.elapsed().as_millis());
            println!("---------------------------------");
        }

        self.best_move
    }
    
    fn search(&mut self, count_from_root : u8, depth: u8, alpha: i32, beta: i32) -> i32 {
        if depth == 0 {
            return self.game.get_score();
        }
        
        let mut alpha = alpha;

        let mut moves = self.game.get_possible_team_moves(self.game.turn);
        self.moves_searched += moves.len() as u64;

        if count_from_root == 0 && moves.len() > 0 {
            self.best_move = moves[0];
        }

        // Oder moves
        if self.settings.move_order {
            self.oder_moves(&mut moves);
        }

        for m in moves {
            self.game.make_move(m);
            let score = -self.search(count_from_root + 1, depth - 1, -beta, -alpha);
            self.game.unmake_move();

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
                if count_from_root == 0 {
                    self.best_move = m;
                }
            }
        }
        
        return alpha;
    }

    fn move_order_score(&self, m : Move) -> i32 {
        let mut score = 0;
        let moving_piece = self.game.board.get_piece(m.from).unwrap();

        if let Some(capture_piece) = self.game.board.get_piece(m.to) {
            score += self.settings.capture_multiplier * capture_piece.piece_type.get_value() - moving_piece.piece_type.get_value();
        }
        if m.move_type.is_promotion() {
            score += self.settings.promotion_bonus + m.move_type.get_promotion_piece().get_value()
        }
        if m.move_type.is_castle() {
            score += self.settings.castle_reword;
        }
        if self.game.is_position_attacked(m.to, moving_piece.color) {
            score -= self.settings.move_on_attacked_penalty;
        }
        score
    }

    pub fn oder_moves(&self, moves: &mut Vec<Move>) { 
        moves.sort_by(|a, b| self.move_order_score(*b).cmp(&self.move_order_score(*a)));
    }
}
