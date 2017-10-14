use serenity::prelude::*;
use serenity::model::*;
use db::actions::create_quote;
use db::models::NewQuote;
use db::Connector;
use typemap::Key;

pub struct InviteUrl(pub String);

impl Key for InviteUrl {
    type Value = InviteUrl;
}

const SHOOTING_STAR_EMOJI: &str = "🌠";
const SMALL_STAR_EMOJI: &str = "⭐";
const SHINY_STAR_EMOJI: &str = "🌟";

const STARS: [&str; 3] = [SHOOTING_STAR_EMOJI, SMALL_STAR_EMOJI, SHINY_STAR_EMOJI];

const NOTEPAD_EMOJI: &str = "📝";

pub struct Handler;

impl EventHandler for Handler {
    fn on_reaction_add(&self, ctx: Context, react: Reaction) {
        let user = match react.user_id.get() {
            Ok(user) => {
                if user.bot {
                    return; 
                }
                user
            }
            Err(_) => return,
        };

        let emoji: &str = &react.emoji.as_data();

        match emoji {
            SHOOTING_STAR_EMOJI | SMALL_STAR_EMOJI | SHINY_STAR_EMOJI => {
                let message = match react.channel_id.message(react.message_id) {
                    Ok(v) => v,
                    Err(_) => return, // message was deleted before we could even log it! someone must really dislike that quote.
                };

                if message.author.id == user.id {
                    let _ = react.channel_id.say("You can't quote yourself >:(");
                    return;
                }

                let guild = match message.channel() {
                    Some(Channel::Guild(g)) => {
                        let guild = g.clone();
                        let guild = match guild.read() {
                            Ok(v) => v,
                            Err(_) => return (),   
                        };   
                        guild.guild_id.clone()
                    },
                    _ => {
                        let _ = react.channel_id.say("I have no idea how I got here, please stop messaging me");
                        return;
                    }
                };

                let star_count = 
                    message.reactions
                    .iter()
                    .filter(|r| STARS.contains(&r.reaction_type.as_data().as_str()))
                    .map(|r| r.count)
                    .sum::<u64>();

                if star_count > 1 { // no dups pls
                    return;
                }

                {
                    let data = ctx.data.lock();
                    let connector = data.get::<Connector>().unwrap();
                    let conn = match connector.get_conn() {
                        Ok(v) => v,
                        Err(_) => {
                            if let Some(c) = message.channel() {
                                let _ = c.say("Could not gain access to database?!");
                            }
                            return;
                        }   
                    }; 

                    let quote = if message.content.len() > 0 {
                        message.content.to_string()
                    } else {
                        message.attachments
                            .iter()
                            .map(|att| att.url.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    };

                    if quote.len() <= 0 {
                        let _ = message.reply("This message is empty (maybe just an embed?)");
                        return;
                    }

                    let save_result = create_quote(&conn, &NewQuote {
                        message_id: &message.id.0.to_string(),
                        quote: &quote,
                        created_by_id: &message.author.id.0.to_string(),
                        quoted_by_id: &react.user_id.0.to_string(),
                        guild_id: &guild.0.to_string(),
                    });

                    if let Err(_) = save_result {
                        if let Some(c) = message.channel() {
                            let _ = c.say("Could not write quote this time :( you got lucky!");
                            return;
                        }
                    }
                }


                if let Err(_) = message.react(NOTEPAD_EMOJI) {
                    return; // couldn't react, no permissions?
                };
            }
            _ => return (),
        }
    }

    fn on_ready(&self, ctx: Context, ready: Ready) {
        let url = format!("https://discordapp.com/api/oauth2/authorize?client_id={}&scope=bot&permissions=0", ready.user.id);
        println!("{}", url);

        let url_holder = InviteUrl(url);
        {
            let mut data = ctx.data.lock();
            data.insert::<InviteUrl>(url_holder);
        }
    }
}
