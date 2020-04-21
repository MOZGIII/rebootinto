use iced::{
    button, executor, scrollable, Application, Button, Command, Container, Element, Length,
    Scrollable, Text,
};

use super::core;

pub struct Init {
    pub backend: core::Backend,
    pub load_options: Vec<core::LoadOption>,
}

struct ButtonData {
    load_option: core::LoadOption,
    state: button::State,
}

pub struct App {
    backend: core::Backend,
    scroll: scrollable::State,
    buttons: Vec<ButtonData>,
    state: State,
}

enum State {
    Choosing,
    Rebooting { index: usize },
    Error { error: super::Error },
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ButtonPressed(usize),
}

pub const LAYOUT_PADDING: u16 = 10;
pub const LAYOUT_SPACING: u16 = 10;

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Init;

    fn new(init: Self::Flags) -> (Self, Command<Self::Message>) {
        let Init {
            backend,
            load_options,
        } = init;
        let buttons = load_options
            .into_iter()
            .map(|load_option| ButtonData {
                load_option,
                state: button::State::new(),
            })
            .collect();

        let app = Self {
            backend,
            scroll: scrollable::State::new(),
            buttons,
            state: State::Choosing,
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Rebootinto")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::ButtonPressed(index) => {
                let load_option = &self.buttons[index].load_option;
                self.state = match self.backend.reboot_into(load_option.number) {
                    Ok(()) => State::Rebooting { index },
                    Err(error) => State::Error { error },
                };
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<'_, Message> {
        match self.state {
            State::Choosing => {
                let layout = Scrollable::new(&mut self.scroll)
                    .padding(LAYOUT_PADDING)
                    .spacing(LAYOUT_SPACING)
                    .width(Length::Fill)
                    .height(Length::Fill);

                self.buttons
                    .iter_mut()
                    .enumerate()
                    .fold(
                        layout,
                        |layout, (index, ButtonData { load_option, state })| {
                            layout.push(
                                Button::new(state, Text::new(load_option.description.clone()))
                                    .width(Length::Fill)
                                    .on_press(Message::ButtonPressed(index)),
                            )
                        },
                    )
                    .into()
            }
            State::Rebooting { index } => {
                let load_option = &self.buttons[index].load_option;
                text_view(format!("Rebooting into {}", load_option))
            }
            State::Error { ref error } => text_view(format!("Error: {}", error)),
        }
    }
}

fn text_view<'a>(label: impl Into<String>) -> Element<'a, Message> {
    let text = Text::new(label).width(Length::Fill).height(Length::Fill);
    Container::new(text)
        .padding(LAYOUT_PADDING)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
