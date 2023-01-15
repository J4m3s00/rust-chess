use std::io;

use crate::{moves::Move, game::Game};

pub trait Player {
    fn play(&self, game: &mut Game) -> Move;
}


pub struct HumanPlayer;
pub struct BotPlayer;

impl Player for HumanPlayer {
    fn play(&self, _: &mut Game) -> Move {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        Move::from_string(input.as_str())
    }
}

struct Search<'a> {
    best_move: Move,
    moves_searched: u64,
    game: &'a mut Game
}

impl<'a> Search<'a> {
    fn new(game : &'a mut Game) -> Search {
        Search {
            best_move: Move::invalid(),
            moves_searched: 0,
            game
        }
    }

    fn start(&mut self, depth: u8) -> Move {
        self.search(0, depth, -1000000, 1000000);

        println!("Searched {} moves", self.moves_searched);
        self.best_move
    }
    
    fn search(&mut self, count_from_root : u8, depth: u8, alpha: i32, beta: i32) -> i32 {
        if depth == 0 {
            return self.game.get_score();
        }
        
        let mut alpha = alpha;

        let moves = self.game.get_possible_team_moves(self.game.turn);
        self.moves_searched += moves.len() as u64;

        if count_from_root == 0 && moves.len() > 0 {
            self.best_move = moves[0];
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
}


impl Player for BotPlayer {
    fn play(&self, game: &mut Game) -> Move {
        let moves = game.get_possible_team_moves(game.turn);
        if moves.len() == 0 {
            println!("No moves available");
            return Move::invalid();
        }
        
        let mut search = Search::new(game);
        let result = search.start(4);
        println!("Bot moves: {}", result.to_string());
        result
    }
}