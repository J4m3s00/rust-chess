use std::io;
use rand::Rng;

use crate::{moves::Move, game::Game};

pub trait Player {
    fn play(&self, game: &Game) -> Move;
}


pub struct HumanPlayer;
pub struct BotPlayer;

impl Player for HumanPlayer {
    fn play(&self, game: &Game) -> Move {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        Move::from_string(input.as_str())
    }
}

impl Player for BotPlayer {
    fn play(&self, game: &Game) -> Move {
        let moves = game.get_possible_team_moves(game.turn);
        // Get random move index
        let index = rand::thread_rng().gen_range(0..moves.len());
        moves[index]
    }
}