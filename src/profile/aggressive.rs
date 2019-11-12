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

//! This module contains the Aggressive algorithm & unit tests

use log::debug;
use pathfinding::prelude::astar;

use super::super::game::{Dir, SafetyIndex, Snake, State};
use super::Profile;

/// `Aggressive` is a basic algorithm that will simply navigate
/// to the nearest snake's head using the A* pathfinding algorithm.
/// If a path cannot be found, a safe move will be selected.
#[derive(Copy, Clone)]
pub struct Aggressive {
    status: &'static str,
}

impl Profile for Aggressive {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        if let Some(nearest_snake) = s.nearest_snake(&st) {
            if nearest_snake.body.len() < s.body.len() {
                let dest_point = nearest_snake
                    .find_safe_move(st)
                    .resulting_point(nearest_snake.body[0]);
                let result = astar(
                    &s.body[0],
                    |p| p.successors(&s, &st),
                    |p| p.manhattan(dest_point),
                    |p| *p == dest_point,
                );

                if let Some((path, len)) = result {
                    if len > 0 {
                        if let Some(dir) = s.body[0].dir_to(path[1]) {
                            if dir.is_safety_index(&s, &st, &SafetyIndex::Safe) {
                                return dir;
                            }
                        }
                    }
                }
            }
        }

        s.find_safe_move(&st)
    }

    fn get_status(&self) -> String {
        String::from(self.status)
    }
}

impl Aggressive {
    #[allow(dead_code)]
    pub fn new() -> Self {
        debug!("Aggressive profile initialized");
        Self {
            status: "Aggressive",
        }
    }
}
