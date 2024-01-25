use std::sync::Arc;
// use std::collections::HashMap;
use std::time::Duration;
use ::serenity::builder::CreateButton;
use ::serenity::futures::StreamExt;
use serenity::prelude::Mutex;
use songbird::input::YoutubeDl as SongbirdDl;
use songbird::Call;
pub use poise::serenity_prelude as serenity;
use poise::reply::CreateReply;
use youtube_dl::{SearchOptions, YoutubeDl, YoutubeDlOutput, SingleVideo};

use crate::{commands, error, Context, StdResult};
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
   if let Some(handler) = commands::handler_exist(ctx).await {
      if let Err(e) = search_up(ctx, title, handler).await {
         panic!("Error queuing music: {:?}", e);
      }
   } else {
      let new_handler = commands::join_channel(ctx).await?;
      if let Err(e) = search_up(ctx, title, new_handler).await {
         panic!("Error queuing music from joining: {:?}", e);
      }
   }

   Ok(())
}

async fn search_up(ctx: Context<'_>, title: String, handler: Arc<Mutex<Call>>) -> StdResult<()> {
   // if let Err(e) = ctx.defer().await {
   //    panic!("Error defering search: {:?}", e);
   // }

   error::check_result::<(), serenity::Error>(ctx.defer().await);

   let search_result = YoutubeDl::search_for(&SearchOptions::youtube(title).with_count(5))
      .socket_timeout("20")
      .extract_audio(true)
      .run_async()
      .await?;

   match search_result {
      YoutubeDlOutput::Playlist(playlist) => {
         let mut search_vec: Vec<SingleVideo> = Vec::new();

         for video in playlist.entries.expect("Failed to get videos of playlist") {
            search_vec.push(video);
         }
         // search_vec.sort_by(|a, b| a.0.cmp(&b.0));
         // let search_map: HashMap<u8, SingleVideo> = search_vec.into_iter().collect();
         
         if let Err(e) = search_init(ctx, search_vec, handler).await {
            panic!("Error creating search embed: {:?}", e);
         }

         Ok(())
      },
      _ => {
         println!("Something went wrong?");
         Ok(())
      }
   }
}

async fn search_init(ctx: Context<'_>, search: Vec<SingleVideo>, handler: Arc<Mutex<Call>>) -> StdResult<()> {
   let mut index = 0;
   let reply = match ctx.send(search_msg(search.clone(), index).unwrap()).await {
      Ok(reply) => reply,
      Err(e) => {
         panic!("Skill issue: {:?}", e);
      }
   };

   let msg = reply.message().await?;
   let mut interaction_stream = msg
      .clone()
      .await_component_interaction(&ctx.serenity_context().shard)
      .timeout(Duration::from_secs(60 * 2))
      .stream();
   while let Some(interaction) = interaction_stream.next().await {
      let custom_id = interaction.data.custom_id.as_str();
      match custom_id {
         "up" => {
            error::check_result::<(), serenity::Error>(ctx.defer().await);

            if index < 5 {
               index += 1;
            } else {
               index = 1;
            }

            if let Err(e) = interaction.edit_response(
               &ctx, 
               search_msg(search.clone(), index).unwrap().to_slash_initial_response_edit(serenity::EditInteractionResponse::new())
            ).await{
               panic!("I'm too tired: {:?}",e);
            }
         }
         "down" => {
            error::check_result::<(), serenity::Error>(ctx.defer().await);

            if index > 1 {
               index -= 1;
            } else {
               index = 5
            }

            if let Err(e) = interaction.edit_response(
               &ctx,
               search_msg(search.clone(), index).unwrap().to_slash_initial_response_edit(serenity::EditInteractionResponse::new())
            ).await{
               panic!("I'm too tired: {:?}",e);
            }
         }
         "select" => {
            error::check_result::<(), serenity::Error>(ctx.defer().await);

            let video = search.get(index as usize).expect("No video found in search").to_owned();
            let http_client = {
               let data = ctx.serenity_context().data.read().await;
               data.get::<HttpKey>()
                  .cloned()
                  .expect("Guaranteed to exist in the typemap.")
            };
            let src = SongbirdDl::new(http_client.clone(), video.url.expect("No url found"));
            let mut handler_lock = handler.lock().await;
            handler_lock.enqueue_input(src.into()).await;
            
            let video_respone = format!("**Successfully added track:** {}", video.title.expect("No title for video"));
            commands::check_message(ctx.say(video_respone).await);

            return Ok(());
         }
         _ => println!("Unknown custom_id")
      }
   }

   Ok(())
}

pub fn search_msg(search: Vec<SingleVideo>, index: u8) -> StdResult<CreateReply> {
   // let mut index_list = String::new();
   let mut song_list = String::new();
   for (k, v) in search.into_iter().enumerate() {
      match k {
         0 => {
            if 0 == index {
               song_list.push(format!("**{}**", v.title.expect("No title found")).as_str());
            } else {
               song_list.push(format!("{}", v.title.expect("No title found")).as_str());
            }
         }
         _ => {
            if k == index as usize {
               song_list.push(format!("\n\n**{}**", v.title.expect("No title found")).as_str());
            } else {
               song_list.push(format!("\n\n{}", v.title.expect("No title found")).as_str());
            }
         }
      }
   }

   let embed = serenity::CreateEmbed::new()
      .title("Search result").color((255, 0, 0))
      .field("Found tracks:", song_list, false);
   let components = serenity::CreateActionRow::Buttons(vec![
      CreateButton::new("up").emoji("⬆️".chars().next().unwrap()).style(serenity::ButtonStyle::Primary),
      CreateButton::new("down").emoji("⬇️".chars().next().unwrap()).style(serenity::ButtonStyle::Primary),
      CreateButton::new("select").emoji("🎵".chars().next().unwrap()).style(serenity::ButtonStyle::Success),
   ]);

   Ok(CreateReply::embed(CreateReply::default(), embed).components(vec![components]))
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
   match test {
      YoutubeDlOutput::SingleVideo(video) => {
         let src = SongbirdDl::new(http_client.clone(), video.url.expect("No url found"));
         handler_lock.enqueue_input(src.into()).await;
         
         let video_respone = format!("**Successfully added track:** {}", video.title.expect("No title for video"));
         commands::check_message(ctx.say(video_respone).await);
      },
      YoutubeDlOutput::Playlist(playlist) => {
         let videos = playlist.entries.expect("Failed to get videos of playlist");
         if videos.len() >= 10 {
            commands::check_message(ctx.say("Sorry, don't take playlists with 10 videos or more\n(This is experimental)").await);

            return Ok(());
         }

         let mut video_list = String::new();
         for video in videos {
            video_list.push_str(format!("\n{}",video.title.expect("No title for video")).as_str());
            let src = SongbirdDl::new(http_client.clone(), video.url.expect("No url found"));
            handler_lock.enqueue_input(src.into()).await;
         }

         let playlist_respone = format!("**Successfully added playlist:** {}\n**__Added tracks :__** {}", playlist.title.expect("No title for playlist"), video_list);
         commands::check_message(ctx.say(playlist_respone).await);
      }
   }

   Ok(())
}

