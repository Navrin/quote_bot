extern crate toml;

/// The config object for the operation of the bot.
#[derive(Deserialize)]
pub struct Config {
    /// Discord config object
    pub discord: Discord,
    /// Database config object
    pub database: Database,
}


/// Contains all fields related to discord, such as tokens.
#[derive(Deserialize)]
pub struct Discord {
    /// Discord bot token snowflake
    pub token: String,
}

/// Database login config for postgresql.
/// will be parsed into `postgres://username:password@location/name`
#[derive(Deserialize)]
pub struct Database {
    pub username: String,
    pub password: String,
    /// The IP location for the database
    pub location: String,
    /// The name of the database in psql
    pub name: String,
}
