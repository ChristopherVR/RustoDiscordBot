use std::{path::PathBuf, time::Duration};

use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    futures::StreamExt,
    json::Value,
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
};
use youtube_dl::SearchOptions;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
    if let Some(guild_id) = command.guild_id {
        let manager = songbird::get(ctx).await.expect(
            "Failed to retrieve Songbird. Check if Songbird is registered on ClientBuilder.",
        );
        let channel_id = &ctx
            .cache
            .guild(guild_id)
            .unwrap()
            .voice_states
            .get(&command.user.id)
            .and_then(|voice_state| voice_state.channel_id)
            .expect("User needs to be connected to a voice channel.");

        let (handler_lock, conn_result) = manager.join(guild_id, channel_id.0).await;

        if let Ok(_) = conn_result {
            let mut handler = handler_lock.lock().await;

            let send_http = ctx.http.clone();
            std::mem::drop(handler);
            return Some("Song stopped.".into());
        }
    }
    return None;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("back")
        .description("Goes back to the previous track.")
}
