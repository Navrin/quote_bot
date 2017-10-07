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
//!

fn main() {
    println!("Hello, world!");
}
