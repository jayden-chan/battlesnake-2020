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

use log::{debug, info};

use crate::game::{Dir, Snake, State};
use crate::profile::Profile;
use std::time::SystemTime;

const SIM_TIME_MAX_MILLIS: u128 = 350;

#[derive(Copy, Clone)]
pub struct MonteCarlo {
    status: &'static str,
}

impl Profile for MonteCarlo {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        let start_time = SystemTime::now();

        let mut enemy_id = String::from("F");
        for (pos_id, _) in &st.board.snakes {
            if *pos_id != s.id {
                enemy_id = pos_id.to_string();
            }
        }

        let mut tree = GameTree::new(st.clone(), s.id.clone(), enemy_id);

        let mut curr = match tree.expand(0) {
            Some(id) => id,
            // We're dead, RIP
            None => return Dir::Up,
        };

        // Perform the Monte Carlo tree search until the time is up
        while start_time.elapsed().unwrap().as_millis() < SIM_TIME_MAX_MILLIS {
            debug!("inside while loop");
            if tree.node_is_leaf(curr) {
                debug!("node is leaf");
                if tree.node_has_sims(curr) {
                    debug!("node has sims");
                    curr = tree.expand(curr).unwrap_or(0);
                } else {
                    debug!("node does not have sims");
                    tree.rollout(curr);
                    curr = 0;
                }
            } else {
                debug!("node is internal");
                curr = tree.next_node(curr);
            }
        }

        debug!("returning best move");
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
