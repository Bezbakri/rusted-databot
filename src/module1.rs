use poise::serenity_prelude as serenity;

use super::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn gpa(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) 
-> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}, is predicted a 4.0 GPA", u.name);
    ctx.say(response).await?;
    Ok(())
}