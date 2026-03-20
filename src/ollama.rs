use rig::agent::Agent;
use rig::client::{CompletionClient, Nothing, ProviderClient};
use rig::providers::ollama;

pub struct MyOllama;

impl MyOllama {
    pub fn generate_agent(&self) -> Agent<ollama::CompletionModel> {
        let client = ollama::Client::from_val(Nothing);
        client
            .agent("llama3.2")
            .preamble("You are a chatbot.")
            .temperature(0.7)
            .build()
    }
}
