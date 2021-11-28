use chrono::{offset::Local, NaiveDate};
use fltk::{app::*, button::*, dialog::*, enums::*, input::*, menu::*, prelude::*, window::*};

const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;
const WIDGET_WIDTH: i32 = 200;

#[derive(Clone, Copy)]
enum Message {
    Update,
    Book,
}

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut wind = Window::default()
        .with_size(
            WIDGET_WIDTH + WIDGET_PADDING * 2,
            WIDGET_HEIGHT * 4 + WIDGET_PADDING * 5,
        )
        .with_label("Book Flight");

    let (sender, receiver) = channel::<Message>();

    let mut choice = Choice::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING, WIDGET_PADDING);
    choice.add_choice("one-way flight");
    choice.add_choice("return flight");
    let one_way_flight_index = 0;
    choice.set_value(one_way_flight_index);
    choice.emit(sender, Message::Update);

    let current_date = Local::now().naive_local().date();

    let mut start_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&choice, WIDGET_PADDING);
    start_input.set_trigger(CallbackTrigger::Changed);
    start_input.emit(sender, Message::Update);
    start_input.set_value(&current_date.format("%Y-%m-%d").to_string());

    let mut return_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&start_input, WIDGET_PADDING);
    return_input.deactivate();
    return_input.set_trigger(CallbackTrigger::Changed);
    return_input.emit(sender, Message::Update);
    return_input.set_value(&current_date.format("%Y-%m-%d").to_string());

    let mut book_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&return_input, WIDGET_PADDING)
        .with_label("Book");
    book_button.emit(sender, Message::Book);

    wind.end();
    wind.show();
    while app.wait() {
        match receiver.recv() {
            Some(Message::Update) => {
                if choice.value() == one_way_flight_index {
                    return_input.deactivate();
                    if get_date(&mut start_input).is_ok() {
                        book_button.activate();
                    } else {
                        book_button.deactivate();
                    }
                } else {
                    return_input.activate();
                    let start_date = get_date(&mut start_input);
                    let return_date = get_date(&mut return_input);
                    if start_date.is_ok()
                        && return_date.is_ok()
                        && start_date.unwrap() <= return_date.unwrap()
                    {
                        book_button.activate();
                    } else {
                        book_button.deactivate();
                    }
                }
            }
            Some(Message::Book) => alert_default(&if choice.value() == one_way_flight_index {
                format!(
                    "You have booked a one-way flight for {}.",
                    start_input.value()
                )
            } else {
                format!(
                    "You have booked a return flight from {} to {}",
                    start_input.value(),
                    return_input.value()
                )
            }),
            None => {}
        }
    }
}

fn get_date(input: &mut Input) -> Result<NaiveDate, chrono::ParseError> {
    let date = NaiveDate::parse_from_str(&input.value(), "%Y-%m-%d");
    input.set_color(match date {
        Ok(_) => Color::BackGround2,
        Err(_) => Color::Red,
    });
    input.redraw();
    date
}
