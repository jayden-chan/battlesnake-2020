/*
 * Copyright (C) 2019 Cobey Hollier. All rights reserved.
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

use log::debug;

use super::super::game::{Dir, Point, Snake, State};
use super::Profile;
use std::clone::Clone;

const MAX: i16 = 1000;
const MIN: i16 = -1000;
const MAX_DEPTH: u8 = 10;
///
///This profile will be used in 1v1 situations. It implements MiniMax alpha beta pruning.
///
#[derive(Copy, Clone)]
pub struct AlphaBeta {
    status: &'static str,
}

impl Profile for AlphaBeta {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        let self_id = &s.id;
        let mut enemy_id = String::from("Not Initalized");
        for (pos_id, _) in &st.board.snakes {
            if *pos_id != *self_id {
                enemy_id = pos_id.to_string();
            }
        }
        let (score, dir) = self.minimax(self_id, &enemy_id, 1, &mut st.clone(), true, MIN, MAX);
        return s.body[0].dir_to(dir).unwrap();
    }

    fn get_status(&self) -> String {
        String::from(self.status)
    }

    fn init(&mut self, _st: &State, _self_id: String) {}
}

impl AlphaBeta {
    #[allow(dead_code)]
    pub fn new() -> Self {
        debug!("AlphaBeta profile initialized");
        Self {
            status: "AlphaBeta",
        }
    }

    //This recursive function simulates our snake and the enemy snake taking turns, with the
    //final nodes being the scores at the current board states.
    //
    //'self_id' - The ID of the snake currently running this profile.
    //'enemy_id' - The ID of the snake not running this profile. If there are more than two snakes it
    //will be a random snake that is not running this profile.
    //'depth' - The current recursive depth.
    //'st' - The current state of the board which moves will be made from.
    //'maximizing_player' - Boolean that is true when it is our turn and false when it is the enemies.
    //'alpha' - The current best score attained anywhere in the tree
    //'beta' - The current worst score found anywhere in the three.
    fn minimax(
        &self,
        self_id: &str,
        enemy_id: &str,
        depth: u8,
        st: &State,
        maximizing_player: bool,
        alpha: i16,
        beta: i16,
    ) -> (i16, Point) {
        if depth > MAX_DEPTH {
            let snake = st.board.snakes.get(self_id).unwrap();
            let near_food = snake.nearest_food(&st);
            let score = match near_food {
                Some(near_food) => (100 - near_food.manhattan(snake.body[0])) as i16,
                None => 0,
            };
            return (score, Point { x: 0, y: 0 });
        }
        let temp_st = st.clone();
        //set up the values that the return will go into and set the snake from the state.
        let (temp_snake, mut best_score) = match maximizing_player {
            true => (st.board.snakes.get(self_id).unwrap(), MIN),
            false => (st.board.snakes.get(enemy_id).unwrap(), MAX),
        };
        let mut best_move = Point { x: 0, y: 0 };
        //Check each possible move and update our snake with that move.
        for (pos_move, _) in temp_snake.body[0].successors(&temp_snake, &temp_st) {
            let dir = temp_snake.body[0].dir_to(pos_move).unwrap();
            //create a new copy of the origonal state that will be modified with the possible move.
            let mut new_st = st.clone();

            if maximizing_player {
                let snake = new_st.board.snakes.get_mut(self_id).unwrap();
                let (_, _) = snake.update_from_move(dir, &st.board.food);
                if snake.health == 0 {
                    continue;
                }
                let (val, _) =
                    self.minimax(self_id, enemy_id, depth + 1, &new_st, false, alpha, beta);
                if val > best_score {
                    best_move = pos_move;
                }
                best_score = std::cmp::max(best_score, val);
                let new_alpha = std::cmp::max(alpha, best_score);

                if beta <= new_alpha {
                    break;
                }
            //If we are not the maximizing player than it must be our opponent.
            } else {
                let snake = new_st.board.snakes.get_mut(enemy_id).unwrap();
                let (_, _) = snake.update_from_move(dir, &st.board.food);
                let (val, _) =
                    self.minimax(self_id, enemy_id, depth + 1, &new_st, true, alpha, beta);
                if val < best_score {
                    best_move = pos_move;
                }
                best_score = std::cmp::min(best_score, val);
                let new_beta = std::cmp::min(best_score, beta);

                if new_beta <= alpha {
                    break;
                }
            }
        }
        return (best_score, best_move);
    }
}
