use std::{io, time::Instant};

use game::Game;
use moves::{Move, MoveType};
use base_types::{Color, Position};
use player::{HumanPlayer, BotPlayer, Player};

mod base_types;
mod precompute;
mod moves;
mod piece;
mod board;
mod game;
mod player;
mod lichess;


static STARTING_POS_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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
    EnemyAttack,
    EnemyPins,
    EnemyChecks,
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
    StartGame,
    LichessChallenge,
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
            return InputMessage::ShowBitboard(BitboardType::EnemyAttack);
        } else if bit_type == "epin" {
            return InputMessage::ShowBitboard(BitboardType::EnemyPins);
        } else if bit_type == "echk" {
            return InputMessage::ShowBitboard(BitboardType::EnemyChecks);
        } else {
            return InputMessage::None;
        }
    } else if args[0] == "start" {
        return InputMessage::StartGame;
    } else if args[0] == "lichess" {
        return InputMessage::LichessChallenge;
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
    println!("start - start a game (human (white) vs computer (black)");
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
        BitboardType::EnemyAttack => game.enemy_attacks,
        BitboardType::EnemyPins => game.king_pins.iter().fold(0,|a, b| { return a | *b; } ),
        BitboardType::EnemyChecks => game.king_check,
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
        game.make_move(*m);
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



#[tokio::main]
async fn main() {


    let mut game = Game::from_fen(STARTING_POS_FEN);
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
            InputMessage::StartGame => {
                println!("Starting game");
                let players = (HumanPlayer, BotPlayer);
                loop {
                    let player: &dyn Player = if game.turn == Color::White { &players.0 } else { &players.1 };
                    let mut mov = player.play(&mut game);
                    while game.make_move(mov) == false {
                        println!("Invalid move!");
                        mov = player.play(&mut game);
                    }
                    game.board.print();
                    if game.get_possible_team_moves(game.turn).len() == 0 {
                        println!("Checkmate!");
                        println!("Winner: {:?}", game.turn.opposite());
                        break;
                    }
                }
            }
            InputMessage::LichessChallenge => {
                let mut online_bot = lichess::Lichess::new(&mut game);    
                online_bot.get_account().await.expect("Failed to get account");
                let challenge = online_bot.get_challenge().await.expect("Failed to get challenger"); 
                online_bot.stream_game(challenge).await.expect("Failed to stream game");
            }
            InputMessage::ShowFen => {
                println!("{}", game.to_fen());
            }
            InputMessage::LoadFen(fen) => {
                game = Game::from_fen(&fen);
                game.board.print();
            }
            InputMessage::Move(mov) => {
                println!("Making move: {}", mov.to_string());
                game.make_move(mov);
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