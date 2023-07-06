use iced::{
    alignment::{Horizontal, Vertical},
    executor,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, row, scrollable, text,
        text_input, Container,
    },
    Alignment, Application, Command, Font, Length, Settings,
};
use iced_native::Pixels;
use search::{
    components::tags::itag,
    styles::modern::{
        self,
        modern_widget::{Element, Renderer, Row, Text},
        ModernButton, ModernColor, ModernContainer, ModernTheme,
    },
};

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() -> iced::Result {
    App::run(Settings {
        default_font: Some(include_bytes!("fonts/Inter-Regular.otf")),
        ..Default::default()
    })
}

struct Inputs {
    pub query: String,
    pub enabled: bool,
}

struct App {
    theme: ModernTheme,
    toggler: bool,
    inputs: Inputs,
    searches: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    OnPressing,
    TagSelected(String /* name of the tag */),
    QueryChange(String),
    OnChangingTheme(bool),
    SetSearch(String),
    RemoveSearch(usize),
}

const ICON_FONT: Font = Font::External {
    name: "icons",
    bytes: include_bytes!("fonts/bootstrap-icons.ttf"),
};

const BOLD_FONT: Font = Font::External {
    name: "bold font",
    bytes: include_bytes!("fonts/Inter-Bold.otf"),
};

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = ModernTheme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                theme: ModernTheme::Dark,
                toggler: false,
                inputs: Inputs {
                    query: String::new(),
                    enabled: true,
                },
                searches: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Capy search".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::OnPressing => {
                if !self.inputs.query.trim().is_empty() {
                    self.searches
                        .insert(0, self.inputs.query.clone().trim().into())
                }
            }
            Message::OnChangingTheme(state) => {
                self.toggler = state;
                self.theme = if self.toggler {
                    ModernTheme::Light
                } else {
                    ModernTheme::Dark
                }
            }
            Message::QueryChange(query) | Message::SetSearch(query) => self.inputs.query = query,
            Message::TagSelected(tag) => {
                println!("{tag}")
            }
            Message::RemoveSearch(id) => {
                println!("removing: {}", self.searches[id]);
                self.searches.remove(id);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        static PLACEHOLDERS: [&str; 3] = [
            "Search anything...",
            "Give me your question...",
            "Let step on that errors...",
        ];
        let mut rng = thread_rng();

        let placeholder = *PLACEHOLDERS.choose(&mut rng).unwrap();

        let title = container(
            column![
                Text::new("Capy")
                    .size(75)
                    .font(BOLD_FONT)
                    .style(ModernColor::Custom(252.0, 187.0, 150.0)),
                Text::new("Programmer search engine").size(18)
            ]
            .spacing(15)
            .align_items(Alignment::Center),
        );

        let tags = row(vec![
            itag(
                "images/stack-overflow.png",
                (252.0, 187.0, 150.0),
                Message::TagSelected("overflow".into()),
            )
            .into(),
            itag(
                "images/stack-exchange.png",
                (175.0, 197.0, 226.0),
                Message::TagSelected("exchange".into()),
            )
            .into(),
            itag(
                "images/geek-for-geeks.png",
                (96.0, 177.0, 121.0),
                Message::TagSelected("geeks".into()),
            )
            .into(),
        ])
        .spacing(10);

        let input_and_button = container(
            row![
                text_input(placeholder, &self.inputs.query)
                    .on_input(Message::QueryChange)
                    .padding([12, 20]),
                button(icon('\u{F144}', 16))
                    .height(30)
                    .width(30)
                    .padding(6.2)
                    .style(if self.inputs.query.trim().is_empty() {
                        ModernButton::Secondary
                    } else {
                        ModernButton::Principal
                    })
                    .on_press(Message::OnPressing),
            ]
            .width(595)
            .align_items(Alignment::Center),
        )
        .width(610)
        .center_x()
        .center_y()
        .style(ModernContainer::Input);

        let principal_container: container::Container<Message, Renderer> = container(
            column![title, input_and_button]
                .align_items(Alignment::Center)
                .spacing(30),
        );

        let historial_container = if self.searches.is_empty() {
            empty_message("You didn't searched anything yet...")
        } else {
            show_historial(&self.searches)
        };

        let principal_box = container(container(
            column![
                principal_container,
                tags,
                horizontal_rule(1),
                historial_container
            ]
            .align_items(Alignment::Center)
            .spacing(15),
        ))
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        principal_box.into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme
    }
}

fn icon(unicode: char, size: impl Into<Pixels>) -> Text<'static> {
    text(unicode.to_string())
        .font(ICON_FONT)
        .horizontal_alignment(Horizontal::Center)
        .size(size)
}

fn historial_text(query: &str, id: usize) -> Element<'static, Message> {
    Row::new()
        .push(
            text(query)
                .size(18)
                .style(ModernColor::Custom(160.0, 160.0, 160.0)),
        )
        .push(horizontal_space(10))
        .push(
            button(icon('\u{F62A}', 18).style(ModernColor::Custom(160.0, 160.0, 160.0)))
                .on_press(Message::RemoveSearch(id))
                .style(ModernButton::Text),
        )
        .align_items(Alignment::Center)
        .into()
}

fn show_historial(queries: &[String]) -> modern::modern_widget::Container<'static, Message> {
    let data: Vec<Element<Message>> = queries
        .iter()
        .enumerate()
        .map(|(id, q)| historial_text(q.trim(), id))
        .collect();
    container(
        scrollable(
            column(data)
                .padding([20, 30])
                .align_items(Alignment::Start)
                .spacing(5),
        )
        .width(580),
    )
    .width(610)
    .height(200)
    .style(ModernContainer::Historial)
}

fn empty_message(msg: &str) -> Container<'_, Message, Renderer> {
    container(
        text(msg)
            .width(Length::Fill)
            .size(20)
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center)
            .style(ModernColor::Custom(82.0, 81.0, 90.0)),
    )
    .width(610)
    .height(200)
    .center_x()
    .center_y()
    .style(ModernContainer::Historial)
}
