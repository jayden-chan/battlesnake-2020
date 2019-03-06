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

//! This module contains the NotSuck algorithm & unit tests

use log::debug;

use super::super::game::{Dir, SafetyIndex, Snake, State};
use super::Profile;

/// `NotSuck` is an extremely basic algorithm that is designed
/// to not instantly commit suicide and to pursue the nearest
/// food using a very rudimentary direction-based algorithm.
/// It frequently leads itself into dead ends and kills itself
/// on its own tail, although it actually works amazingly well
/// given how simple it is. Mostly designed just to test some
/// of the basic util functions like `safety_index`, `dir_to`,
/// and `orthogonal`.
#[derive(Copy, Clone)]
pub struct NotSuck {
    status: &'static str,
}

impl Profile for NotSuck {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        if let Some(nearest_food) = s.nearest_food(&st) {
            if let Some(d) = s.body[0].dir_to(nearest_food) {
                if d.is_safety_index(&s, &st, &SafetyIndex::Safe) {
                    return d;
                }
            };
        }

        s.find_safe_move(&st)
    }

    fn get_status(&self) -> String {
        String::from(self.status)
    }

    fn init(&mut self, _st: &State, _self_id: String) {}
}

impl NotSuck {
    #[allow(dead_code)]
    pub fn new() -> Self {
        debug!("NotSuck profile initialized");
        Self { status: "NotSuck" }
    }
}
