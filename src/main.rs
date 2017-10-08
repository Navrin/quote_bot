#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

//! # A discord quote bot!
//!
//! Quotes users in a star-bot like manner, without the obnoxious dedicated channel for messages.
//!
//! ## Usage
//!
//! ```plain
//!     Cool User 20519
//! []  (Insert quote worthy material here)
//!     [🌟 1]
//! ```
//!
//! Reacting a 🌟 will add a confirmation reaction and store that message in the Quote Collection™.
//!
//! To get a quote from a user
//!
//! ```plain
//! !quote from @User <type>
//! ```
//!
//! (As of now) there are **3** different types
//!
//! ```plain
//! rand                         // will get a random quote from the user.
//! contains "query" <skip = 0>  // searches for a quote that matches the query
//! list <amount = 3> <page = 1> // lists out quotes from the user
//! ```
//!
//! ### Example commands
//!
//! ```plain
//! !quote from @Bep rand
//!
//! !quote from @Bep contains "puppy"
//!
//! !quote from @Bep list amount = 3
//!
//! !quote from @Bep list amount = 5 page = 1
//! ```

#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::io::prelude::*;
use std::fs::File;

pub mod config;
use config::Config;

fn main() {
    let mut config = String::new();

    let mut file = File::open("example_config.toml").unwrap();
    file.read_to_string(&mut config).unwrap();


    let parsed_config: Config = toml::from_str(&config).unwrap();

    println!("Token: {}", parsed_config.discord.token);
    println!(
        "Postgres DB: postgres://{}:{}@{}/{}",
        parsed_config.database.name,
        parsed_config.database.password,
        parsed_config.database.location,
        parsed_config.database.name
    );
}
