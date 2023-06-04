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

        let manager = songbird::get(ctx)
            .await
            .expect(
                "Failed to retrieve Songbird. Check if Songbird is registered on ClientBuilder.",
            )
            .clone();

        let has_handler = manager.get(guild_id).is_some();

        if has_handler {
            if let Err(e) = manager.remove(guild_id).await {
                "Failed to leave the voice channel";
            }

            "Bot has left the voice channel.";
        } else {
            "User is not in a voice channel.";
        }
    }

    return "Unable to execute command. User is not connected to a channel".into();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("stop").description("Stops a song")
}
