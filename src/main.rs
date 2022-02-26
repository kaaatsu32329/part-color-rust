pub mod cvision;

#[allow(unused_imports)]
use iced::{
    button, executor, Align, Application, Button, Column, Command, Container, Element, Font,
    HorizontalAlignment, Image, Length, Row, Settings, Subscription, Text,
};
use image::*;
use native_dialog::FileDialog;
use std::path::PathBuf;
use covrus;

pub use crate::cvision::*;

#[derive(Debug, Clone)]
pub enum Message {
    Select,
    Start,
    Color,
    Save,
}

pub enum Color {
    Clear = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

const FONT: Font = Font::External {
    //name: "PixelMplus12-Regular",
    //bytes: include_bytes!("../font/PixelMplus12-Regular.ttf"),
    name: "NotoSans-Bold",
    bytes: include_bytes!("../font/NotoSans-Bold.ttf"),
};

struct GUI {
    select_button_state: button::State,
    start_button_state: button::State,
    color_select_button_state: button::State,
    save_button_state: button::State,
    color_select: i8,
    target_image_path: PathBuf,
    processed_image: ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
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
                color_select_button_state: button::State::new(),
                save_button_state: button::State::new(),
                color_select: 0,
                target_image_path: PathBuf::new(),
                processed_image: ImageBuffer::new(1080, 1920),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Part-ColoRS")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Select => {
                self.target_image_path = FileDialog::new()
                    .set_location("~")
                    .add_filter("Image", &["png", "jpg", "jpeg"])
                    .show_open_single_file()
                    .unwrap()
                    .unwrap();
                self.processed_image = image::open(&self.target_image_path).unwrap().to_rgb8();
            }
            Message::Start => {
                // Part color processing
                let target_image = covrus::cvt_img2array(&self.processed_image);
                let target_image = part_color(&target_image, 1u8);
                self.processed_image = covrus::cvt_array2img(&target_image);
            }
            Message::Color => {
                self.color_select = 0;
            }
            Message::Save => {
                let save_path = FileDialog::new()
                    .set_location("~")
                    .show_save_single_file()
                    .unwrap()
                    .unwrap();
                self.processed_image.save(save_path).unwrap();
            }
        }
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
        .on_press(Message::Start);

        let color_select_button = Button::new(
            &mut self.color_select_button_state,
            Text::new("Color")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80)
        .on_press(Message::Color);

        let save_button = Button::new(
            &mut self.save_button_state,
            Text::new("Save")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80)
        .on_press(Message::Save);

        // prepare image
        let img = Container::new(
            Image::new(&self.target_image_path)
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
                    .push(color_select_button)
                    .push(start_button)
                    .push(save_button)
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
