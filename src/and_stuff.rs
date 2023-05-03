// Import the required libraries and modules
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use serenity::{ChannelId, GatewayIntents, Message, MessageId};
use serenity::model::id::GuildId;

use super::{Context, Error};

use plotly::{Plot, Scatter};
use std::collections::HashMap;
use plotly::Layout;
use plotly::layout::Axis;
use chrono::prelude::*;

// Define the `member_count` command that displays the member count of the server
// This command can be invoked as a slash command or a prefix command
#[poise::command(slash_command, prefix_command)]
pub async fn member_count(ctx: Context<'_>) -> Result<(), Error> {
    // Get the current guild (server) from the context
    let guild = ctx.guild().ok_or("Could not get guild")?;
    // Get the member count from the guild
    let member_count = guild.member_count;
    // Format the response text
    let response = format!("This server has {} members", member_count);
    // Send the response text to the channel
    ctx.say(response).await?;
    // Return Ok if the command execution was successful
    Ok(())
}

// Define the `count_messages` command that displays the message count in a channel
// This command can be invoked as a slash command or a prefix command
#[poise::command(slash_command, prefix_command)]
pub async fn count_messages(
    ctx: Context<'_>,
    channel: ChannelId,
) -> Result<(), Error> {
    // Initialize the message count to 0
    let mut count = 0;
    // Initialize the last_message_id variable as an Option of MessageId
    let mut last_message_id : Option<MessageId> = None;

    // Loop to fetch messages in batches and count them
    loop {
        // Fetch messages from the channel using the context and the last_message_id
        let messages = channel
        .messages(ctx.serenity_context(), |retriever| { //.messages(ctx.discord(), |retriever| {
            if let Some(last_id) = last_message_id {
                retriever.limit(100).before(last_id.0 - 1)
            } else {
                retriever.limit(100)
            }
        })
        .await?;

        // Get the length of the fetched messages array
        let len = messages.len();
        // Break the loop if no messages are fetched
        if len == 0 {
            break;
        }

        // Add the length of the fetched messages array to the count
        count += len;
        // Update the last_message_id variable
        last_message_id = messages.last().map(|m| m.id);
    }

    // Format the reply text with the message count and channel ID
    let reply_text = format!("There are {} messages in <#{}>.", count, channel.0);
    // Send the reply text to the channel
    ctx.say(reply_text).await?;
    // Return Ok if the command execution was successful
    Ok(())
}

// Chris stuff

#[poise::command(slash_command, prefix_command)]
pub async fn new_users(ctx: Context<'_>) -> Result<(), Error> { // Move this fn to your own file if you want chris
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            poise::say_reply(ctx, "This command can only be used in a server.").await?;
            return Ok(());
        }
    };

    let guild = ctx.guild(); //returns a Result<Guild> object

    let member_list = guild.unwrap().members; //Returns HashMap<Userid, Member> Member is an obj
    let mut new_users_per_date = HashMap::new();

    for member in member_list.values() {    //Iterates and extracts name + date joined, counts number of new users per date
        let user = &member.user;
        let join_time = member.joined_at.unwrap();
        // let join_minute = join_time.minute();
        let join_date = join_time.date_naive();
        //poise::say_reply(ctx, format!("User: {} | Joined at: {}", user.name, join_time)).await?;

        //Increment users_per_date with member's join date
        let count = new_users_per_date.entry(join_date).or_insert(0);
        *count += 1;
    }

    // Put number of users per time into a vector and sort in order of date
    let mut sorted_dates: Vec<_> = new_users_per_date.into_iter().collect();
    sorted_dates.sort_by_key(|&(k, _)| k);

    let (mode_date, mode_value) = sorted_dates.iter()
        .max_by_key(|&(_, v)| v).unwrap();
    let highest_date = format!("Highest join date was: {}, with {} new server members", mode_date, mode_value); 

    let vec_str = sorted_dates
    .iter()
    .map(|(a, b)| format!("New users on {}: {}", a, b))
    .collect::<Vec<_>>()
    .join("\n");

    ctx.say(vec_str).await?;
    ctx.say(highest_date).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn total_users(ctx: Context<'_>) -> Result<(), Error> { // Move this fn to your own file if you want chris
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            poise::say_reply(ctx, "This command can only be used in a server.").await?;
            return Ok(());
        }
    };
    
    let guild = ctx.guild(); //returns a Result<Guild> object

    let member_list = guild.unwrap().members; //Returns HashMap<Userid, Member> Member is an obj
    let mut new_users_per_date = HashMap::new();

    for member in member_list.values() {    //Iterates and extracts name + date joined, counts number of new users per date
        let user = &member.user;
        let join_time = member.joined_at.unwrap();
        let join_date = join_time.date_naive();

        //Increment users_per_date with member's join date
        let count = new_users_per_date.entry(join_date).or_insert(0);
        *count += 1;
    }

    // Put number of users per time into a vector and sort in order of date
    let mut sorted_dates: Vec<_> = new_users_per_date.into_iter().collect();
    sorted_dates.sort_by_key(|&(k, _)| k);

    let mut cumulative_dates = HashMap::new();
    let mut total = 0;
    for (date, value) in sorted_dates {
        total += value;
        cumulative_dates.insert(date, total);
    }

    let vec_str = cumulative_dates
    .iter()
    .map(|(a, b)| format!("Total users on {}: {}", a, b))
    .collect::<Vec<_>>()
    .join("\n");

    ctx.say(vec_str).await?;
    Ok(())
}