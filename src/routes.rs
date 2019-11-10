/*
 * Copyright (C) 2019 Jayden Chan. All rights reserved.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
 *
 */
use log::{error, info, warn};
use serde_derive::Deserialize;
use std::collections::{HashMap, HashSet};

use std::env;

use super::analytics::Analytics;
use super::game::{Board, Dir, Game, Point, Snake, State};
use super::profile::{Profile, Sim};

#[derive(Deserialize, Debug)]
pub struct BoardJson {
    pub height: i8,
    pub width: i8,
    pub food: Vec<Point>,
    pub snakes: Vec<SnakeJson>,
}

#[derive(Deserialize, Debug)]
pub struct MoveRequest {
    pub game: Game,
    pub turn: u32,
    pub board: BoardJson,
    pub you: Snake,
}

#[derive(Deserialize, Debug)]
pub struct SnakeJson {
    pub id: String,
    pub name: String,
    pub health: u8,
    pub body: Vec<Point>,
}

/// Handle the /start POST request
pub fn start_handler(
    buffer: &str,
    profile: &mut impl Profile,
    analytics: &mut HashMap<String, Analytics>,
) -> String {
    let color = match env::var("COLOR") {
        Ok(v) => v,
        Err(_) => String::from("#DEA584"),
    };

    match parse_body(buffer) {
        Ok((you, state)) => {
            profile.init(&state, you.id);
            let mut new_analytic =
                Analytics::new(&state, &["cautious", "astarbasic", "aggressive"]);
            new_analytic.update_full_game(buffer);
            analytics.insert(state.game.id.clone(), new_analytic);
            format!("{{\"color\":\"{}\"}}", color)
        }
        Err(_) => format!("{{\"color\":\"{}\"}}", color),
    }
}

/// Handle the /move POST request
pub fn move_handler(
    buffer: &str,
    profile: &mut Sim,
    analytics: &mut HashMap<String, Analytics>,
) -> String {
    match parse_body(buffer) {
        Ok((you, state)) => {
            let this_analytics = analytics.get_mut(&state.game.id).unwrap();

            this_analytics.fire(&you.id, &state);
            this_analytics.update_full_game(buffer);
            profile.update_analytics(this_analytics.matches.clone());

            let dir = profile.get_move(&you, &state);

            info!("Move: {:?}", dir);
            serde_json::to_string(&dir.as_move()).unwrap()
        }
        Err(_) => serde_json::to_string(&Dir::Left.as_move()).unwrap(),
    }
}

/// Handle the /end POST request
pub fn end_handler(buffer: &str, analytics: &mut HashMap<String, Analytics>) {
    if let Ok((_, state)) = parse_body(buffer) {
        analytics.remove(&state.game.id);
    }
}

/// Parse the JSON from the request body, then return
/// our snake and the game state
fn parse_body(buffer: &str) -> Result<(Snake, State), String> {
    let json = serde_json::from_str::<MoveRequest>(buffer);
    match json {
        Ok(json) => {
            let mut foods = HashSet::<Point>::new();
            let mut snakes = HashMap::<String, Snake>::new();

            for food in &json.board.food {
                foods.insert(*food);
            }

            for snake_json in json.board.snakes {
                let snake = Snake {
                    id: snake_json.id.clone(),
                    health: snake_json.health,
                    body: snake_json.body,
                };

                if snake.body.len() < 3 {
                    return Err(String::from("Snake body not long enough!!"));
                }

                snakes.insert(snake_json.id, snake);
            }

            let board = Board {
                height: json.board.height,
                width: json.board.width,
                food: foods,
                snakes,
            };

            let state = State {
                game: json.game,
                turn: json.turn,
                board,
            };

            info!("Turn: {}", json.turn);
            Ok((json.you, state))
        }
        Err(e) => {
            error!("Error: {}", e);
            warn!("Request body: {}", buffer);
            Err(e.to_string())
        }
    }
}
