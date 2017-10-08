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
#[macro_use]
extern crate serenity;
extern crate toml;

use serenity::client::Client;
use serenity::framework::standard::StandardFramework;

use std::io::prelude::*;
use std::fs::File;

pub mod config;
pub mod interactions;
use config::Config;
use interactions::handler::Handler;

fn main() {
    let mut raw_config = String::new();

    let mut file = File::open("config.toml").unwrap();
    file.read_to_string(&mut raw_config).unwrap();


    let config: Config = toml::from_str(&raw_config).unwrap();

    let mut client = Client::new(&config.discord.token, Handler);

    println!("invite! https://discordapp.com/api/oauth2/authorize?client_id=366186820347625472&scope=bot&permissions=0");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
