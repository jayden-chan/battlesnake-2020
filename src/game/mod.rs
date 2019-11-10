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
mod dir;
mod point;
mod snake;

pub use dir::Dir;
pub use point::Point;
pub use snake::Snake;

use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

use super::routes::MoveRequest;

#[derive(Serialize, Debug, Clone)]
pub struct State {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub id: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Board {
    pub height: i8,
    pub width: i8,
    pub food: HashSet<Point>,
    pub snakes: HashMap<String, Snake>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Move {
    #[serde(rename = "move")]
    pub dir: &'static str,
}

#[derive(Debug, PartialEq)]
pub enum SafetyIndex {
    Safe,
    Risky,
    Unsafe,
}

// Util function for the unit tests
#[allow(dead_code)]
pub fn load_sample_data() -> Vec<(Snake, State)> {
    let paths = vec![
        "tests/scenario1.json",
        "tests/scenario2.json",
        "tests/scenario3.json",
        "tests/scenario4.json",
        "tests/scenario5.json",
        "tests/scenario6.json",
    ];

    let mut requests = Vec::new();

    for path in paths {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        let json = serde_json::from_reader::<BufReader<File>, MoveRequest>(reader).unwrap();

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

        requests.push((json.you, state));
    }

    requests
}
