use fltk::{app::*, button::*, output::*, prelude::*, window::*};

const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;
const WIDGET_WIDTH: i32 = 70;

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut wind = Window::default()
        .with_size(
            WIDGET_WIDTH * 2 + 3 * WIDGET_PADDING,
            WIDGET_HEIGHT + WIDGET_PADDING * 2,
        )
        .with_label("Counter");

    let output = Output::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING, WIDGET_PADDING);
    output.set_value("0");

    let mut button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&output, WIDGET_PADDING)
        .with_label("Count");
    let mut value = 0;
    button.set_callback(move |_| {
        value += 1;
        output.set_value(&format!("{}", value));
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
