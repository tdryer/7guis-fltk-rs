use {
    cascade::cascade,
    fltk::{app::*, button::Button, enums::Event, output::Output, prelude::*, window::Window},
    std::{cell::RefCell, rc::Rc},
};

const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;
const WIDGET_WIDTH: i32 = 70;
const UPDATE: Event = Event::from_i32(404);

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let state = Rc::new(RefCell::new(0u8));
    cascade!(
        Window::default().with_size(
            WIDGET_WIDTH * 2 + 3 * WIDGET_PADDING,
            WIDGET_HEIGHT + WIDGET_PADDING * 2,
        );
        ..set_label("Counter");
        ..set_callback(move |_| {
            if event() == Event::Close {
                quit();
            }
        });
        ..add(&cascade!(
            Output::default();
            ..set_size(WIDGET_WIDTH, WIDGET_HEIGHT);
            ..set_pos(WIDGET_PADDING, WIDGET_PADDING);
            ..handle({
                let state = state.clone();
                move |output, event| {
                    if event == UPDATE {
                        output.set_value(&state.borrow().to_string());
                        return true;
                    };
                    false
                }
            });
            ..handle_event(UPDATE);
        ));
        ..add(&cascade!(
            Button::default();
            ..set_size(WIDGET_WIDTH, WIDGET_HEIGHT);
            ..set_pos(WIDGET_PADDING * 2 + WIDGET_WIDTH, WIDGET_PADDING);
            ..set_label("Count");
            ..set_callback({
                let state = state.clone();
                move |_| {
                    *state.borrow_mut() += 1;
                    handle_main(UPDATE).unwrap();
                }
            });
        ));
        ..end();
    )
    .show();
    app.run().unwrap();
}
