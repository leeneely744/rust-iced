mod ollama;

use std::io::Cursor;
use std::path::PathBuf;

use base64::{Engine as _, engine::general_purpose};
use iced::widget::{Column, Image, button, text, text_input};
use iced::{Element, Task};
use image::ImageFormat;
use rfd::FileDialog;
use rig::agent::Agent;
use rig::completion::Prompt;
use rig::providers::ollama::CompletionModel as OllamaCompletionModel;

fn main() -> iced::Result {
    iced::application(
        || {
            let agent = ollama::MyOllama.generate_agent();

            Editor {
                lines: Vec::new(),
                input_text: String::new(),
                image_path: None,
                agent,
            }
        },
        Editor::update,
        Editor::view,
    )
    .title("My Editor")
    .run()
}

struct Editor {
    lines: Vec<String>,
    input_text: String,
    image_path: Option<PathBuf>,
    agent: Agent<OllamaCompletionModel>,
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

async fn chat_with_image(prompt: String, image_path: PathBuf) -> Result<String, String> {
    let img = image::open(&image_path).map_err(|e| e.to_string())?;
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::Png)
        .map_err(|e| e.to_string())?;
    let b64 = general_purpose::STANDARD.encode(buf.into_inner());

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": "llava",
        "prompt": prompt,
        "images": [b64],
        "stream": false
    });

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    Ok(json["response"]
        .as_str()
        .unwrap_or("No response")
        .to_string())
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

                if let Some(image_path) = self.image_path.clone() {
                    Task::perform(
                        chat_with_image(input_text, image_path),
                        |result| match result {
                            Ok(response) => Message::ChatSucceeded(response),
                            Err(error) => Message::ChatFailed(error),
                        },
                    )
                } else {
                    let agent = self.agent.clone();
                    Task::perform(
                        async move { agent.prompt(input_text).await },
                        |result| match result {
                            Ok(response) => Message::ChatSucceeded(response),
                            Err(error) => Message::ChatFailed(error.to_string()),
                        },
                    )
                }
            }
            Message::ChatSucceeded(response) => {
                self.push_line(&response);
                println!("success");
                Task::none()
            }
            Message::ChatFailed(error) => {
                self.push_line(&format!("Error: {error}"));
                println!("failed");
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
            .push(
                text_input("Type something...", &self.input_text)
                    .on_input(Message::InputChanged),
            )
            .push(button("Send").on_press(Message::ButtonPressed));
        let content = self.lines.iter().fold(content, |content, l| {
            content.push(text(l))
        });
        content.into()
    }
}
