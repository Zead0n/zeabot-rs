// use poise::serenity_prelude as serenity;
use poise::reply::CreateReply;
use songbird::tracks::LoopState;
use crate::{Context, StdResult, commands};

/// Show queue
#[poise::command(slash_command)]
pub async fn r#loop(
   ctx: Context<'_>
) -> StdResult<()> {
   if let Some(handler) = commands::handler_exist(ctx).await {
      let handler_lock = handler.lock().await;
      let current_track = match handler_lock.queue().current() {
         Some(track) => track,
         None => {
            let message = CreateReply::default().content("No track currently playing").ephemeral(true);
            if let Err(e) = ctx.send(message).await {
               panic!("Error sending failed track search loop: {:?}", e);
            }

            panic!("No track found to loop");
         }
      };
      let track_state = match current_track.get_info().await {
         Ok(state) => state,
         Err(e) => panic!("Error getting loop state: {:?}", e)
      };
      let loop_state = track_state.loops;

      match loop_state {
         LoopState::Infinite => {
            if let Err(e) = current_track.disable_loop() {
               panic!("Error disabling loop: {:?}", e);
            }

            let message = CreateReply::default().content("Disabled looping");
            if let Err(e) = ctx.send(message).await {
               panic!("Error sending disabling loop: {:?}", e);
            }
         },
         LoopState::Finite(_) => {
            if let Err(e) = current_track.enable_loop() {
               panic!("Error enabling loop: {:?}", e);
            }

            let message = CreateReply::default().content("Enabled looping");
            if let Err(e) = ctx.send(message).await {
               panic!("Error sending enabling loop: {:?}", e);
            }
         }
      }
   } else {
      let message = CreateReply::default().content("Not in VC");
      if let Err(e) = ctx.send(message).await {
         panic!("Error sending no handler loop: {:?}", e);
      }
   }

   Ok(())
}
