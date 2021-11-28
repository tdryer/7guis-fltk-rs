use fltk::{app::*, button::*, enums::*, frame::*, misc::*, prelude::*, valuator::*, window::*};
use std::thread;
use std::time::Duration;

const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;
const WIDGET_WIDTH: i32 = 180;
const WIDGET_LABEL_WIDTH: i32 = 110;

const DURATION_DEFAULT: f64 = 15.0;
const DURATION_MAXIMUM: f64 = 30.0;

#[derive(Clone, Copy)]
enum Message {
    Reset,
    ChangeDuration,
    Tick,
}

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut wind = Window::default()
        .with_size(
            WIDGET_LABEL_WIDTH + WIDGET_WIDTH + WIDGET_PADDING * 2,
            WIDGET_HEIGHT * 4 + WIDGET_PADDING * 5,
        )
        .with_label("Timer");

    let (sender, receiver) = channel::<Message>();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        sender.send(Message::Tick);
    });

    let mut elapsed_progress = Progress::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING + WIDGET_LABEL_WIDTH, WIDGET_PADDING)
        .with_align(Align::Left)
        .with_label("Elapsed Time:");
    elapsed_progress.set_selection_color(Color::Blue);
    elapsed_progress.set_maximum(DURATION_DEFAULT);

    let mut elapsed_frame = Frame::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&elapsed_progress, WIDGET_PADDING)
        .with_label("0.0s")
        .with_align(Align::Inside | Align::Left);

    let mut duration_slider = HorSlider::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&elapsed_progress, WIDGET_HEIGHT + WIDGET_PADDING * 2)
        .with_align(Align::Left)
        .with_label("Duration:");
    duration_slider.set_value(DURATION_DEFAULT);
    duration_slider.set_maximum(DURATION_MAXIMUM);
    duration_slider.emit(sender, Message::ChangeDuration);

    let mut reset_button = Button::default()
        .with_size(WIDGET_LABEL_WIDTH + WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING, WIDGET_HEIGHT * 3 + WIDGET_PADDING * 4)
        .with_label("Reset");
    reset_button.emit(sender, Message::Reset);

    wind.end();
    wind.show();
    while app.wait() {
        match receiver.recv() {
            Some(Message::Reset) => {
                elapsed_progress.set_value(0.0);
            }
            Some(Message::ChangeDuration) => {
                elapsed_progress.set_maximum(duration_slider.value());
            }
            Some(Message::Tick) => {
                if duration_slider.value() - elapsed_progress.value() >= 0.01 {
                    elapsed_progress.set_value(elapsed_progress.value() + 0.1);
                    elapsed_frame.set_label(&format!("{:.1}s", elapsed_progress.value()));
                }
            }
            None => {}
        }
    }
}
