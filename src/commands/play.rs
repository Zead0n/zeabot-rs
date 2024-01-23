use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use ::serenity::builder::CreateButton;
use ::serenity::futures::StreamExt;
use serenity::prelude::Mutex;
use songbird::input::YoutubeDl as SongbirdDl;
use songbird::Call;
pub use poise::serenity_prelude as serenity;
use poise::reply::CreateReply;
use youtube_dl::{SearchOptions, YoutubeDl, YoutubeDlOutput, SingleVideo};

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
   if let Err(e) = ctx.defer().await {
      panic!("Error defering search: {:?}", e);
   }

   let search_result = YoutubeDl::search_for(&SearchOptions::youtube(title).with_count(5))
      .socket_timeout("20")
      .extract_audio(true)
      .run_async()
      .await?;

   println!("YoutubeDl search done");
   match search_result {
      YoutubeDlOutput::Playlist(playlist) => {
         let mut search_map: HashMap<u8, SingleVideo> = HashMap::new();
         // let mut search_map = ctx.data().search.lock().await;
         // let mut search_list = String::new();
         let mut index = 1;

         for video in playlist.entries.expect("Failed to get videos of playlist") {
            search_map.insert(index, video);
            index += 1;
         }
         index = 1;

         if let Err(e) = search_init(ctx, search_map, &mut index, handler).await {
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

async fn search_init(ctx: Context<'_>, search: HashMap<u8, SingleVideo>, index: &mut u8, handler: Arc<Mutex<Call>>) -> StdResult<()> {
   println!("Search initiation started");
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
   println!("Waiting for interaction");
   while let Some(interaction) = interaction_stream.next().await {
      let custom_id = interaction.data.custom_id.as_str();
      match custom_id {
         "up" => {
            if *index < 5 {
               *index += 1;
            } else {
               *index = 1;
            }

            if let Err(e) = interaction.edit_response(
               &ctx, 
               search_msg(search.clone(), index).unwrap().to_slash_initial_response_edit(serenity::EditInteractionResponse::new())
            ).await{
               panic!("I'm too tired: {:?}",e);
            }
         }
         "down" => {
            if *index > 1 {
               *index -= 1;
            } else {
               *index = 5
            }

            if let Err(e) = interaction.edit_response(
               &ctx,
               search_msg(search.clone(), index).unwrap().to_slash_initial_response_edit(serenity::EditInteractionResponse::new())
            ).await{
               panic!("I'm too tired: {:?}",e);
            }
         }
         "select" => {
            let video = search.get(&index).expect("No video found in search").to_owned();
            let http_client = {
               let data = ctx.serenity_context().data.read().await;
               data.get::<HttpKey>()
                  .cloned()
                  .expect("Guaranteed to exist in the typemap.")
            };
            let src = SongbirdDl::new(http_client.clone(), video.url.expect("No url found"));
            let mut handler_lock = handler.lock().await;
            handler_lock.enqueue_input(src.into()).await;
            
            println!("Track playing from search");
            let video_respone = format!("**Successfully added track:** {}", video.title.expect("No title for video"));
            commands::check_message(ctx.say(video_respone).await);

            return Ok(());
         }
         _ => println!("Unknow custom_id")
      }
   }

   // if let Ok(reply) = &ctx.send(search_msg(search.clone(), index).unwrap()).await {
   //    let msg = reply.message().await?;
   //    let mut interaction_stream = msg
   //       .clone()
   //       .await_component_interaction(&ctx.serenity_context().shard)
   //       .timeout(Duration::from_secs(60 * 2))
   //       .stream();
   //    println!("Waiting for interaction");
   //    while let Some(interaction) = interaction_stream.next().await {
   //       let custom_id = interaction.data.custom_id.as_str();
   //       match custom_id {
   //          "up" => {
   //             if *index < 5 {
   //                *index += 1;
   //             } else {
   //                *index = 1;
   //             }

   //             if let Err(e) = interaction.edit_response(
   //                &ctx, 
   //                search_msg(search.clone(), index).unwrap().to_slash_initial_response_edit(serenity::EditInteractionResponse::new())
   //             ).await{
   //                panic!("I'm too tired: {:?}",e);
   //             }
   //          }
   //          "down" => {
   //             if *index > 1 {
   //                *index -= 1;
   //             } else {
   //                *index = 5
   //             }

   //             if let Err(e) = interaction.edit_response(
   //                &ctx,
   //                search_msg(search.clone(), index).unwrap().to_slash_initial_response_edit(serenity::EditInteractionResponse::new())
   //             ).await{
   //                panic!("I'm too tired: {:?}",e);
   //             }
   //          }
   //          "select" => {
   //             let video = search.get(&index).expect("No video found in search").to_owned();
   //             let http_client = {
   //                let data = ctx.serenity_context().data.read().await;
   //                data.get::<HttpKey>()
   //                   .cloned()
   //                   .expect("Guaranteed to exist in the typemap.")
   //             };
   //             let src = SongbirdDl::new(http_client.clone(), video.url.expect("No url found"));
   //             let mut handler_lock = handler.lock().await;
   //             handler_lock.enqueue_input(src.into()).await;
               
   //             println!("Track playing from search");
   //             let video_respone = format!("**Successfully added track:** {}", video.title.expect("No title for video"));
   //             commands::check_message(ctx.say(video_respone).await);
   //          }
   //          _ => println!("Unknow custom_id")
   //       }
   //    }
   // } else {
   //    panic!("Something wrong here then");
   // }

   Ok(())
}

pub fn search_msg(search: HashMap<u8, SingleVideo>, index: &mut u8) -> StdResult<CreateReply> {
   println!("New search msg generating");
   let mut search_list = String::new();
   for (k, v) in search.clone().into_iter() {
      if k == *index {
         search_list.push_str(format!("→{}. {}\n", k, v.title.expect("No title for video")).as_str());
      } else {
         search_list.push_str(format!(" {}. {}\n", k, v.title.expect("No title for video")).as_str());
      }
   }
   println!("Formated selection");

   let embed = serenity::CreateEmbed::new().title("Search result").color((255, 0, 0)).field("Found tracks:", search_list, false);
   let mut button_vec = Vec::new();
   button_vec.push(CreateButton::new("up").emoji(serenity::ReactionType::Unicode("U+2191".to_string())).style(serenity::ButtonStyle::Primary));
   button_vec.push(CreateButton::new("down").emoji(serenity::ReactionType::Unicode("U+2193".to_string())).style(serenity::ButtonStyle::Primary));
   button_vec.push(CreateButton::new("select").emoji(serenity::ReactionType::Unicode("U+1F3B5".to_string())).style(serenity::ButtonStyle::Success));
   println!("Made Embed and Buttons");

   Ok(CreateReply::embed(CreateReply::default(), embed).components(vec![serenity::CreateActionRow::Buttons(button_vec)]))
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
   // let mut queue = ctx.data().track_queue.lock().await;
   // queue.push(track_name.clone());

   // handler_lock.enqueue_input(src.into()).await;

   Ok(())
}

