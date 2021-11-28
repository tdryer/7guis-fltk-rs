use fltk::{
    app::*, button::*, draw::*, enums::*, menu::*, prelude::*, valuator::*, widget::*, window::*,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

const WIDGET_WIDTH: i32 = 70;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;

const CANVAS_WIDTH: i32 = 350;
const CANVAS_HEIGHT: i32 = 250;

const MOUSE_LEFT: i32 = 1;
const MOUSE_RIGHT: i32 = 3;

type Coord = (i32, i32);
type Radius = i32;

fn distance(a: Coord, b: Coord) -> i32 {
    f64::sqrt(f64::from(i32::pow(a.0 - b.0, 2) + i32::pow(a.1 - b.1, 2))) as i32
}

struct Model {
    sender: Sender<Message>,
    states: Vec<HashMap<Coord, Radius>>,
    current_state_index: usize,
}
impl Model {
    fn new(sender: Sender<Message>) -> Self {
        Self {
            sender,
            states: vec![HashMap::new()],
            current_state_index: 0,
        }
    }
    fn save(&mut self) {
        for _ in self.current_state_index + 1..self.states.len() {
            self.states.pop();
        }
        self.states
            .push(self.states[self.current_state_index].clone());
        self.current_state_index += 1;
    }
    fn undo(&mut self) {
        assert!(self.can_undo());
        self.current_state_index -= 1;
        self.sender.send(Message::ModelChanged);
    }
    fn can_undo(&self) -> bool {
        self.current_state_index > 0
    }
    fn redo(&mut self) {
        assert!(self.can_redo());
        self.current_state_index += 1;
        self.sender.send(Message::ModelChanged);
    }
    fn can_redo(&self) -> bool {
        self.current_state_index + 1 < self.states.len()
    }
    fn set(&mut self, coord: Coord, radius: Radius) {
        self.states[self.current_state_index].insert(coord, radius);
        self.sender.send(Message::ModelChanged);
    }
    fn circles(&self) -> &HashMap<Coord, Radius> {
        &self.states[self.current_state_index]
    }
}

struct Canvas {
    widget: Widget,
    circles: Rc<RefCell<HashMap<Coord, Radius>>>,
    selected: Rc<RefCell<Option<(Coord, Radius)>>>,
}

impl Canvas {
    fn new(x: i32, y: i32, width: i32, height: i32, sender: Sender<Message>) -> Self {
        let mut canvas = Canvas {
            widget: Widget::new(x, y, width, height, ""),
            circles: Rc::default(),
            selected: Rc::default(),
        };
        canvas.widget.set_trigger(CallbackTrigger::Release);

        let circles = canvas.circles.clone();
        let selected = canvas.selected.clone();
        canvas.widget.handle(move |widget, event| {
            let event_pos = (event_x() - widget.x(), event_y() - widget.y());
            match event {
                Event::Enter => true,
                Event::Move => {
                    let new_selection = circles
                        .borrow()
                        .iter()
                        .map(|(pos, radius)| (*pos, *radius))
                        .filter(|(pos, radius)| distance(*pos, event_pos) <= *radius)
                        .min_by_key(|(pos, _radius)| distance(*pos, event_pos));
                    if new_selection != *selected.borrow() {
                        widget.redraw();
                        selected.replace(new_selection);
                    }
                    true
                }
                Event::Released if event_button() == MOUSE_LEFT => {
                    if selected.borrow().is_none() {
                        sender.send(Message::Add(event_pos));
                    }
                    true
                }
                Event::Released if event_button() == MOUSE_RIGHT => {
                    let selected = *selected.borrow(); // Limit borrow lifetime.
                    if selected.is_some() {
                        let menu = MenuItem::new(&["Adjust diameter..."]);
                        if menu.popup(event_x(), event_y()).is_some() {
                            sender.send(Message::AdjustOpened);
                        }
                    }
                    true
                }
                _ => false,
            }
        });

        let circles = canvas.circles.clone();
        let selected = canvas.selected.clone();
        canvas.widget.draw(move |wid| {
            push_clip(wid.x(), wid.y(), wid.width(), wid.height());
            draw_rect_fill(wid.x(), wid.y(), wid.width(), wid.height(), Color::White);
            for (pos, radius) in &*circles.borrow() {
                let draw_x = wid.x() + pos.0 - radius;
                let draw_y = wid.y() + pos.1 - radius;
                let diameter = radius * 2;
                if matches!(*selected.borrow(), Some((selected_pos, _)) if selected_pos == *pos) {
                    set_draw_color(Color::from_rgb(200, 200, 200));
                    draw_pie(draw_x, draw_y, diameter, diameter, 0.0, 360.0);
                }
                set_draw_color(Color::Black);
                draw_arc(draw_x, draw_y, diameter, diameter, 0.0, 360.0);
            }
            set_draw_color(Color::Black);
            draw_rect(wid.x(), wid.y(), wid.width(), wid.height());
            pop_clip();
        });

        canvas
    }
    fn set_circles(&mut self, circles: &HashMap<Coord, Radius>) {
        self.circles.replace(circles.clone());
        self.redraw();
    }
    fn selected(&self) -> Option<(Coord, Radius)> {
        *self.selected.borrow()
    }
}

impl Deref for Canvas {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.widget
    }
}

