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
extern crate env_logger;
extern crate hashbrown;
extern crate log;
extern crate pathfinding;
extern crate rayon;
extern crate tiny_http;

mod analytics;
mod game;
mod profile;
mod routes;

use hashbrown::HashMap;
use log::{error, info};
use std::env;
use std::time::SystemTime;
use tiny_http::{Response, Server};

use analytics::Analytics;

#[allow(unused_imports)]
use profile::{Aggressive, AlphaBeta, AStarBasic, Cautious, Follow, NotSuck, Profile, Sim, Straight};

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    let port = match env::var("PORT") {
        Ok(v) => v,
        Err(_) => String::from("9000"),
    };

    env_logger::init();

    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();
    let mut profile = AlphaBeta::new();
    let mut analytics_profiles = HashMap::<String, Analytics>::new();

    info!("Battlesnake server running on port {}", port);
    info!("Profile set to {}", profile.get_status());

    for mut request in server.incoming_requests() {
        let start_time = SystemTime::now();
        let mut content = String::new();
        request.as_reader().read_to_string(&mut content).unwrap();

        let response;

        match request.url() {
            "/start" => {
                let res = routes::start_handler(&content, &mut profile, &mut analytics_profiles);
                response = Response::from_string(res);
            }
            "/move" => {
                let res = routes::move_handler(&content, &mut profile, &mut analytics_profiles);
                response = Response::from_string(res);
            }
            "end" => {
                info!("End of game");
                routes::end_handler(&content, &mut analytics_profiles);
                response = Response::from_string("OK");
            }
            _ => {
                response = Response::from_string("OK");
            }
        }

        match request.respond(response) {
            Ok(_) => {
                let end_time = start_time.elapsed().unwrap();
                info!(
                    "{} \u{b5}s {} ms",
                    end_time.as_micros(),
                    end_time.as_millis()
                );
            }
            Err(e) => {
                error!("Error occurred while responding to request: {}", e);
            }
        }
    }
}
