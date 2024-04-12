use dotenv::dotenv;

use serde::Deserialize;
use serde::Serialize;

use crate::base_types::Color;
use crate::base_types::Position;
use crate::game::Game;
use crate::moves::Move;
use crate::player::BotPlayer;
use crate::player::Player;

pub struct Lichess<'a> {
    auth: String,
    client: reqwest::Client,
    game: &'a mut Game,
}

#[derive(Deserialize, Serialize, Debug)]
struct PlayerData {
    id: String,
    username: String,
    title: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct State {
    #[serde(rename = "type")]
    state_type: String,
    moves: String,
    status: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Challenge {
    id: String,
    #[serde(rename = "finalColor")]
    final_color: String,
}

pub struct LichessPlayer;

impl Player for LichessPlayer {
    fn play(&self, _: &mut Game) -> Move {
        Move::new(Position::new(0), Position::new(0))
    }
}

static BASE_URL: &str = "https://lichess.org/api";

impl<'a> Lichess<'a> {
    pub fn new(game: &'a mut Game) -> Lichess<'a> {
        dotenv().ok();
        let auth =
            "Bearer ".to_owned() + &std::env::var("LICHESS_TOK").expect("LICHESS_TOK not set");

        Lichess {
            auth,
            client: reqwest::Client::new(),
            game,
        }
    }

    pub async fn get_account(&self) -> Result<(), reqwest::Error> {
        let url = BASE_URL.to_string() + "/account";
        let response = self
            .client
            .get(url)
            .header("Authorization", self.auth.clone())
            .send()
            .await?;

        let result = response.text().await?;

        let json: PlayerData = serde_json::from_str(result.as_str()).unwrap();
        println!("{:?}", json);
        Ok(())
    }

    pub async fn get_challenge(&self) -> Result<Challenge, String> {
        let url = BASE_URL.to_string() + "/stream/event";
        let mut response = self
            .client
            .get(url)
            .header("Authorization", self.auth.clone())
            .send()
            .await
            .expect("Failed to send request");

        while let Some(chunk) = response.chunk().await.expect("Failed to read chunk") {
            let chunk = std::str::from_utf8(&chunk).unwrap();

            println!("Challenge chunk: {}", chunk);

            if chunk.len() < 5 {
                // We need some text for json
                continue;
            }

            let json: serde_json::Value = serde_json::from_str(chunk).unwrap();
            if json.is_object() {
                let event = json.get("type").unwrap().as_str().unwrap();
                if event == "challenge" {
                    let challenge: Challenge =
                        serde_json::from_value(json.get("challenge").unwrap().clone()).unwrap();
                    // Accept challenge
                    self.client
                        .post(BASE_URL.to_string() + "/challenge/" + &challenge.id + "/accept")
                        .header("Authorization", self.auth.clone())
                        .send()
                        .await
                        .expect("Failed to accept challenge");

                    //let accept_text = accept_response.text().await.expect("Failed to read response");
                    println!("Accept response: {:?}", challenge);
                    return Ok(challenge);
                }
                continue;
            }

            return Err("No challenge".to_string());
        }
        Err("Cant connect to event stream".to_string())
    }

    pub async fn stream_game(&mut self, chal: Challenge) -> Result<(), ()> {
        let url = BASE_URL.to_string() + "/bot/game/stream/" + &chal.id;
        let mut response = self
            .client
            .get(url)
            .header("Authorization", self.auth.clone())
            .send()
            .await
            .expect("Failed to send request");

        let challenger_team = if chal.final_color == "white" {
            Color::White
        } else {
            Color::Black
        };
        let player_white: &dyn Player = if let Color::White = challenger_team {
            &LichessPlayer
        } else {
            &BotPlayer
        };
        let player_black: &dyn Player = if let Color::Black = challenger_team {
            &LichessPlayer
        } else {
            &BotPlayer
        };

        let mut current_player: &dyn Player;
        loop {
            if let Some(chunk) = response.chunk().await.expect("Failed to read game chunk") {
                println!("Game chunk: {}", std::str::from_utf8(&chunk).unwrap());
                let chunk = std::str::from_utf8(&chunk).unwrap();
                if chunk.len() < 5 {
                    // We need some text for json
                    continue;
                }
                let chunk_json: serde_json::Value = serde_json::from_str(chunk).unwrap();
                if chunk_json.is_object() {
                    let state: State;
                    if chunk_json.get("state").is_some() {
                        state = serde_json::from_value(chunk_json.get("state").unwrap().clone())
                            .unwrap();
                    } else {
                        state = serde_json::from_value(chunk_json.clone()).unwrap();
                    }

                    let moves: Vec<&str> = state.moves.split_whitespace().collect();
                    if let Some(last) = moves.last() {
                        self.game.make_move(Move::from_string(*last));
                        self.game.board.print();
                    }

                    match self.game.turn {
                        Color::White => current_player = player_white,
                        Color::Black => current_player = player_black,
                    }

                    if self.game.turn == challenger_team {
                        continue;
                    }

                    let move_ = current_player.play(self.game);
                    if !move_.is_valid() {
                        println!("No more moves to make. Game over");
                        break;
                    }
                    let move_str = move_.to_string();
                    println!("Move: {}", move_str);
                    let url = BASE_URL.to_string() + "/bot/game/" + &chal.id + "/move/" + &move_str;
                    self.client
                        .post(url)
                        .header("Authorization", self.auth.clone())
                        .send()
                        .await
                        .expect("Failed to send move");
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}
