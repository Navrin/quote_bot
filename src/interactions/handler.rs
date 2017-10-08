use serenity::prelude::*;
use serenity::model::*;

const SHOOTING_STAR_EMOJI: &str = "🌠";
const SMALL_STAR_EMOJI: &str = "⭐";
const SHINY_STAR_EMOJI: &str = "🌟";

const NOTEPAD_EMOJI: &str = "📝";

pub struct Handler;

impl EventHandler for Handler {
    fn on_reaction_add(&self, _: Context, react: Reaction) {
        match react.user_id.get() {
            Ok(user) => {
                if user.bot {
                    return; 
                }
            }
            Err(_) => return,
        }

        println!(
            "someone reacted a: {}  in channel: {}",
            react.emoji.as_data(),
            react.channel_id.name().unwrap()
        );

        let emoji: &str = &react.emoji.as_data();

        match emoji {
            SHOOTING_STAR_EMOJI | SMALL_STAR_EMOJI | SHINY_STAR_EMOJI => {
                println!("Ooh! A shiny star!");
                let message = match react.channel_id.message(react.message_id) {
                    Ok(v) => v,
                    Err(_) => return, // message was deleted before we could even log it! someone must really dislike that quote.
                };

                if let Err(_) = message.react(NOTEPAD_EMOJI) {
                    return; // couldn't react, no permissions?
                };
            }
            _ => return (),
        }
    }
}
