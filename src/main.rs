use poise::serenity_prelude as serenity;
use dotenv::dotenv;

use plotly::{Plot, Scatter};
use std::collections::HashMap;
use plotly::Layout;
use plotly::layout::Axis;
use chrono::prelude::*;


pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod bez_stuff;
mod oh_stuff;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Displays the member count of the server
#[poise::command(slash_command, prefix_command)]
async fn member_count(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().ok_or("Could not get guild")?;
    let member_count = guild.member_count;
    let response = format!("This server has {} members", member_count);
    ctx.say(response).await?;
    Ok(())
}

/// Graphs the number of users in the server over time
// #[poise::command(slash_command)]
// async fn user_graph(ctx: Context<'_>) -> Result<(), Error> {
//     let guild = ctx.guild().ok_or("Could not get guild")?;

//     // Collect member count for each day
//     let mut member_counts = HashMap::new();
//     // Create an Http reference from the Context
//     let http = ctx.as_ref().http().clone();
//     let members = guild.members(&http, Some(1000), None).await?;
//     for member in members {
//         if let Some(joined_at) = member.joined_at {
//             let date = joined_at.date().naive_utc();
//             *member_counts.entry(date).or_insert(0) += 1;
//         }
//     }

//     // Sort member counts by date
//     let mut member_counts: Vec<(NaiveDate, usize)> = member_counts.into_iter().collect();
//     member_counts.sort_by_key(|(date, _)| *date);

//     // Create plot
//     let trace = Scatter::new(
//         member_counts.iter().map(|(date, _)| date.to_string()).collect(),
//         member_counts.iter().map(|(_, count)| *count).collect(),
//     )
//     .name("Member Count")
//     .mode(plotly::common::Mode::Lines);

//     let mut plot = Plot::new();
//     plot.add_trace(trace);

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

//     Ok(())
// }

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), member_count(), bez_stuff::channel_count(), oh_stuff::server_info(), oh_stuff::start_OH()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}