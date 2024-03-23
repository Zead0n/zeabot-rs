// use poise::serenity_prelude as serenity;

// use crate::{Context, StdResult, commands};

// /// Show queue
// #[poise::command(slash_command)]
// pub async fn queue(
//    ctx: Context<'_>
// ) -> StdResult<()> {
//    if let Some(handler) = commands::handler_exist(ctx).await {
//       let _handler_lock = handler.lock().await;
//       let queue_list = ctx.data().track_queue.lock().await;
//       let mut queue_view = String::new();

//       for track in queue_list.clone().into_iter() {
//          queue_view.push_str(format!("{}\n", track).as_str());
//       }
//       let reply: poise::reply::CreateReply = Default::default();
//       let embed = serenity::CreateEmbed::new().field("Songs", queue_view, false);

//       if let Err(e) = ctx.send(poise::reply::CreateReply::embed(reply, embed)).await {
//          panic!("failed to send queue: {:?}", e)
//       }

//    }

//    Ok(())
// }
