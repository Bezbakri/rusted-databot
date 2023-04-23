use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::option;
use poise::serenity_prelude::CacheAndHttp;

use super::{Context, Error};

static mut QUEUE: VecDeque<serenity::Member> = VecDeque::new();
static mut OPEN: bool = false;

#[poise::command(slash_command, prefix_command)]
pub async fn server_info(
    ctx: Context<'_>,
)
    -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    ctx.send(|b| {
        b.embed(|b| {
            b.field("Members", guild.member_count, true)
                .title(guild.name)
        })
            .ephemeral(false)
    }).await?;
    // ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async unsafe fn start_OH(
    ctx: Context<'_>,
)
    -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    // get "CA" role
    let ta_role: serenity::Role;
    for role in guild.roles {
        if role.1.name == "CA" {
            ta_role = role.1;
        }
    }
    // if user has "CA" role then start OH
    // i have no idea how to do this
    if ctx.author().has_role().await.unwrap() {
        let response = format!("OH has started!");
        ctx.say(response).await?;
        OPEN = true;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async unsafe fn print_queue(
    ctx: Context<'_>,
)
    -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let mut queue_str = String::new();
    for member in &QUEUE {
        queue_str.push_str(member.to_string().as_str());
        queue_str += " ";
    }
    let response = format!("Queue: {}", queue_str);
    ctx.say(response).await?;
    Ok(())
}

