use serenity::{
    builder::CreateApplicationCommand, client::Context, json::Value,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
};

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
            );

            // println!("Songbird object: {:?}", manager);

            // println!("This is the user object: {:?}", command.user);
            // println!("Current channel id - {}", command.channel_id);

            // println!("Hehe boi {:?}", &ctx.cache.guild(guild_id));

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
            if let Ok(_) = conn_result {
                let mut handler = handler_lock.lock().await;

                command
                    .create_followup_message(&ctx.http, |f| {
                        f.ephemeral(true).content("Searching for song.")
                    })
                    .await
                    .unwrap();

                let obj = match songbird::input::restartable::Restartable::ytdl_search(&song, true)
                    .await
                {
                    Ok(source) => Some(source),
                    Err(why) => {
                        println!("An error ocurred {:?}", why);
                        None
                    }
                };

                if let Some(song) = obj {
                    handler.stop();
                    let response = handler.play_source(song.into());
                    println!("Response for song object - {:?}", response);
                    command
                        .create_followup_message(&ctx.http, |f| {
                            f.ephemeral(true).content("Song played!.")
                        })
                        .await
                        .unwrap();
                } else {
                    println!("Rip song didn't play");
                    command
                        .create_followup_message(&ctx.http, |f| {
                            f.ephemeral(true).content("Unable to handle song request.")
                        })
                        .await
                        .unwrap();
                }
            }
        }

        return None;
    } else {
        return Some("Unable to execute command. User is not connected to a channel".into());
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("play").description("Plays a song")
}