impl DerefMut for Canvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.widget
    }
}

#[derive(Clone, Copy)]
enum Message {
    Undo,
    Redo,
    Add(Coord),
    AdjustOpened,
    RadiusChanged,
    ModelChanged,
}

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut wind = Window::default().with_label("CircleDraw");

    let (sender, receiver) = channel::<Message>();

    let mut model = Model::new(sender);

    let mut undo_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(
            WIDGET_PADDING + (CANVAS_WIDTH - WIDGET_WIDTH * 2 - WIDGET_PADDING) / 2,
            WIDGET_PADDING,
        )
        .with_label("Undo");
    undo_button.emit(sender, Message::Undo);

    let mut redo_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&undo_button, WIDGET_PADDING)
        .with_label("Redo");
    redo_button.emit(sender, Message::Redo);

    let mut canvas = Canvas::new(
        WIDGET_PADDING,
        undo_button.y() + undo_button.height() + WIDGET_PADDING,
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        sender,
    );

    wind.set_size(
        canvas.x() + canvas.width() + WIDGET_PADDING,
        canvas.y() + canvas.height() + WIDGET_PADDING,
    );
    wind.end();

    let mut adjust_window = Window::default().with_label("Adjust diameter");
    adjust_window.make_modal(true);

    let mut diameter_slider = HorNiceSlider::default()
        .with_pos(WIDGET_PADDING, WIDGET_HEIGHT / 2 + WIDGET_PADDING * 2)
        .with_size(WIDGET_WIDTH * 4, WIDGET_HEIGHT)
        .with_align(Align::Left);
    diameter_slider.set_align(Align::Top | Align::Left);
    diameter_slider.set_minimum(1.0);
    diameter_slider.set_maximum(50.0);
    diameter_slider.emit(sender, Message::RadiusChanged);

    adjust_window.set_size(
        diameter_slider.x() + diameter_slider.width() + WIDGET_PADDING,
        diameter_slider.y() + diameter_slider.height() + WIDGET_PADDING,
    );
    adjust_window.end();

    wind.show();
    sender.send(Message::ModelChanged);
    while app.wait() {
        match receiver.recv() {
            Some(Message::Undo) => model.undo(),
            Some(Message::Redo) => model.redo(),
            Some(Message::Add(pos)) => {
                model.save();
                model.set(pos, 20);
            }
            Some(Message::AdjustOpened) => {
                model.save();
                let (pos, radius) = canvas.selected().unwrap();
                diameter_slider.set_label(&format!("Adjust diameter of circle at {:?}.", pos));
                diameter_slider.set_value(f64::from(radius));
                adjust_window.show();
            }
            Some(Message::RadiusChanged) => model.set(
                canvas.selected().unwrap().0,
                diameter_slider.value() as Radius,
            ),
            Some(Message::ModelChanged) => {
                set_activated(&mut undo_button, model.can_undo());
                set_activated(&mut redo_button, model.can_redo());
                canvas.set_circles(model.circles());
            }
            None => {}
        }
    }
}

fn set_activated<T: WidgetExt>(widget: &mut T, is_activated: bool) {
    if is_activated {
        widget.activate();
    } else {
        widget.deactivate();
    }
}
