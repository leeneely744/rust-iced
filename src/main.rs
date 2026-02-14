use iced::widget::text;
use iced::Element;

fn main() -> iced::Result {
    iced::application(|| Editor, Editor::update, Editor::view)
        .title("My Editor")
        .run()
}

struct Editor;

#[derive(Debug)]
enum Message {}

impl Editor {
    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello, world!").into()
    }
}
