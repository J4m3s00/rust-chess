use std::{io, option, time::Instant};

use board::Board;
use game::Game;
use moves::{Move, MoveType};
use base_types::{Color, Position};

use crate::precompute::NUM_SQUARES_TO_EDGE;

mod base_types;
mod precompute;
mod moves;
mod piece;
mod board;
mod game;


static starting_position_fen: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug)]
struct RunTestOptions {
    depth : u8,
    debug: bool,
    show_board: bool,
    show_moves: bool,
    show_time: bool
}

impl RunTestOptions {
    fn new(depth: u8) -> RunTestOptions {
        RunTestOptions {
            depth,
            debug: false,
            show_board: false,
            show_moves: false,
            show_time: false
        }
    }
}

enum BitboardType {
    ENEMY_ATTACK,
    ENEMY_PINS,
    ENEMY_CHECKS,
}
enum InputMessage {
    Move(Move),
    UndoMove,
    ShowMoves(Position),
    ShowTeam(Color),
    ShowBoard,
    LoadFen(String),
    RunTest(RunTestOptions),
    ShowFen,
    ShowBitboard(BitboardType),
    Quit,
    None
}

fn get_input() -> InputMessage {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    if input.len() == 0 {
        return InputMessage::None;
    }
    let input = input.trim();
    if input == "quit" || input == "q" {
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
        
        let depth = args[1].parse::<u8>().unwrap();
        let mut options = RunTestOptions::new(depth);
        
        args[2..].iter().for_each(|arg| {
            if *arg == "-d" {
                options.debug = true;
            } else if *arg == "-s" {
                options.show_board = true;
            } else if *arg == "-m" {
                options.show_moves = true;
            } else if *arg == "-t" {
                options.show_time = true;
            }
        });

        return InputMessage::RunTest(options);
    } else if args[0] == "bit" {
        // bit <type> - type is either epat (enemy_attack), epin (enemy_pins), echk (enemy_checks)
        if args.len() != 2 {
            return InputMessage::None;
        }
        let bit_type = args[1];
        if bit_type == "epat" {
            return InputMessage::ShowBitboard(BitboardType::ENEMY_ATTACK);
        } else if bit_type == "epin" {
            return InputMessage::ShowBitboard(BitboardType::ENEMY_PINS);
        } else if bit_type == "echk" {
            return InputMessage::ShowBitboard(BitboardType::ENEMY_CHECKS);
        } else {
            return InputMessage::None;
        }
    }
    return InputMessage::None;
}

fn print_help() {
    println!("Commands:");
    println!("m <move> - make a move");
    println!("s <position> - show possible moves for a piece");
    println!("s <color> - show all pieces for a team");
    println!("s - show the board");
    println!("fen - show the fen");
    println!("fen <fen> - load a fen");
    println!("um - undo a move");
    println!("rt <depth> -flags - run a perftest");
    println!("    -d - debug (show number of moves for each move)");
    println!("    -s - show board (show the board after each move)");
    println!("    -m - show moves (show the moves after each move)");
    println!("    -t - show time (show the time taken for each move)");
    println!("bit <type> - show a bitboard");
    println!("    type is either epat (enemy_attack), epin (enemy_pins), echk (enemy_checks)");
    println!("quit/q - quit");
}

fn print_moves(game: &Game, moves : &Vec<Move>) {
    game.board.print_custom(&|pos| -> char {

        if let Some(found_move) = moves.iter().find(|m| {m.to == pos}) {
            match found_move.move_type {
                MoveType::Capture => return 'c',
                MoveType::EnPassantCapture => return 'e',
                MoveType::KingCastle | MoveType::QueenCastle => return 'k',
                _ => {
                    if !game.board.has_piece(pos) {
                        return 'x';
                    }
                },
            }    
            
        }

        if let Some(piece) = game.board.get_piece(pos) {
            return piece.get_char();
        }

        return ' ';
    });
}

fn print_bitboard(game: &Game, bitboard_type : BitboardType) {
    let bitboard = match bitboard_type {
        BitboardType::ENEMY_ATTACK => game.enemy_attacks,
        BitboardType::ENEMY_PINS => game.king_pins.iter().fold(0,|a, b| { return a | *b; } ),
        BitboardType::ENEMY_CHECKS => game.king_check,
    };
    game.board.print_custom(&|pos| -> char {
        if bitboard & 1 << pos.index() != 0 {
            return 'x';
        }
        return ' ';
    });
}

fn run_test(game : &mut Game, options : RunTestOptions) -> usize {
    if options.depth == 0 {
        return 1;
    }

    let moves = game.get_possible_team_moves(game.turn);
    let mut count = 0;
    for m in moves.iter() {
        if options.show_moves {
            println!("Makeing move: {}", m.to_string());
        }
        game.make_move(m.from, m.to);
        if options.show_board {
            game.board.print();
        }

        let add = run_test(game, RunTestOptions{ depth: options.depth - 1, debug: false, ..options });

        if options.debug {
            println!("{}: {}", m.to_string(), add);
        }
        count += add;

        if options.show_moves {
            println!("Unmakeing move: {}", m.to_string());
        }
        game.unmake_move();
    }
    return count;
}



fn main(){

    let mut game = Game::from_fen(starting_position_fen);
    game.board.print();

    /*run_test(&mut game, RunTestOptions {
        depth: 3,
        debug: false,
        show_board: true,
        show_moves: true,
        show_time: false,
    });*/

    loop {
        let input = get_input();
        match input {
            InputMessage::ShowFen => {
                println!("{}", game.to_fen());
            }
            InputMessage::LoadFen(fen) => {
                game = Game::from_fen(&fen);
                game.board.print();
            }
            InputMessage::Move(mov) => {
                game.make_move(mov.from, mov.to);
                game.board.print();
            }
            InputMessage::UndoMove => { 
                game.unmake_move();
                game.board.print();
            }
            InputMessage::RunTest(options) => {
                // Time the test
                let start = Instant::now();
                let show_time = options.show_time;

                println!("Running test {:?}", options);
                println!("Starting fen: {}", game.to_fen());
                println!("------------------");
                println!("Result: {}", run_test(&mut game, options));

                if show_time {
                    let duration = start.elapsed();
                    println!("------------------");
                    println!("Test took: {}ms", duration.as_millis());
                    println!("------------------");
                }
            }
            InputMessage::ShowMoves(pos) => {
                if let Some(piece) = game.board.get_piece(pos) {
                    let moves = game.get_possible_piece_moves(piece);
                    println!("Possible moves for {:?}", moves);
                    print_moves(&game, &moves);
                }
            }
            InputMessage::ShowTeam(color) => {
                let moves = game.get_possible_team_moves(color);
                print_moves(&game, &moves);
            }
            InputMessage::ShowBitboard(bitboard_type) => {
                print_bitboard(&game, bitboard_type);
            }
            InputMessage::ShowBoard => game.board.print(),
            InputMessage::None => print_help(),
            InputMessage::Quit => break,
        }
    }
}