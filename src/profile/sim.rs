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

//! This module contains the Sim algorithm & unit tests

use crate::simulator::{process_step, Future};
use log::{debug, info, warn};
use rayon::prelude::*;
use std::collections::HashMap;

use std::cmp::Ordering;
use std::time::SystemTime;

use super::super::game::{Dir, SafetyIndex, Snake, State};
use super::{string_to_profile, Profile};

const SIM_TIME_MAX_MILLIS: u128 = 450;

/// The Simulation algorithm will simulate future game states
/// using some of the other profiles for the enemy snakes. After
/// simulating until we die or win the game, the profile will
/// choose the best move based on the simulation data.
pub struct Sim {
    status: &'static str,
    branches: Vec<SimBranch>,
    analytics: HashMap<String, String>,
}

struct SimBranch {
    self_controller: Box<dyn Profile>,
    enemy_controller: Box<dyn Profile>,
    self_prefix: Dir,
    enemy_prefix: Dir,
    state: State,
    futures: Vec<Future>,
    self_id: String,
}

unsafe impl Send for SimBranch {}
unsafe impl Sync for SimBranch {}

impl Profile for Sim {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        let start_time = SystemTime::now();
        let tmp_analytics = self.analytics.clone();

        self.branches.par_iter_mut().for_each(|b| {
            b.futures.clear();
            b.state = st.clone();
            b.self_id = s.id.clone();
        });

        self.branches.par_iter_mut().for_each(|b| {
            b.perform_prefix();
        });

        while start_time.elapsed().unwrap().as_millis() < SIM_TIME_MAX_MILLIS {
            self.branches
                .par_iter_mut()
                .filter(|b| match b.futures.last() {
                    Some(l) => l.alive && !l.finished,
                    None => true,
                })
                .for_each(|b| {
                    b.step(&tmp_analytics);
                });

            if !self.branches.iter().any(|b| match b.futures.last() {
                Some(l) => l.alive && !l.finished,
                None => true,
            }) {
                break;
            }
        }

        let scores = self.choose_dir(&s, &st);
        let all_dirs = [Dir::Down, Dir::Left, Dir::Right, Dir::Up];
        let mut scores_vec = Vec::with_capacity(4);

        for dir in &all_dirs {
            if let Some((score, len)) = scores.get(dir) {
                scores_vec.push((dir, score, len));
            }
        }

        scores_vec.sort_unstable_by(|a, b| {
            if a.1 < b.1 {
                Ordering::Greater
            } else if a.1 > b.1 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        'outer: for (idx, (dir, score, len)) in scores_vec.iter().enumerate() {
            if dir.is_safety_index(&s, &st, &SafetyIndex::Safe)
            // && !(!s.body[0].is_outer(&st) && dir.resulting_point(s.body[0]).is_outer(&st))
            {
                return **dir;
            }

            let mut idx_tmp = idx;
            while idx_tmp + 1 < scores_vec.len() {
                let (next_best_move, next_bext_score, next_best_len) = scores_vec[idx_tmp + 1];

                if next_best_move.is_safety_index(&s, &st, &SafetyIndex::Safe)
                    && *next_bext_score > **score - (**score / 2.5).abs()
                    && *next_best_len > **len - (**len / 2)
                    && !next_best_move.is_corner_risky(&s, &st)
                // && !(!s.body[0].is_outer(&st)
                //     && next_best_move.resulting_point(s.body[0]).is_outer(&st))
                {
                    warn!("SKIPPED MOVE {:?} AT RANK {}", dir, idx_tmp + 1);
                    continue 'outer;
                }

                idx_tmp += 1;
            }

            warn!(
                "NEXT BEST MOVES NOT GOOD ENOUGH, RETURNING RISKY MOVE OF RANK {:?}",
                idx + 1
            );

            return **dir;
        }

        s.find_safe_move(&st)
    }

    fn get_status(&self) -> String {
        String::from(self.status)
    }

    fn init(&mut self, st: &State, self_id: String) {
        let self_profiles = vec!["astarbasic", "cautious", "straight"];
        let enemy_profiles = vec!["astarbasic", "cautious", "straight", "aggressive"];
        let prefixes = vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right];

