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
use std::collections::{HashMap, HashSet};

use crate::game::{Dir, Point, State};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Future {
    /// Whether the protagonist snake is still alive
    pub alive: bool,
    /// Whether the protagonist snake has won
    pub finished: bool,
    /// The number of snakes that have died
    pub dead_snakes: u16,
    /// The number of foods collected by the protagonist
    pub foods: u16,
    /// The number of foods collected by enemies
    pub enemy_foods: u16,
    /// The starting direction of the future
    pub dir: Dir,
}

pub fn process_step(
    st: &mut State,
    self_id: &str,
    moves: &HashMap<String, Dir>,
) -> Future {
    let mut tmp_future = Future {
        alive: true,
        finished: false,
        dead_snakes: 0,
        foods: 0,
        enemy_foods: 0,
        dir: Dir::Up,
    };

    st.turn += 1;

    let mut results = HashMap::<String, Point>::with_capacity(moves.len());
    let mut eaten_foods = HashSet::new();

    for (id, dir) in moves {
        if *id == self_id {
            tmp_future.dir = *dir;
        }

        let snake = st.board.snakes.get_mut(id).unwrap();
        let (head, food_eaten) = snake.update_from_move(*dir, &st.board.food);

        if let Some(p) = food_eaten {
            if *id == self_id {
                tmp_future.foods += 1;
            } else {
                tmp_future.enemy_foods += 1;
            }

            // self.st.board.food.remove(&p);
            eaten_foods.insert(p);
        }

        results.insert(id.to_string(), head);
    }

    for (id, snake) in &st.board.snakes {
        if !results.contains_key(id) {
            results.insert(id.to_string(), snake.body[0]);
        }
    }

    for food in &eaten_foods {
        st.board.food.remove(&food);
    }

    let mut to_remove = Vec::new();

    for (id, head) in results {
        let snake = st.board.snakes.get(&id).unwrap();

        if !head.is_valid(snake, &st) || snake.health == 0 {
            if id == self_id {
                tmp_future.alive = false;
                tmp_future.finished = true;
            } else {
                tmp_future.dead_snakes += 1;
                to_remove.push(id);
            }
        }
    }

    for id in &to_remove {
        st.board.snakes.remove(id);
    }

    if !to_remove.is_empty() && st.board.snakes.len() == 1 {
        tmp_future.finished = true;
    }

    tmp_future
}
