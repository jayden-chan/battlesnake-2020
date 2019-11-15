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
use rayon::prelude::*;

use crate::game::{Dir, Snake, State};
use crate::profile::Profile;
use std::time::SystemTime;

const SIM_TIME_MAX_MILLIS: u128 = 390;
const NUM_TREES: usize = 22;

#[derive(Copy, Clone)]
pub struct MonteCarlo {
    status: &'static str,
}

type TreeThread = (GameTree, usize);

impl Profile for MonteCarlo {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        let start_time = SystemTime::now();

        let mut enemy_id = String::from("F");
        for (pos_id, _) in &st.board.snakes {
            if *pos_id != s.id {
                enemy_id = pos_id.to_string();
            }
        }

        let mut starter_tree =
            GameTree::new(st.clone(), s.id.clone(), enemy_id);

        let curr = match starter_tree.expand(0) {
            Some(id) => id,
            // We're dead, RIP
            None => return Dir::Up,
        };

        let mut trees: Vec<TreeThread> = (0..NUM_TREES)
            .map(|_| (starter_tree.clone(), curr))
            .collect();

        // Perform the Monte Carlo tree search until the time is up
        while start_time.elapsed().unwrap().as_millis() < SIM_TIME_MAX_MILLIS {
            trees.par_iter_mut().for_each(|(tree, curr)| {
                if tree.node_is_leaf(*curr) {
                    if tree.node_has_sims(*curr) {
                        *curr = tree.expand(*curr).unwrap_or(0);
                    } else {
                        tree.rollout(*curr);
                        *curr = 0;
                    }
                } else {
                    *curr = tree.next_node(*curr);
                }
            });
        }

        // Merge the simulated trees
        let final_scores = trees
            .iter()
            .map(|(tree, _)| tree.root_child_scores())
            .fold(vec![], |acc, t| {
                let mut tmp_acc = acc;
                t.iter().enumerate().for_each(|(idx_1, (score, idx_2))| {
                    if tmp_acc.len() <= idx_1 {
                        tmp_acc.push((*score, *idx_2));
                    } else {
                        tmp_acc[idx_1].0 += score;
                    }
                });
                tmp_acc
            });

        return starter_tree.get_best_move(final_scores);
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