        let mut branches = Vec::new();

        for self_profile in &self_profiles {
            for enemy_profile in &enemy_profiles {
                for enemy_prefix in &prefixes {
                    for self_prefix in &prefixes {
                        branches.push(SimBranch {
                            self_controller: super::string_to_profile(self_profile),
                            enemy_controller: super::string_to_profile(enemy_profile),
                            self_prefix: *self_prefix,
                            enemy_prefix: *enemy_prefix,
                            state: st.clone(),
                            futures: Vec::new(),
                            self_id: self_id.clone(),
                        });
                    }
                }
            }
        }

        info!("Initialized {} simulation branches", branches.len());
        self.branches = branches;
    }
}

impl Sim {
    #[allow(dead_code)]
    pub fn new() -> Self {
        debug!("Sim profile initialized");

        Self {
            status: "Sim",
            branches: Vec::new(),
            analytics: HashMap::<String, String>::new(),
        }
    }

    pub fn update_analytics(&mut self, analytics: HashMap<String, String>) {
        self.analytics = analytics;
    }

    fn choose_dir(&self, s: &Snake, st: &State) -> HashMap<Dir, (f64, usize)> {
        let mut scores: HashMap<Dir, (f64, usize)> = HashMap::with_capacity(4);

        for branch in &self.branches {
            let mut dead: f64 = 0.0;
            let mut foods: f64 = 0.0;
            let dir = branch.futures[0].dir;

            let future_length = branch.futures.len();

            for future in &branch.futures {
                if future.alive {
                    dead += future.dead_snakes as f64;
                }

                foods += future.foods as f64;
            }

            let length_score = ((future_length as f64) - 30.0) * 1.5;
            let death_score = dead * 30.0;

            let food_score = if st.board.snakes.len() == 2
                && st
                    .board
                    .snakes
                    .iter()
                    .any(|(id, sn)| *id != s.id && sn.body.len() >= s.body.len() - 2)
            {
                (foods * 300.0)
            } else if st.board.snakes.len() == 1 {
                0.0
            } else {
                (foods * 1.7)
            };

            let mut total = length_score + death_score + food_score;

            if let Some(last_future) = branch.futures.last() {
                if last_future.finished && last_future.alive && future_length < 100 {
                    total += (100.0 - future_length as f64) * 5.0;
                }
            }

            if !s.body[0].is_outer(&st) && dir.resulting_point(s.body[0]).is_outer(&st) {
                total *= 0.8;
            }

            debug!(
                "Future length: {:04} Foods: {:02} First move: {:?}",
                future_length, foods, dir
            );

            if let Some((score, len)) = scores.get_mut(&dir) {
                *score += total;
                *len += future_length;
            } else {
                scores.insert(dir, (total, future_length));
            }
        }

        scores
    }
}

impl SimBranch {
    fn perform_prefix(&mut self) {
        let mut dirs = HashMap::<String, Dir>::with_capacity(self.state.board.snakes.len());

        for (id, _) in &self.state.board.snakes {
            let dir = if *id == self.self_id {
                self.self_prefix
            } else {
                self.enemy_prefix
            };

            dirs.insert(id.to_string(), dir);
        }

        let new_future = process_step(&mut self.state, &self.self_id, &dirs);
        self.futures.push(new_future);
    }

    fn step(&mut self, analytics: &HashMap<String, String>) {
        let mut dirs = HashMap::<String, Dir>::new();

        for (id, snake) in &self.state.board.snakes {
            let dir = if *id == self.self_id {
                self.self_controller.get_move(&snake, &self.state)
            } else if let Some(s) = analytics.get(id) {
                let mut profile = string_to_profile(&s);
                profile.get_move(&snake, &self.state)
            } else {
                self.enemy_controller.get_move(&snake, &self.state)
            };

            dirs.insert(id.to_string(), dir);
        }

        let new_future = process_step(&mut self.state, &self.self_id, &dirs);
        self.futures.push(new_future);
    }
}
