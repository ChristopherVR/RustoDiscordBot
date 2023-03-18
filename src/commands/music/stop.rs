use serenity::{
    builder::CreateApplicationCommand, client::Context,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
};

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    println!("Running stop command.");
    if let Some(guild_id) = command.guild_id {
        let channel_id = &ctx
            .cache
            .guild(guild_id)
            .unwrap()
            .voice_states
            .get(&command.user.id)
            .and_then(|voice_state| voice_state.channel_id)
            .expect("User needs to be connected to a voice channel.");
        let manager = songbird::get(ctx).await.expect(
            "Failed to retrieve Songbird. Check if Songbird is registered on ClientBuilder.",
        );
        let (handler_lock, conn_result) = manager.join(guild_id, channel_id.0).await;

        if let Ok(_) = conn_result {
            let mut handler = handler_lock.lock().await;

            handler.stop();
            std::mem::drop(handler);

            return "Song stopped.".into();
        }
    }

    return "Unable to execute command. User is not connected to a channel".into();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("stop").description("Stops a song")
}
