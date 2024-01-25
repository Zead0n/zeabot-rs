use poise::serenity_prelude as serenity;
use poise::reply::CreateReply;

use crate::Context;

pub fn check_result<T, E: std::fmt::Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(success) => return success,
        Err(e) => panic!("Error in result: {:?}", e)
    }
}

pub async fn has_perm(ctx: &Context<'_>) -> bool {
    let member = ctx.author_member().await.expect("No member found");
    let perm = match member.guild_id.get() {
        270329415404093440 => {
            let roled = member.roles.contains(&serenity::RoleId::new(540989126803980289));
            let bot_channel = ctx.channel_id() == serenity::ChannelId::new(360582111398330369);

            if roled && bot_channel {
                return true;
            }

            return false;
        }
        459781165377650688 => return true,
        _ => false,
    };

    if !perm {
        check_result(
            ctx
            .send(
                CreateReply::content(
                    CreateReply::default(), 
                    "You don't have the permission to run this command")
                .ephemeral(true)
            )
            .await
        );
    }

    perm
}