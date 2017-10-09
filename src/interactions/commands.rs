use db::Connector;
use db::actions::*;
use db::models::*;

use serenity::builder::CreateEmbedField;
use serenity::prelude::*;
use serenity::model::{Message, User, Embed};
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError;

const DEFAULT_DISCORD_AVATAR: &str = "http://is1.mzstatic.com/image/thumb/Purple118/v4/98/0d/81/980d8181-c84b-4c21-cef8-126464197968/source/300x300bb.jpg";

pub fn command_from(
    ctx: &mut Context,
    message: &Message,
    mut args: Args,
) -> Result<(), CommandError> {
    let author = match args.single::<User>() {
        Ok(v) => v,
        Err(_) => {
            let _ = message.reply("No user was mentioned? Does the void make quotes? pls tell");
            return Ok(());
        }
    };

    let branch = match args.single::<String>() {
        Ok(v) => v,
        Err(_) => {
            let _ = message.reply("No find types were specified! Consider revising the help list.");
            return Ok(());
        }
    };

    let result = match branch.to_lowercase().as_str() {
        "rand" => rand_quote(ctx, message, &author, args),
        "contains" => contains_quotes(ctx, message, &author, args),
        "list" => list_quotes(ctx, message, &author, args),
        _ => { 
            message.reply("This find type doesn't exist! Consider revising the help list.")?;
            return Ok(());
        }
    };

    match result { 
        Ok(_) => return Ok(()),
        Err(e) => { 
            message.channel().ok_or("No channel here?!")?.say(&format!("Error while getting quotes: {}", e))?;
            return Ok(())
        }
    }
}

fn rand_quote(ctx: &mut Context, message: &Message, author: &User, mut args: Args) -> Result<(), String> {
    let data = ctx.data.lock();
    let connector = data.get::<Connector>().unwrap();
    let conn = connector
        .get_conn()
        .map_err(|_| "Could not get a connection to the db!")?;

    let guild_id = message
        .guild_id()
        .ok_or("This isn't a real channel. You're not real.")?;

    let quotes = find_rand_quote(&conn, &author.id.to_string(), &guild_id.to_string())
        .map_err(|_| "Could not read the quote database.")?
        .ok_or("No quotes found! :(")?;

    message
        .channel()
        .ok_or("This channel is fake!")?
        .send_message(|reply| reply.embed(|f| 
            f.thumbnail(&author.avatar_url().unwrap_or(DEFAULT_DISCORD_AVATAR.to_string()))
             .title(format!("A random quote from the lovely {}.", author.name))
             .fields(create_quote_embed_section(vec![quotes]))
        ))
        .map_err(|e| format!("Could not send message! Error: {:?}", e))?;
    
    Ok(())
}

fn contains_quotes(ctx: &mut Context, message: &Message, author: &User, mut args: Args) -> Result<(), String> {
    Ok(())
}

fn list_quotes(ctx: &mut Context, message: &Message, author: &User, mut args: Args) -> Result<(), String> {
    Ok(())
}


fn create_quote_embed_section(quotes: Vec<Quote>) -> Vec<CreateEmbedField> {
    quotes
        .iter()
        .map(|quote| {
            CreateEmbedField::default().value(quote.quote.to_string()).inline(false).name("A quote.")
        })
        .collect()
}
