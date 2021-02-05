use fltk::{app::*, button::*, frame::*, input::*, window::*};

const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;
const WIDGET_WIDTH: i32 = 70;

#[derive(Clone)]
enum Message {
    CelsiusChanged,
    FahrenheitChanged,
}

fn main() {
    let app = App::default();
    let mut wind = Window::default()
        .with_size(
            WIDGET_WIDTH * 4 + WIDGET_PADDING * 5,
            WIDGET_HEIGHT + WIDGET_PADDING * 2,
        )
        .with_label("TempConv");

    let (sender, reciever) = channel::<Message>();

    let mut celsius_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING, WIDGET_PADDING);
    celsius_input.set_trigger(CallbackTrigger::Changed);
    celsius_input.emit(sender.clone(), Message::CelsiusChanged);

    let celsius_frame = Frame::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&celsius_input, WIDGET_PADDING)
        .with_label("Celsius = ");

    let mut fahrenheit_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&celsius_frame, WIDGET_PADDING);
    fahrenheit_input.set_trigger(CallbackTrigger::Changed);
    fahrenheit_input.emit(sender.clone(), Message::FahrenheitChanged);

    let _fahrenheit_frame = Frame::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&fahrenheit_input, WIDGET_PADDING)
        .with_label("Fahrenheit");

    wind.end();
    wind.show_with_args(&["-scheme", "gtk+"]);

    while app.wait() {
        match reciever.recv() {
            Some(Message::CelsiusChanged) => {
                if let Some(celsius) = celsius_input.value().parse::<i32>().ok() {
                    let value = f64::from(celsius) * (9.0 / 5.0) + 32.0;
                    fahrenheit_input.set_value(&format!("{}", value.round()))
                } else {
                    fahrenheit_input.set_value("");
                }
            }
            Some(Message::FahrenheitChanged) => {
                if let Some(fahrenheit) = fahrenheit_input.value().parse::<i32>().ok() {
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