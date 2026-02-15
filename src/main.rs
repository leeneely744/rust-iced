use iced::widget::{button, text, column};
use iced::Element;

fn main() -> iced::Result {
    iced::application(|| Editor, Editor::update, Editor::view)
        .title("My Editor")
        .run()
}

struct Editor;

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

impl Editor {
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                println!("Button pressed in update!");
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text("Hello, world!"),
            button("Press me!").on_press(Message::ButtonPressed),
        ].into()
    }
}
