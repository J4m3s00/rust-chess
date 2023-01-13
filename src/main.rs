use std::io;

use board::Board;
use game::Game;
use moves::Move;
use base_types::{Color, Position};

mod base_types;
mod precompute;
mod moves;
mod piece;
mod board;
mod game;


static starting_position_fen: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

enum InputMessage {
    Move(Move),
    UndoMove,
    ShowMoves(Position),
    ShowTeam(Color),
    ShowBoard,
    LoadFen(String),
    RunTest(u8, bool),
    ShowFen,
    Quit,
    None
}

fn get_input() -> InputMessage {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    if input == "quit" || input == "q" {
        return InputMessage::Quit;
    }
    if input.len() == 0 {
        return InputMessage::Quit;
    }

    let args: Vec<&str> = input.split(" ").collect();
    if args[0] == "m" {
        if args.len() != 2 {
            return InputMessage::None;
        }
        

        return InputMessage::Move(Move::from_string(args[1]));
    } else if args[0] == "s" {
        if args.len() != 2 {
            return InputMessage::ShowBoard;
        }
        if args[1].to_uppercase() == "BLACK" || args[1].to_uppercase() == "WHITE" {
            let team = if args[1].to_uppercase() == "BLACK" { Color::Black } else { Color::White };
            return InputMessage::ShowTeam(team);
        } else {
            let position = Position::from(args[1].to_string());
            return InputMessage::ShowMoves(position);
        }
    } else if args[0] == "fen" {
        if args.len() == 1 {
            return InputMessage::ShowFen;
        }
        return InputMessage::LoadFen(input[4..].to_string());
    } else if args[0] == "um" {
        return InputMessage::UndoMove;
    } else if args[0] == "rt" {
        if args.len() < 2 {
            return InputMessage::None;
        }
        let test_number = args[1].parse::<u8>().unwrap();
        let debug = if args.len() == 3 {args[2].parse::<bool>().unwrap()} else { false };
        return InputMessage::RunTest(test_number, debug);
    }
    return InputMessage::None;
}
fn main(){
    let mut game : Game = Default::default();
    game.from_fen(starting_position_fen);
    game.board.print();

    loop {
        let input = get_input();
        match input {
            InputMessage::ShowFen => {
                println!("{}", game.to_fen());
            }
            InputMessage::LoadFen(fen) => {
                game.from_fen(fen.as_str());
            }
            InputMessage::Move(mov) => {
                game.make_move(mov);
                game.board.print();
            }
            InputMessage::ShowMoves(pos) => {
                if let Some(piece) = game.board.get_piece(pos) {
                    let moves = game.get_possible_piece_moves(piece);
                    game.board.print_custom(&|pos| -> char {
                        if let Some(piece) = game.board.get_piece(pos) {
                            return piece.get_char();
                        }
                        if let Some(found_move) = moves.iter().find(|m| {m.to == pos}) {
                            return 'x';
                        }
                        return ' ';
                    });
                }
            }
            InputMessage::ShowTeam(color) => {
                let moves = game.get_possible_team_moves(color);
                game.board.print_custom(&|pos| -> char {
                    if let Some(piece) = game.board.get_piece(pos) {
                        return piece.get_char();
                    }

                    if let Some(found_move) = moves.iter().find(|m| {m.to == pos}) {
                        return 'x';
                    }
                    return ' ';
                });
            }
            InputMessage::ShowBoard => game.board.print(),
            InputMessage::Quit => break,
            _ => ()
        }
    }
}