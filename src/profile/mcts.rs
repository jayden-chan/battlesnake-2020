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

use log::debug;

use super::super::game::{Dir, Snake, State};
use super::Profile;

#[derive(Copy, Clone)]
pub struct MonteCarlo {
    status: &'static str,
}

impl Profile for MonteCarlo {
    fn get_move(&mut self, _s: &Snake, _st: &State) -> Dir {
        Dir::Up
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
