use std::env;

use chatgpt::prelude::*;
use serenity::builder::CreateApplicationCommand;

pub async fn run(client: ChatGPT) -> String {
    let token = env::var("OPEN_AI_TOKEN").expect("Expected a token in the environment");
    let client = ChatGPT::new(token).unwrap();
    // Sending a message and getting the completion
    let response = client
        .send_message("Describe in five words the Rust programming language.")
        .await
        .unwrap();

    response.message().content.clone()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("chatgpt").description("Ask Chat-GPT anything")
}
