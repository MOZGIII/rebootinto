//! The iced app.

use iced::{
    button, executor, scrollable, Application, Button, Command, Container, Element, Length,
    Scrollable, Text,
};

use super::core;

/// Initialization params.
pub struct Init {
    /// The core backend.
    pub backend: core::Backend,
    /// The load options.
    pub load_options: Vec<core::LoadOption>,
}

/// The button data.
struct ButtonData {
    /// The load option of the button.
    load_option: core::LoadOption,
    /// The button state.
    state: button::State,
}

/// The app.
pub struct App {
    /// The core backend.
    backend: core::Backend,
    /// The scroll state.
    scroll: scrollable::State,
    /// The buttons.
    buttons: Vec<ButtonData>,
    /// The app state.
    state: State,
}

/// The possible states of the app.
enum State {
    /// User is choosing the input.
    Choosing,
    /// Reboot is in progress.
    Rebooting {
        /// The index of the load option.
        index: usize,
    },
    /// An error has occured.
    Error {
        /// The error that occured.
        error: core::RebootIntoError,
    },
}

/// The application control messages.
#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// The button is pressed.
    ButtonPressed(usize),
}

/// The standard layout padding.
pub const LAYOUT_PADDING: u16 = 10;
/// The standard layout spacing.
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

/// Render text with the standard padding.
fn text_view<'a>(label: impl Into<String>) -> Element<'a, Message> {
    let text = Text::new(label).width(Length::Fill).height(Length::Fill);
    Container::new(text)
        .padding(LAYOUT_PADDING)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
