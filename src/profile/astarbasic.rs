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

//! This module contains the AStarBasic algorithm & unit tests

use log::debug;
use pathfinding::prelude::astar;

use super::super::game::{Dir, Snake, State};
use super::Profile;

/// `AStarBasic` is a basic algorithm that will simply navigate
/// to the nearest food using the A* pathfinding algorithm.
/// If a path cannot be found, a safe move will be selected.
#[derive(Copy, Clone)]
pub struct AStarBasic {
    status: &'static str,
}

impl Profile for AStarBasic {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        if let Some(nearest_food) = s.nearest_food(&st) {
            let result = astar(
                &s.body[0],
                |p| p.successors(&s, &st),
                |p| p.manhattan(nearest_food),
                |p| *p == nearest_food,
            );

            if let Some((path, len)) = result {
                if len > 0 {
                    if let Some(dir) = s.body[0].dir_to(path[1]) {
                        return dir;
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

impl AStarBasic {
    #[allow(dead_code)]
    pub fn new() -> Self {
        debug!("AStarBasic profile initialized");
        Self {
            status: "AStarBasic",
        }
    }
}
