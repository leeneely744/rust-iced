use std::path::PathBuf;

use iced::widget::{Column, button, text, text_input, Image};
use iced::{Element, Task};

use rfd::FileDialog;
use rig::completion::Prompt;
use rig::{providers::openai};
use rig::agent::Agent;
use rig::client::{ProviderClient, CompletionClient};

fn main() -> iced::Result {
    iced::application(
        || {
            let client = openai::Client::from_env(); // need env variable OPENAI_API_KEY
            let agent = client.agent("gpt-4")
                .preamble("You are a chatbot.")
                .temperature(0.7)
                .build();
            
            Editor {
                lines: Vec::new(),
                input_text: String::new(),
                image_path: None,
                agent
            }
        },
        Editor::update,
        Editor::view
    )
    .title("My Editor")
    .run()
}

struct Editor {
    lines: Vec<String>,
    input_text: String,
    image_path: Option<PathBuf>,
    agent: Agent<openai::responses_api::ResponsesCompletionModel>,
}
impl Editor {
    fn push_line(&mut self, new_line: &str) {
        self.lines.push(new_line.to_string());
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ButtonPressed,
    ButtonFileSelect,
    ChatSucceeded(String),
    ChatFailed(String),
}

impl Editor {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ButtonFileSelect => {
                let file = FileDialog::new()
                    .add_filter("picture", &["png", "jpg", "jpeg"])
                    .set_directory("/")
                    .pick_file();
                match file {
                    Some(file) => {
                        self.image_path = Some(file);
                    }
                    None => {
                        println!("file picking is failed!");
                    }
                }
                Task::none()
            }
            Message::InputChanged(value) => {
                self.input_text = value;
                Task::none()
            }
            Message::ButtonPressed => {
                let input_text = self.input_text.clone();
                if input_text.trim().is_empty() {
                    return Task::none();
                }

                self.push_line(&input_text);
                self.input_text.clear();
                let agent = self.agent.clone();

                Task::perform(
                    async move { agent.prompt(input_text).await },
                    |result| match result {
                        Ok(response) => Message::ChatSucceeded(response),
                        Err(error) => Message::ChatFailed(error.to_string()),
                    },
                )
            }
            Message::ChatSucceeded(response) => {
                self.push_line(&response);
                Task::none()
            }
            Message::ChatFailed(error) => {
                self.push_line(&format!("Error: {error}"));
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut content = Column::new()
            .push(button("File Select").on_press(Message::ButtonFileSelect));
        if let Some(ref path) = self.image_path {
            content = content.push(Image::new(path.as_path())).width(300);
        }
        let content = content
            .push(text_input("Type something...", &self.input_text).on_input(Message::InputChanged))
            .push(button("Send").on_press(Message::ButtonPressed));
        let content = self.lines.iter().fold(content, |content, l| {
            content.push(text(l))
        });
        content.into()
    }
}
