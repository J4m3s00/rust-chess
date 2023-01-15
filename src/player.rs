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

fn search(game: &mut Game, depth: u8) -> i32 {
    if depth == 0 {
        return game.get_score();
    }

    let mut best_score = -1000;
    let moves = game.get_possible_team_moves(game.turn);
    for m in moves {
        game.make_move(m);
        let score = -search(game, depth - 1);
        if score > best_score {
            best_score = score;
        }
        game.unmake_move();
    }
    best_score
}

impl Player for BotPlayer {
    fn play(&self, game: &mut Game) -> Move {
        let moves = game.get_possible_team_moves(game.turn);
        if moves.len() == 0 {
            println!("No moves available");
            return Move::from_string("a1a1");
        }
        // Get random move index
        let mut best_move = moves[0];
        let mut best_score = -1000;
        for m in moves {
            game.make_move(m);
            let score = -search(game, 2);
            if score > best_score {
                best_score = score;
                best_move = m;
            }
            game.unmake_move();
        }
        return best_move;
    }
}