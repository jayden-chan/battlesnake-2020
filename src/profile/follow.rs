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

//! This module contains the Follow algorithm & unit tests

use log::debug;
use pathfinding::prelude::astar;

use super::super::game::{Dir, Snake, State};
use super::Profile;

/// `Follow` is an algorithm that will follow the tail of an enemy snake.
#[derive(Copy, Clone)]
pub struct Follow {
    status: &'static str,
}

impl Profile for Follow {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        if let Some(enemy) = s.nearest_snake(&st) {
            let len = enemy.body.len();
            let result = astar(
                &s.body[0],
                |p| p.successors(&s, &st),
                |p| p.manhattan(enemy.body[len - 1]),
                |p| *p == enemy.body[len - 1],
            );

            if let Some((path, _)) = result {
                if path.len() > 1 {
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

    fn init(&mut self, _st: &State, _self_id: String) {}
}

impl Follow {
    #[allow(dead_code)]
    pub fn new() -> Self {
        debug!("Follow profile initialized");
        Self { status: "Follow" }
    }
}
