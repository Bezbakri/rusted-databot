use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use serenity::{ChannelId, GatewayIntents, Message, MessageId};

use super::{Context, Error};

use plotly::{Plot, Scatter};
use std::collections::HashMap;
use plotly::common::Mode;
// use plotly::Layout;
// use plotly::layout::Axis;
 use chrono::prelude::*;

/// Displays the member count of the server
#[poise::command(slash_command, prefix_command)]
pub async fn member_count(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().ok_or("Could not get guild")?;
    let member_count = guild.member_count;
    let response = format!("This server has {} members", member_count);
    ctx.say(response).await?;
    Ok(())
}

// Displays message count in a channel
#[poise::command(slash_command, prefix_command)]
pub async fn count_messages(
    ctx: Context<'_>,
    channel: ChannelId,
) -> Result<(), Error> {
    let mut count = 0;
    let mut last_message_id : Option<MessageId> = None;

    loop {
        let messages = channel
        .messages(ctx.discord(), |retriever| {
            if let Some(last_id) = last_message_id {
                retriever.limit(100).before(last_id.0 - 1)
            } else {
                retriever.limit(100)
            }
        })
        .await?;

        let len = messages.len();
        if len == 0 {
            break;
        }

        count += len;
        last_message_id = messages.last().map(|m| m.id);
    }

    let reply_text = format!("There are {} messages in <#{}>.", count, channel.0);
    ctx.say(reply_text).await?;
    Ok(())
}

// /// Graphs the number of users in the server over time
#[poise::command(slash_command)]
pub async fn user_graph(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().ok_or("Could not get guild")?;

    // Collect member count for each day
    let mut member_counts = HashMap::new();
    // Create an Http reference from the Context
    let http = ctx.as_ref().http().clone();
    let members = guild.members(&http, Some(1000), None).await?;
    for member in members {
        if let Some(joined_at) = member.joined_at {
            let date = joined_at.date().naive_utc();
            *member_counts.entry(date).or_insert(0) += 1;
        }
    }

    // Sort member counts by date
    let mut member_counts: Vec<(NaiveDate, usize)> = member_counts.into_iter().collect();
    member_counts.sort_by_key(|(date, _)| *date);

//     // Create plot
    let x: Vec<NaiveDate> = member_counts.iter().map(|(x, _)| *x).collect();
    let y: Vec<usize> = member_counts.iter().map(|(_, y)| *y).collect();

    let trace = Scatter::new(x,y).mode(Mode::Lines);
//     let trace = Scatter::new(
//         member_counts.iter().map(|(date, _)| date.to_string()).collect(),
//         member_counts.iter().map(|(_, count)| *count).collect(),
//     )
//     .name("Member Count")
//     .mode(plotly::common::Mode::Lines);

    let mut plot = Plot::new();
    plot.add_trace(trace);

//     let max_count = member_counts.iter().map(|(_, count)| count).max().unwrap_or(&0);

//     let layout = Layout::new()
//         .title("User Count Over Time")
//         .x_axis(Axis::new().title("Date"))
//         .y_axis(Axis::new().title("Member Count").range(vec![0., *max_count as f64]));

//     plot.set_layout(layout);




//     // Send plot
//     let image = plot.to_image("png").await?;
//     ctx.say("User count over time:", |f| f.add_file("plot.png", image.as_slice()))
//         .await?;

    Ok(())
}