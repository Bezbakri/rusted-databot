use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::option;

use super::{Context, Error};


#[poise::command(slash_command, prefix_command)]
pub async fn channel_count(
    ctx: Context<'_>,
) 
-> Result<(), Error> {
    let guild = ctx.guild().unwrap().name;
    let channel = ctx.guild().unwrap().channels;
    let mut channel_count:usize = 0;
    let mut category_count:usize = 0;
    let mut channel_map_str = String::new();
    for (_key, value) in channel {
        match value.clone().category() {
            Some(_category) => {
                category_count += 1;
            },
            None => {
                channel_count += 1;
                channel_map_str.push_str(value.to_string().as_str());
                channel_map_str += " ";
            },
        }
        
    }
    let response = format!("{} has {} channels in {} categories.",guild, channel_count, category_count);
    ctx.say(response).await?;
    let response2 = format!("channels: {}", channel_map_str);
    ctx.say(response2).await?;
    Ok(())
}