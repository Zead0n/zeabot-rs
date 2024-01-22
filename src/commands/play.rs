use std::sync::Arc;
use serenity::prelude::Mutex;
use songbird::input::{YoutubeDl as SongbirdDl, Compose};
use youtube_dl::{SearchOptions, YoutubeDl, YoutubeDlOutput};
use songbird::Call;

use crate::{Context, StdResult, commands};
use crate::bot::HttpKey;

#[poise::command(slash_command)]
pub async fn play(
   ctx: Context<'_>,
) -> StdResult<()> {
   if let Err(e) = ctx.say("Should require subsommand").await {
      panic!("Failed send play is subcommand nitification: {:?}", e);
   }

   Ok(())
}

///Play via URL
#[poise::command(slash_command)]
pub async fn url(
   ctx: Context<'_>,
   #[description = "Enter a URL"] url: String
) -> StdResult<()> {
   if let Some(handler) = commands::handler_exist(ctx).await {
      if let Err(e) = queue_up(ctx, url, handler).await {
         panic!("Error queuing music: {:?}", e);
      }
   } else {
      let new_handler = commands::join_channel(ctx).await?;
      if let Err(e) = queue_up(ctx, url, new_handler).await {
         panic!("Error queuing music from joining: {:?}", e);
      }
   }

   Ok(())
}

// Search a title (WIP)
#[poise::command(slash_command)]
pub async fn search(
   ctx: Context<'_>,
   #[description = "Enter a title"] title: String
) -> StdResult<()> {
   let search_result = YoutubeDl::search_for(&SearchOptions::youtube(title).with_count(5))
      .socket_timeout("20")
      .extract_audio(true)
      .run_async()
      .await?;

   match search_result {
      YoutubeDlOutput::SingleVideo(_) => println!("SingleVideo"),
      YoutubeDlOutput::Playlist(_) => println!("Playlist"),
   }
}

async fn queue_up(ctx: Context<'_>, url: String, handler: Arc<Mutex<Call>>) -> StdResult<()> {
   if let Err(e) = ctx.defer().await {
      panic!("Failed to defer song addition: {:?}", e);
   }

   let http_client = {
      let data = ctx.serenity_context().data.read().await;
      data.get::<HttpKey>()
         .cloned()
         .expect("Guaranteed to exist in the typemap.")
   };
   let test = YoutubeDl::new(url).socket_timeout("10").extract_audio(true).run_async().await?;

   let mut handler_lock = handler.lock().await;
   // let src = SongbirdDl::new(http_client, url);
   match test {
      YoutubeDlOutput::SingleVideo(video) => {
         let src = SongbirdDl::new(http_client, video.url);
         handler_lock.enqueue_input(src.into()).await;
         
         let video_respone = format!("**Successfully added track:** {}", video.title.expect("No title for video"));
         commands::check_message(ctx.say(video_respone));
      },
      YoutubeDlOutput::Playlist(playlist) => {
         let videos = playlist.entries.expect("Failed to get videos of playlist");
         if videos.len() >= 10 {
            commands::check_message(ctx.say("Sorry, don't take playlists with 10 videos or more\n(This is experimental)").await);

            Ok(())
         }

         let video_list: Vec<String> = Vec::new();
         for video in videos {
            video_list.push(format!("\n{}",video.title.expect("No title for video")));
            let src = SongbirdDl::new(http_client, video.url);
            handler_lock.enqueue_input(src.into()).await;
         }

         let playlist_respone = format!("**Successfully added playlist:** {}\n**__Added tracks :__** {}", playlist.title.expect("No title for playlist"), video_list);
         commands::check_message(ctx.say(playlist_respone).await);
      }
   }
   // let mut queue = ctx.data().track_queue.lock().await;
   // queue.push(track_name.clone());

   // handler_lock.enqueue_input(src.into()).await;

   Ok(())
}