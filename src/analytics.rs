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
 */

//! This module runs analytics on the enemy snakes to try
//! and figure out what kind of moves they are likely to make
//! in the future.

use hashbrown::HashMap;
use log::info;

use super::game::{Dir, State};
use super::profile::{string_to_profile, Profile};

const MATCH_THRESH: usize = 9;
const MOVE_BUFFER_SIZE: usize = 10;

/// The Analytics struct holds information for the analyzer
/// as well as any matches it finds
pub struct Analytics {
    real_moves: HashMap<String, Vec<Dir>>,
    expected_moves: HashMap<String, HashMap<String, Vec<Dir>>>,
    pub matches: HashMap<String, String>,
    algs: HashMap<String, Box<Profile>>,
}

impl Analytics {
    /// Creates a new instance of the Analytics struct
    pub fn new(st: &State, algs: &[&'static str]) -> Self {
        let mut real_moves = HashMap::<String, Vec<Dir>>::new();
        let mut expected_moves = HashMap::<String, HashMap<String, Vec<Dir>>>::new();

        for (id, _) in &st.board.snakes {
            let mut alg_moves = HashMap::<String, Vec<Dir>>::new();

            real_moves.insert(id.clone(), vec![Dir::Up; MOVE_BUFFER_SIZE]);

            for alg in algs {
                alg_moves.insert(alg.to_string(), vec![Dir::Down; MOVE_BUFFER_SIZE]);
            }

            expected_moves.insert(id.clone(), alg_moves.clone());
        }

        let mut algs_map = HashMap::<String, Box<Profile>>::new();

        for alg in algs {
            algs_map.insert(alg.to_string(), string_to_profile(alg));
        }

        Self {
            real_moves,
            expected_moves,
            algs: algs_map,
            matches: HashMap::<String, String>::new(),
        }
    }

    /// Updates the analytics. This function will update the moves
    /// that the enemies made, compare them against the existing
    /// expected moves, and calculate the next set of expected moves.
    pub fn fire(&mut self, s_id: &str, st: &State) {
        // Update the real moves for each of the snakes
        for (id, s) in &st.board.snakes {
            if let Some(d) = s.body[1].dir_to(s.body[0]) {
                let entry = self.real_moves.get_mut(id).unwrap();

                entry.insert(0, d);
                entry.pop();
            }
        }

        // Check for matches
        for (snake_id, alg_map) in &self.expected_moves {
            if *snake_id == s_id {
                continue;
            }

            for (alg_id, exp_moves) in alg_map {
                let real_moves = self.real_moves.get(snake_id).unwrap();

                let mut match_score = 0;
                for (index, item) in real_moves.iter().enumerate() {
                    if exp_moves[index] == *item {
                        match_score += 1;
                    }
                }

                if match_score >= MATCH_THRESH {
                    info!("Matched snake as {} profile", alg_id);
                    self.matches.insert(snake_id.clone(), alg_id.clone());
                } else {
                    self.matches.remove(snake_id);
                }
            }
        }

        // Get the new expected moves for the next turn
        for (s_id, s) in &st.board.snakes {
            for (alg_id, alg) in &mut self.algs {
                let expected_move = alg.get_move(s, st);
                let move_map = self.expected_moves.get_mut(s_id).unwrap();

                let alg_vec = move_map.get_mut(alg_id).unwrap();
                alg_vec.insert(0, expected_move);
                alg_vec.pop();
            }
        }
    }
}
