use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use songbird::SerenityInit;
use std::env;

mod commands;
mod terminator;

use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::id::GuildId;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // println!("The interaction {:?} ", interaction);

        if let Interaction::ApplicationCommand(command) = interaction {
            // println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "hello" => commands::hello::run(&command.data.options),
                "play" => match commands::music::play::run(&ctx, &command).await {
                    Some(v) => v,
                    None => "".into(),
                },
                "stop" => commands::music::stop::run(&ctx, &command).await,
                _ => "not implemented :(".to_string(),
            };

            if content.len() > 0 {
                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(content))
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_option = ready.guilds.iter().find(|f| f.unavailable);

        if let Some(guild_id) = guild_option {
            let commands = GuildId::set_application_commands(&guild_id.id, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::hello::register(command))
                    .create_application_command(|command| commands::music::play::register(command))
                    .create_application_command(|command| commands::music::stop::register(command))
            })
            .await;

            println!(
                "I now have the following guild slash commands: {:#?}",
                commands
            );
        } else {
            println!(
                "Unable to find a Guild id in the ready payload - {:?}",
                ready
            );
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("CLIENT_TOKEN").expect("Discord token not found in environment file.");
    let framework = StandardFramework::new().configure(|c| c.prefix("~"));

    let mut client = Client::builder(
        token,
        GatewayIntents::GUILDS
            | GatewayIntents::GUILD_VOICE_STATES
            | GatewayIntents::GUILD_MESSAGES,
    )
    .event_handler(Handler)
    .framework(framework)
    .register_songbird()
    .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
