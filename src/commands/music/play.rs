use std::time::Duration;

use serenity::{
    builder::CreateApplicationCommand, client::Context, json::Value,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
};
use youtube_dl::SearchOptions;

const SLASH_NAME: &str = "link-or-query";
pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
    println!("Running play command.");

    if let Some(guild_id) = command.guild_id {
        println!("Guild Id found. Command will execute.");
        let song = command
            .data
            .options
            .iter()
            .find(|&f| f.name.eq(SLASH_NAME))
            .unwrap()
            .value
            .as_ref()
            .unwrap();

        if let Value::String(song) = song {
            println!("Song to play - {}", song);
            let manager = songbird::get(ctx).await.expect(
                "Failed to retrieve Songbird. Check if Songbird is registered on ClientBuilder.",
            ).clone();

            let channel_id = &ctx
                .cache
                .guild(guild_id)
                .unwrap()
                .voice_states
                .get(&command.user.id)
                .and_then(|voice_state| voice_state.channel_id)
                .expect("User needs to be connected to a voice channel.");

            let (handler_lock, conn_result) = manager.join(guild_id, channel_id.0).await;

            let _res = command.defer(&ctx.http).await;
            if conn_result.is_ok() {
                let mut handler = handler_lock.lock().await;

                command
                    .create_followup_message(&ctx.http, |f| {
                        f.ephemeral(true).content("Searching for song.")
                    })
                    .await
                    .unwrap();

                let search_options = SearchOptions::youtube(song).with_count(5);
                let mut ytd = youtube_dl::YoutubeDl::search_for(&search_options);
                let dl = youtube_dl::YoutubeDl::youtube_dl_path(&mut ytd, r"D:\youtube-dl-second");
                let playlist = dl.run().unwrap().into_playlist().unwrap().entries.unwrap();

                let m = command
                    .channel_id
                    .send_message(&ctx.http, |f| {
                        f.content("Select a choice between **1** to **4**, or **cancel**.")
                            .components(|c| {
                                c.create_action_row(|row| {
                                    row.create_select_menu(|m| {
                                        m.custom_id("songs")
                                            .placeholder("No song selected")
                                            .options(|o| {
                                                for song in &playlist {
                                                    o.create_option(|fo| {
                                                        fo.label(&song.title).value(&song.id)
                                                    });
                                                }

                                                o
                                            })
                                    })
                                })
                            })
                    })
                    .await
                    .unwrap();

                match m
                    .await_component_interaction(ctx)
                    .timeout(Duration::from_secs(60 * 3))
                    .await
                {
                    Some(x) => x,
                    None => {
                        m.reply(&ctx, "Timed out").await.unwrap();
                        return None;
                    }
                };

                let obj = match songbird::ytdl(format!("ytsearch1:{}", &song)).await {
                    Ok(source) => Some(source),
                    Err(why) => {
                        println!("An error ocurred {:?}", why);
                        None
                    }
                };
                if let Some(song) = obj {
                    handler.stop();
                    let response = handler.play_source(song);

                    match response.set_volume(1.0) {
                        Ok(_) => (),
                        Err(err) => println!("Failed to adjust song volume - {:?}", err),
                    } // Default to full volume.

                    command
                        .create_followup_message(&ctx.http, |f| {
                            f.ephemeral(true).content("Song played!.")
                        })
                        .await
                        .unwrap();
                } else {
                    command
                        .create_followup_message(&ctx.http, |f| {
                            f.ephemeral(true).content("Unable to handle song request.")
                        })
                        .await
                        .unwrap();
                }
            }
        }

        None
    } else {
        Some("Unable to execute command. User is not connected to a channel".into())
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("play").description("Plays a song")
}
