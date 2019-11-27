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
use std::{clone::Clone, cmp::max, cmp::min};

const MAX: i16 = 1000;
const MIN: i16 = -1000;
const HEAD_ON: i16 = -500;
const MAX_DEPTH: u8 = 10;
///
/// This profile will be used in 1v1 situations. It implements MiniMax alpha beta pruning.
///
#[derive(Copy, Clone)]
pub struct AlphaBeta {
    status: &'static str,
}

impl Profile for AlphaBeta {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        if st.board.snakes.len() == 1 {
            panic!("Cannot initialize AlphaBeta with only 1 snake")
        };
        let self_id = &s.id;
        let mut enemy_id = String::from("Not Initalized");
        for (pos_id, _) in &st.board.snakes {
            if *pos_id != *self_id {
                enemy_id = pos_id.to_string();
            }
        }
        let (score, point) = self.minimax(self_id, &enemy_id, 1, st, true, MIN, MAX);
        if score > MIN {
            s.body[0].dir_to(point).unwrap()
        } else {
            s.find_safe_move(&st)
        }
    }

    fn get_status(&self) -> String {
        String::from(self.status)
    }
}

impl AlphaBeta {
    #[allow(dead_code)]
    pub fn new() -> Self {
        debug!("AlphaBeta profile initialized");
        Self {
            status: "AlphaBeta",
        }
    }
    /// This recursive function simulates our snake and the enemy snake taking turns, with the
    /// final nodes being the scores at the current board state.
    ///
    /// # Arguments
    /// `self_id` - The ID of the snake currently running this profile.
    /// `enemy_id` - The ID of the snake not running this profile. If there are more than two snakes it
    /// will be a random snake that is not running this profile.
    /// `depth` - The current recursive depth.
    /// `st` - The current state of the board which moves will be made from.
    /// `maximizing_player` - Boolean that is true when it is our turn and false when it is the enemies.
    /// `alpha` - The current best score attained anywhere in the tree
    /// `beta` - The current worst score found anywhere in the three.
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
            return (
                2 * self.get_flood_score(&st, self_id) - self.get_flood_score(&st, enemy_id),
                Point { x: 0, y: 0 },
            );
        }
        // Set the default score and best move
        let (temp_snake, mut best_score) = if maximizing_player {
            (st.board.snakes.get(self_id).unwrap(), MIN)
        } else {
            (st.board.snakes.get(enemy_id).unwrap(), MAX)
        };
        let mut best_move = Point { x: 0, y: 0 };
        let mut successors = temp_snake.body[0].successors(&temp_snake, &st);
        // Manually add our head back as a valid move for the enemy.
        if !maximizing_player {
            let self_head = st.board.snakes.get(self_id).unwrap().body[0];
            let orth = temp_snake.body[0].orthogonal();
            for i in 0..4 {
                if orth[i] == self_head {
                    successors.push((self_head, 0));
                }
            }
        }
        // Iterate through moves in our successors and call minimax for each
        for (pos_move, _) in successors {
            let dir = temp_snake.body[0].dir_to(pos_move).unwrap();
            let mut new_st = st.clone();

            if maximizing_player {
                let snake = new_st.board.snakes.get_mut(self_id).unwrap();
                // Update state with eaten food
                let (_, food_eaten) = snake.update_from_move(dir, &st.board.food);
                if let Some(p) = food_eaten {
                    new_st.board.food.remove(&p);
                }
                if snake.health == 0 {
                    continue;
                }
                let (val, _) =
                    self.minimax(self_id, enemy_id, depth + 1, &new_st, false, alpha, beta);
                if val > best_score {
                    best_move = pos_move;
                }
                // Updates the current available best move and prune.
                best_score = max(best_score, val);
                let new_alpha = max(alpha, best_score);

                if beta <= new_alpha {
                    break;
                }
            // Move for enemy snake
            } else {
                let snake = new_st.board.snakes.get_mut(enemy_id).unwrap();
                // Update state with eaten food
                let (_, food_eaten) = snake.update_from_move(dir, &st.board.food);
                if let Some(p) = food_eaten {
                    new_st.board.food.remove(&p);
                }

                // Deal with head on collisions
                let our_snake = st.board.snakes.get(self_id).unwrap();
                if our_snake.body[0] == pos_move {
                    if our_snake.body.len() > snake.body.len() {
                        continue;
                    } else {
                        return (HEAD_ON, best_move);
                    }
                }

                let (val, _) =
                    self.minimax(self_id, enemy_id, depth + 1, &new_st, true, alpha, beta);
                if val < best_score {
                    best_move = pos_move;
                }
                best_score = min(best_score, val);
                let new_beta = min(best_score, beta);

                if new_beta < alpha {
                    break;
                }
            }
        }
        (best_score, best_move)
    }

    fn get_flood_score(&self, st: &State, id: &str) -> (i16) {
        let s = st.board.snakes.get(id).unwrap();
        let len = s.body.len() as u16;
        let flood = s.body[0].flood_fill(s, st, len);
        let score = flood.len() as i16;
        return score;
    }
}
