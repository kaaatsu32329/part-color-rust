extern crate image;

use iced::{
    button, executor, Align, Application, Button, Column, Command, Container, Element, Font,
    HorizontalAlignment, Image, Length, Row, Settings, Subscription, Text,
};

use iced_futures::{self, futures};

use image::*;
use image::io::Reader as ImageReader;

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
        // prepare buttons
        let select_button = Button::new(
            &mut self.select_button_state,
            Text::new("Select")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80)
        .on_press(Message::Select);

        let start_button = Button::new(
            &mut self.start_button_state,
            Text::new("Start")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80)
        .on_press(Message::Select);

        /*let img = Image::new(iced::image::Handle::from_path(format!("./image/image.jpg")))
            .width(Length::Fill)
            .height(Length::Fill);
        img.into()*/

        // prepare image
        let img = Container::new(
            Image::new("./image/image.jpg")
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Align::Center)
        .align_y(Align::Center);

        // widgets
        Column::new()
            .push(img)
            .push(
                Row::new()
                    .push(select_button)
                    .push(start_button)
                    .spacing(10),
            )
            .spacing(10)
            .padding(10)
            .align_items(Align::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // fn subscription(&self) -> Subscription<Message> {
    // }
}

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (640u32, 480u32);
    GUI::run(settings);
}
