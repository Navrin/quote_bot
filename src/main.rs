#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![doc(html_root_url = "https://navrin.github.io/quote_bot/")]
#![recursion_limit="128"]

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
extern crate serenity;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate toml;
extern crate dotenv;
extern crate rand;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate typemap;
extern crate chrono;

use serenity::client::Client;
use serenity::framework::standard::help_commands;
use serenity::framework::standard::StandardFramework;

use std::io::prelude::*;
use std::fs::File;

pub mod config;
pub mod interactions;
pub mod db;

use db::Connector;
use config::Config;
use interactions::handler::Handler;
use interactions::commands::{command_from, invite_link, command_delete_quote};

fn main() {
    let mut raw_config = String::new();

    let mut file = File::open("./config.toml").unwrap();
    file.read_to_string(&mut raw_config).unwrap();


    let config: Config = toml::from_str(&raw_config).unwrap();

    let connector = Connector::new();

    let mut client = Client::new(&config.discord.token, Handler);
    {
        let mut data = client.data.lock();
        data.insert::<Connector>(connector);
    }

    client.with_framework(StandardFramework::new()
        .configure(|c| {
            c.on_mention(true)
             .prefix("!quote ")
             .allow_dm(false)
             .case_insensitivity(true)
        })
        .command("help", |c| c.exec_help(help_commands::with_embeds))
        .command("invite", |c| 
            c.desc("Get the invite link for this bot!")
             .example("!quote invite")
             .exec(invite_link)
        )
        .command("delete", |c| 
            c.desc("Delete a quote with it's ID.")
             .example("!quote delete 42")
             .exec(command_delete_quote)
        )
        .command("from", |c| {
            c.desc("Gets a command from the user, can be either `list`, `rand` or `contains`")
             .example("\n!quote from @user list amount = 10 page = 2 \n!quote from @user rand \n!quote from @user contains puppies")
             .guild_only(true)
             .exec(command_from)
        })
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
