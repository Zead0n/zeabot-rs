use poise::serenity_prelude as serenity;
use serenity::model::id::RoleId;
use poise::reply::CreateReply;

use crate::Context;

pub fn check_result<T, E: std::fmt::Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(success) => return success,
        Err(e) => panic!("Error in result: {:?}", e)
    }
}

const TEST_SERVER: u64 = 884664077643829248;
const MEME_CORP: u64 = 459781165377650688;
const NIPPON: u64 = 270329415404093440;

pub async fn has_perm(ctx: &Context<'_>) -> bool {
    let member = ctx.author_member().await.expect("No member found");
    let perm = match member.guild_id.get() {
        NIPPON => {
            const MY_BOI: u64 = 540989126803980289;
            const OKAMI: u64 = 153682548017463296;
            const BOT_CHANNEL: u64 = 360582111398330369;

            let roled = member.roles.contains(&RoleId::new(MY_BOI));
            let okami = member.user.id.get() == OKAMI;
            let bot_channel = ctx.channel_id().get() == BOT_CHANNEL;

            if (roled || okami) && bot_channel {
                return true;
            }

            return false;
        }
        TEST_SERVER => true,
        MEME_CORP => true,
        _ => {
            println!("An unknown server has run a command");
            false
        },
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