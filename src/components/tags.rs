use std::fs;

use crate::styles::modern::{modern_widget::Button, ModernButton, RGBColor};

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, text, Image},
    Length,
};
use iced_native::image;

pub fn tag<Message>(content: &str, color: RGBColor, message: Message) -> Button<Message> {
    button(
        text(content)
            .size(20)
            .width(Length::Shrink)
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center),
    )
    .style(ModernButton::Tag(color))
    .padding([5, 10])
    .on_press(message)
}

pub fn itag<Message>(path: &str, color: RGBColor, message: Message) -> Button<Message> {
    let file = fs::read(format!("./src/{}", path)).unwrap();
    let content = Image::new(image::Handle::from_memory(file));
    button(content.height(30).width(100))
        .padding([0, 10])
        .height(30)
        .width(Length::Shrink)
        .style(ModernButton::Tag(color))
        .on_press(message)
}
