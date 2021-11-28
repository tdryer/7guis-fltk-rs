use fltk::{app::*, enums::*, frame::*, input::*, prelude::*, window::*};

const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;
const WIDGET_WIDTH: i32 = 70;

#[derive(Clone, Copy)]
enum Message {
    CelsiusChanged,
    FahrenheitChanged,
}

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut wind = Window::default()
        .with_size(
            WIDGET_WIDTH * 4 + WIDGET_PADDING * 5,
            WIDGET_HEIGHT + WIDGET_PADDING * 2,
        )
        .with_label("TempConv");

    let (sender, receiver) = channel::<Message>();

    let mut celsius_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING, WIDGET_PADDING);
    celsius_input.set_trigger(CallbackTrigger::Changed);
    celsius_input.emit(sender, Message::CelsiusChanged);

    let celsius_frame = Frame::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&celsius_input, WIDGET_PADDING)
        .with_label("Celsius = ");

    let mut fahrenheit_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&celsius_frame, WIDGET_PADDING);
    fahrenheit_input.set_trigger(CallbackTrigger::Changed);
    fahrenheit_input.emit(sender, Message::FahrenheitChanged);

    let _fahrenheit_frame = Frame::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&fahrenheit_input, WIDGET_PADDING)
        .with_label("Fahrenheit");

    wind.end();
    wind.show();
    while app.wait() {
        match receiver.recv() {
            Some(Message::CelsiusChanged) => {
                if let Ok(celsius) = celsius_input.value().parse::<i32>() {
                    let value = f64::from(celsius) * (9.0 / 5.0) + 32.0;
                    fahrenheit_input.set_value(&format!("{}", value.round()))
                } else {
                    fahrenheit_input.set_value("");
                }
            }
            Some(Message::FahrenheitChanged) => {
                if let Ok(fahrenheit) = fahrenheit_input.value().parse::<i32>() {
                    let value = (f64::from(fahrenheit) - 32.0) * (5.0 / 9.0);
                    celsius_input.set_value(&format!("{}", value.round()))
                } else {
                    celsius_input.set_value("");
                }
            }
            None => {}
        }
    }
}
