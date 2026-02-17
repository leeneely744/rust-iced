use std::path::PathBuf;

use iced::widget::{Column, button, text, text_input};
use iced::Element;
use rfd::FileDialog;

fn main() -> iced::Result {
    iced::application(|| Editor { lines: Vec::new(), input_text: String::new() }, Editor::update, Editor::view)
        .title("My Editor")
        .run()
}

struct Editor {
    lines: Vec<String>,
    input_text: String,
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
}

impl Editor {
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonFileSelect => {
                let file = FileDialog::new()
                    .add_filter("picture", &["png", "jpg", "jpeg"])
                    .set_directory("/")
                    .pick_file();
                match file {
                    Some(file) => {
                        println!("picked file path: {}", file.as_path().display())
                    }
                    None => {
                        println!("file picking is failed!")
                    }
                }
            }
            Message::InputChanged(value) => {
                self.input_text = value;
            }
            Message::ButtonPressed => {
                self.push_line(&self.input_text.clone());
                self.input_text.clear();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = Column::new()
            .push(button("File Select").on_press(Message::ButtonFileSelect))
            .push(text_input("Type something...", &self.input_text).on_input(Message::InputChanged))
            .push(button("Send").on_press(Message::ButtonPressed));
        let content = self.lines.iter().fold(content, |content, l| {
            content.push(text(l))
        });
        content.into()
    }
}
