use serenity::{
    builder::CreateApplicationCommand, client::Context, json::Value,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
};
pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    println!("I'm alive");
    if let Some(guild_id) = command.guild_id {
        println!("I am a guild");
        let song = command
            .data
            .options
            .iter()
            .find(|&f| f.name.eq("link-or-query"))
            .unwrap()
            .value
            .as_ref()
            .unwrap();

        if let Value::String(song) = song {
            println!("Im a song {}", song);
            let manager = songbird::get(ctx).await.expect("lol it broke");
            println!("{:?}", manager);

            let handler_lock = manager.join(guild_id, command.channel_id).await.0;

            // if let Some(handler_lock) = manager.get(guild_id) {
            // println!("Hello other side {:?}", b);
            // let mut handler = handler_lock.lock().await;

            let source = match songbird::ytdl(&song).await {
                Ok(source) => source,
                Err(why) => panic!("{}", why),
            };

            handler_lock.lock().await.play_source(source);
        }

        return "Oi mate".into();
    } else {
        return "Unable to execute command. User is not connected to a channel".into();
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("play").description("Plays a song")
}
