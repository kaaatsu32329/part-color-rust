extern crate image;

use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};

use iced_futures::{self, futures};

use image::*;

#[derive(Debug, Clone)]
pub enum Message {
    Select,
    Start,
    Color,
}

pub enum Color {
    Clear = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../font/PixelMplus12-Regular.ttf"),
};

struct GUI {
    select_button_state: button::State,
    start_button_state: button::State,
    color_select: i32,
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                select_button_state: button::State::new(),
                start_button_state: button::State::new(),
                color_select: 0,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Part-ColoRS")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello World!").into()
    }

    // fn subscription(&self) -> Subscription<Message> {
    // }
}

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (640u32, 480u32);
    GUI::run(settings);
}
