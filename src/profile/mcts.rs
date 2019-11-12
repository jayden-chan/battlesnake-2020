/*
 * Copyright (C) 2019 Jayden Chan, Cobey Hollier. All rights reserved.
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

use indextree::Arena;
use log::debug;
use std::collections::HashMap;
use std::f32;

use crate::game::{Dir, Snake, State};
use crate::profile::Profile;
use crate::simulator::{process_step, Future};
use std::time::SystemTime;

const SIM_TIME_MAX_MILLIS: u128 = 450;
const WINNING_SCORE: u8 = 1;
const LOSING_SCORE: u8 = 0;

#[derive(Clone)]
pub struct MCState {
    pub state: State,
    pub score: usize,
    pub sim_count: usize,
}

#[derive(Copy, Clone)]
pub struct MonteCarlo {
    status: &'static str,
}

impl MCState {
    pub fn ucb_one(&self, N: usize) -> f32 {
        (self.score as f32 / self.sim_count as f32)
            + 2.0 * f32::sqrt(f32::ln(N as f32) / self.sim_count as f32)
    }
}

impl Profile for MonteCarlo {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        let start_time = SystemTime::now();
        let root = MCState {
            state: st.clone(),
            score: 0,
            sim_count: 0,
        };

        let tree = &mut Arena::new();
        let mut curr_node_id = tree.new_node(root);
        let mut N = 0;

        while start_time.elapsed().unwrap().as_millis() < SIM_TIME_MAX_MILLIS {
            let curr_node = tree.get(curr_node_id).unwrap();
            if curr_node.first_child().is_none() {
                let inner = curr_node.get();
                if inner.sim_count == 0 {
                    // rollout
                } else {
                    for dir in
                        get_children_dirs(inner.state.board.snakes.get(&s.id).unwrap()).iter()
                    {
                        let mut new_state = inner.state.clone();
                        let mut moves = HashMap::with_capacity(1);
                        moves.insert(s.id.clone(), *dir);
                        process_step(&mut new_state, &s.id, &moves);

                        let new = tree.new_node(MCState {
                            state: new_state,
                            score: 0,
                            sim_count: 0,
                        });

                        new.append(curr_node_id, tree);
                    }
                }
            } else {
                let mut max_score = 0.0;
                let mut new_curr_id = None;
                let mut curr = curr_node.first_child();

                while curr.is_some() {
                    let id = curr.unwrap();
                    let curr_node = tree.get(id).unwrap();
                    let curr_score = curr_node.get().ucb_one(N);
                    if curr_score > max_score {
                        max_score = curr_score;
                        new_curr_id = Some(id);
                    }

                    curr = curr_node.next_sibling();
                }

                curr_node_id = new_curr_id.unwrap();
            }
        }

        return Dir::Up;
    }

    fn get_status(&self) -> String {
        String::from(self.status)
    }
}

impl MonteCarlo {
    #[allow(dead_code)]
    pub fn new() -> Self {
        debug!("MonteCarlo profile initialized");
        Self {
            status: "MonteCarlo",
        }
    }
}

fn get_children_dirs(s: &Snake) -> [Dir; 3] {
    match s.body[0].dir_to(s.body[1]).unwrap() {
        Dir::Down => [Dir::Up, Dir::Left, Dir::Right],
        Dir::Up => [Dir::Down, Dir::Left, Dir::Right],
        Dir::Right => [Dir::Up, Dir::Left, Dir::Down],
        Dir::Left => [Dir::Up, Dir::Down, Dir::Right],
    }
}
