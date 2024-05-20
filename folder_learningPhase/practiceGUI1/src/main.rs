use iced::{Sandbox, Settings, Element};
use iced::widget::{Button, Column, Text, button::State as ButtonState};

pub struct Counter {
    value: i32,
    increment_button: ButtonState,
    decrement_button: ButtonState,
    reset_button: ButtonState,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
    Reset,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            value: 0,
            increment_button: Button::State::new(),
            decrement_button: Button::State::new(),
            reset_button: Button::State::new(),
        }
    }
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Counter {
        Counter::new()
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            Message::Reset => self.value = 0,
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::Increment),
            )
            .push(
                Text::new(self.value.to_string()),
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::Decrement),
            )
            .push(
                Button::new(&mut self.reset_button, Text::new("Reset"))
                    .on_press(Message::Reset),
            )
            .into()
    }
}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}