use songbird::input::YoutubeDl;

use crate::{Context, StdResult};

#[poise::command(slash_command)]
pub async fn play(
   ctx: Context<'_>,
   //#[description = "Enter a URL"] url: String
) -> StdResult<()> {
   // let _do_search = !url.starts_with("http");
   // let guild_id = ctx.guild_id().unwrap();
   // let http_client = ctx.data().http_key.clone();
   // let manager = songbird::get(ctx.serenity_context())
   //    .await
   //    .expect("Songbird Voice client placed in at initialisation.")
   //    .clone();

   // if let Some(handler_lock) = manager.get(guild_id) {
   //    let mut handler = handler_lock.lock().await;
   //    let src = YoutubeDl::new(http_client, url);

   //    let _ = handler.play_input(src.clone().into());
   //    if let Err(e) = ctx.say("Successfully found a track").await {
   //       panic!("Failed to send success play notice: {:?}", e);
   //    }
   // } else {
   //    if let Err(e) = ctx.say("I ain't here (need to be in VC)").await {
   //       panic!("Failed to send error play notice: {:?}", e);
   //    }
   // }

   Ok(())
}

///Play via URL
#[poise::command(slash_command)]
pub async fn url(
   ctx: Context<'_>,
   #[description = "Enter a URL"] url: String
) -> StdResult<()> {
   let _do_search = !url.starts_with("http");
   let guild_id = ctx.guild_id().unwrap();
   let http_client = ctx.data().http_key.clone();
   let manager = songbird::get(ctx.serenity_context())
      .await
      .expect("Songbird Voice client placed in at initialisation.")
      .clone();

   if let Some(handler_lock) = manager.get(guild_id) {
      let mut handler = handler_lock.lock().await;
      let src = YoutubeDl::new(http_client, url);

      let _ = handler.play_input(src.clone().into());
      if let Err(e) = ctx.say("Successfully found a track").await {
         panic!("Failed to send success play notice: {:?}", e);
      }
   } else {
      if let Err(e) = ctx.say("I ain't here (need to be in VC)").await {
         panic!("Failed to send error play notice: {:?}", e);
      }
   }

   Ok(())
}