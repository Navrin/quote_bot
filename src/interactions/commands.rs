use db::Connector;
use db::actions::*;
use db::models::*;

use interactions::handler::InviteUrl;
use interactions::parsing::get_values;

use serenity::builder::CreateEmbedField;
use serenity::prelude::*;
use serenity::model::{Message, User, GuildId, MessageId};
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError;

const DEFAULT_DISCORD_AVATAR: &str = "http://is1.mzstatic.com/image/thumb/Purple118/v4/98/0d/81/980d8181-c84b-4c21-cef8-126464197968/source/300x300bb.jpg";

pub fn invite_link(ctx: &mut Context, message: &Message, _: Args) -> Result<(), CommandError> {
    let data = ctx.data.lock();
    let inv_container = data.get::<InviteUrl>();

    match inv_container {
        Some(c) => {
            message.reply(&c.0)?;
        },
        None => {
            message.reply("The invite link could not be obtained at this time :(")?;
        }
    }

    Ok(())
}

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

fn rand_quote(ctx: &mut Context, message: &Message, author: &User, _: Args) -> Result<(), String> {
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
             .fields(create_quote_embed_section(vec![quotes], &guild_id))
        ))
        .map_err(|e| format!("Could not send message! Error: {:?}", e))?;
    
    Ok(())
}

fn contains_quotes(ctx: &mut Context, message: &Message, author: &User, mut args: Args) -> Result<(), String> {
    let data = ctx.data.lock();
    let connector = data.get::<Connector>().unwrap();
    let conn = connector
        .get_conn()
        .map_err(|_| "Could not get a connection to the db!")?;

    let guild_id = message
        .guild_id()
        .ok_or("This isn't a real channel. You're not real.")?;

    let query = args.single_quoted::<String>()
        .map_err(|_| "No query was given!")?;

    let quotes = find_contains_quotes(&conn, &author.id.to_string(), &guild_id.to_string(), &query)
        .map_err(|_| "Could not read the quote database.")?;

    if quotes.len() <= 0 {
        return Err(format!("No quotes that match {} found :(", query));
    }

    message
        .channel()
        .ok_or("This channel is fake!")?
        .send_message(|reply| reply.embed(|f| 
            f.thumbnail(&author.avatar_url().unwrap_or(DEFAULT_DISCORD_AVATAR.to_string()))
             .title(format!("A collection of quotes that contain {} from {}.", query, author.name))
             .fields(create_quote_embed_section(quotes, &guild_id))
        ))
        .map_err(|e| format!("Could not send message! Error: {:?}", e))?;
    
    Ok(())
}

fn list_quotes(ctx: &mut Context, message: &Message, author: &User, args: Args) -> Result<(), String> {
    let data = ctx.data.lock();
    let connector = data.get::<Connector>().unwrap();
    let conn = connector
        .get_conn()
        .map_err(|_| "Could not get a connection to the db!")?;

    let guild_id = message
        .guild_id()
        .ok_or("This isn't a real channel. You're not real.")?;

    let rest = args.full();

    let set = get_values(&rest)?;

    let amount = match set.get(&"amount".to_string()) {
        Some(v) => Some(v.as_str().parse::<i64>().map_err(|_| "Could not parse the amount into a number!")?),
        None => None,
    };

    let page = match set.get(&"page".to_string()) {
        Some(v) => Some(v.as_str().parse::<i64>().map_err(|_| "Could not parse the page into a number!")?),
        None => None,
    };

    let quotes = find_listed_quotes(&conn, &author.id.to_string(), &guild_id.to_string(), ListParams {
        amount,
        page,
    })
        .map_err(|_| "Could not read the quote database.")?;

    if quotes.len() <= 0 {
        return Err("No quotes found :(".to_string());
    }

    message
        .channel()
        .ok_or("This channel is fake!")?
        .send_message(|reply| reply.embed(|f| 
            f.thumbnail(&author.avatar_url().unwrap_or(DEFAULT_DISCORD_AVATAR.to_string()))
             .title(format!("A collection of quotes from {} page {} amount {}.", author.name, page.unwrap_or(1), amount.unwrap_or(5)))
             .fields(create_quote_embed_section(quotes, &guild_id))
        ))
        .map_err(|e| format!("Could not send message! Error: {:?}", e))?;
    
    Ok(())
}


fn create_quote_embed_section(quotes: Vec<Quote>, guild: &GuildId) -> Vec<CreateEmbedField> {
    quotes
        .iter()
        .map(|quote| {
            CreateEmbedField::default()
                .value(quote.quote.to_string())
                .inline(false)
                .name(
                    format!(
                        "{}, quoted by {}", 
                        MessageId(quote.message_id
                            .parse::<u64>()
                            .unwrap())
                            .created_at()
                            .format("%m-%d @ %H:%S"), 
                        guild
                            .member(quote.quoted_by_id.parse::<u64>().unwrap())
                            .map(|m| m.display_name().to_string())
                            .unwrap_or("<user left>".to_string())
                    )
                )
        })
        .collect()
}
