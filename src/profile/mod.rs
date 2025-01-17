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
use super::game::{Dir, Snake, State};

mod aggressive;
mod alpha_beta;
mod astarbasic;
mod cautious;
mod follow;
mod mcts;
mod notsuck;
mod sim;
mod straight;

pub use aggressive::Aggressive;
pub use alpha_beta::AlphaBeta;
pub use astarbasic::AStarBasic;
pub use cautious::Cautious;
pub use follow::Follow;
pub use mcts::MonteCarlo;
pub use notsuck::NotSuck;
pub use sim::Sim;
pub use straight::Straight;

///
/// A profile is a unique algorithm that defines how the snake
/// will behave in game. Multiple profiles are required for use
/// by the simulator profile as well as others for predicting
/// the behavior of other snakes.
///
pub trait Profile {
    ///
    /// Setup the profile with the initial game state
    ///
    fn init(&mut self, _st: &State, _self_id: String) {}

    ///
    /// Update the game state and get the next move from the profile
    ///
    fn get_move(&mut self, s: &Snake, st: &State) -> Dir;

    ///
    /// Get the status of the profile
    ///
    fn get_status(&self) -> String;
}

pub fn string_to_profile(profile: &str) -> Box<dyn Profile> {
    match profile {
        "aggressive" => Box::new(Aggressive::new()),
        "alpha_beta" => Box::new(AlphaBeta::new()),
        "astarbasic" => Box::new(AStarBasic::new()),
        "cautious" => Box::new(Cautious::new()),
        "notsuck" => Box::new(NotSuck::new()),
        "sim" => Box::new(Sim::new()),
        "straight" => Box::new(Straight::new()),
        "follow" => Box::new(Follow::new()),
        "monte_carlo" => Box::new(MonteCarlo::new()),
        _ => panic!("Invalid string provided!"),
    }
}
