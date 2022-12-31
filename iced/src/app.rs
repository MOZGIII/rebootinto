//! The iced app.

use iced::{
    alignment, executor,
    widget::{button, container, scrollable, text, Column},
    Application, Command, Element, Length, Theme,
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
}

/// The app.
pub struct App {
    /// The core backend.
    backend: core::Backend,
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
    type Theme = Theme;
    type Flags = Init;

    fn new(init: Self::Flags) -> (Self, Command<Self::Message>) {
        let Init {
            backend,
            load_options,
        } = init;
        let buttons = load_options
            .into_iter()
            .map(|load_option| ButtonData { load_option })
            .collect();

        let app = Self {
            backend,
            buttons,
            state: State::Choosing,
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Rebootinto")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
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

    fn view(&self) -> Element<'_, Message> {
        match self.state {
            State::Choosing => {
                let button = |index: usize, load_option: &core::LoadOption| {
                    button(
                        text(&load_option.description)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .width(Length::Fill)
                    .on_press(Message::ButtonPressed(index))
                    .into()
                };

                let buttons = Column::with_children(
                    self.buttons
                        .iter()
                        .enumerate()
                        .map(|(index, ButtonData { load_option })| button(index, load_option))
                        .collect(),
                )
                .padding(LAYOUT_PADDING)
                .spacing(LAYOUT_SPACING);

                container(scrollable(buttons))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
            State::Rebooting { index } => {
                let load_option = &self.buttons[index].load_option;
                text_view(format!("Rebooting into {load_option}"))
            }
            State::Error { ref error } => text_view(format!("Error: {error}")),
        }
    }
}

/// Render text with the standard padding.
fn text_view<'a>(label: impl ToString) -> Element<'a, Message> {
    let text = text(label).width(Length::Fill).height(Length::Fill);
    container(text)
        .padding(LAYOUT_PADDING)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
