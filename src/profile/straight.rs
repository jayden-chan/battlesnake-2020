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

//! This module contains the Straight algorithm & unit tests

use log::debug;

use super::super::game::{Dir, SafetyIndex, Snake, State};
use super::Profile;

/// The Straight algorithm will go in a straight line until
/// it's unsafe to do so, at which point it will resort to any
/// safe move, then keep going straight.
#[derive(Copy, Clone)]
pub struct Straight {
    status: &'static str,
}

impl Profile for Straight {
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir {
        if let Some(d) = s.body[1].dir_to(s.body[0]) {
            if d.is_safety_index(&s, &st, &SafetyIndex::Safe) {
                return d;
            }
        }

        s.find_safe_move(&st)
    }

    fn get_status(&self) -> String {
        String::from(self.status)
    }
}

impl Straight {
    #[allow(dead_code)]
    pub fn new() -> Self {
        debug!("Straight profile initialized");
        Self { status: "Straight" }
    }
}
