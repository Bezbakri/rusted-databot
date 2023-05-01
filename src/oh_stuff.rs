use std::collections::HashMap;
use poise::serenity_prelude as serenity;
use poise::futures_util::Stream;
use serde::de::SeqAccess;

use super::{Context, Error};


#[poise::command(slash_command, prefix_command)]
pub async fn server_info(
    ctx: Context<'_>,
)
    -> Result<(), Error> {
    let guild = ctx.guild().unwrap();

    let mut map: HashMap<String, i32> = HashMap::new();

    let messages = ctx.channel_id()
        .messages(ctx.serenity_context(), |retriever| {
            retriever.limit(100)
        })
        .await?;

    // go through messages and add to map
    for message in messages {
        let author = message.author.name;
        let count = map.entry(author).or_insert(0);
        *count += 1;
    }

    let mut top3: Vec<(String, i32)> = Vec::from_iter(map.into_iter());
    top3.sort_by(|a, b| {b.1.cmp(&a.1)});

    let mut top: Vec<String> = top3.into_iter().map(|v| {v.0}).collect();
    top.truncate(5);

    let display_top_users: String = top.into_iter().reduce(|a, b| {a + "\n" + &b}).unwrap_or("".to_string());

    ctx.send(|b| {
        b.embed(|b| {
            b.field("Members", guild.member_count, true)
                .title(guild.name)
                .field("Most Active In This Channel", display_top_users, true)
        })
            .ephemeral(false)
    }).await?;
    // ctx.say(response).await?;
    Ok(())
}

