use std::io;

use crate::{moves::Move, game::Game, search::Search};

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

impl Player for BotPlayer {
    fn play(&self, game: &mut Game) -> Move {
        let moves = game.get_possible_team_moves(game.turn);
        if moves.len() == 0 {
            println!("No moves available");
            return Move::invalid();
        }
        
        let mut search = Search::new(game);
        let result = search.start();
        println!("Bot moves: {}", result.to_string());
        result
    }
}