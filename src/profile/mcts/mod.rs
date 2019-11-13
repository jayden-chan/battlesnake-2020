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

mod game_tree;

use game_tree::GameTree;

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

#[derive(Copy, Clone)]
pub struct MonteCarlo {
    status: &'static str,
}

impl Profile for MonteCarlo {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        let start_time = SystemTime::now();
        let mut tree = GameTree::new(st.clone(), s.id.clone());

        let mut curr = match tree.expand(0) {
            Some(id) => id,
            // We're dead, RIP
            None => return Dir::Up,
        };

        // Perform the Monte Carlo tree search until the time is up
        while start_time.elapsed().unwrap().as_millis() < SIM_TIME_MAX_MILLIS {
            if tree.node_is_leaf(curr) {
                if tree.node_has_sims(curr) {
                    tree.expand(curr);
                } else {
                    tree.rollout(curr);
                    curr = 0;
                }
            } else {
                curr = tree.next_node(curr);
            }
        }

        return tree.get_best_move();
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
