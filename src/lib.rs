extern crate xcb;

mod widget;
mod window;
mod display;

pub use widget::Widget;
pub use window::{Event, Window};
pub use display::Display;

#[derive(Default)]
pub struct App {
    window: Window,
    widgets: Vec<Box<Widget>>,
}

impl App {
    pub fn run(&mut self) -> {
        self.window.map_and_flush();
        for event in window.events() {
            for widget in self.widgets {
                widget.handle(event);
            }
        }
    }

    pub fn close(&mut self) -> {
        self.should_close = true;
    }
}

