use fltk::{app::*, button::*, output::*, window::*};

const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;
const WIDGET_WIDTH: i32 = 70;

#[derive(Clone)]
enum Message {
    Increment,
}

fn main() {
    let app = App::default();
    let mut wind = Window::default()
        .with_size(
            WIDGET_WIDTH * 2 + 3 * WIDGET_PADDING,
            WIDGET_HEIGHT + WIDGET_PADDING * 2,
        )
        .with_label("Counter");

    let (sender, reciever) = channel::<Message>();
    let mut value = 0;

    let output = Output::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING, WIDGET_PADDING);
    output.set_value("0");

    let mut button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&output, WIDGET_PADDING)
        .with_label("Count");
    button.emit(sender, Message::Increment);

    wind.end();
    wind.show_with_args(&["-scheme", "gtk+"]);

    while app.wait() {
        match reciever.recv() {
            Some(Message::Increment) => {
                value += 1;
                output.set_value(&format!("{}", value));
            }
            None => {}
        }
    }
}
