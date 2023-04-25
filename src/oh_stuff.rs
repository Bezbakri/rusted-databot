use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::option;
use poise::serenity_prelude::CacheAndHttp;

use super::{Context, Error};

const QUEUE: VecDeque<serenity::Member> = VecDeque::new();
const OPEN: bool = false;

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
pub async fn start_OH(
    ctx: Context<'_>,
)
    -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    // get "CA" role
    let mut ta_role: Option<serenity::Role> = None;
    for role in guild.roles {
        if role.1.name == "CA" {
            ta_role = Some(role.1);
        }
    }
    if ta_role == None {
        ctx.say("Not a CA").await?;
    }
    // iterate over user's roles
    for role in ctx.author_member().await.unwrap().roles.clone() {
        if role == ta_role.clone().unwrap().id {
            let response = format!("OH has started!");
            ctx.say(response).await?;
            // OPEN = true;
        }
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn print_queue(
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

