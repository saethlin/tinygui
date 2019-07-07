extern crate xcb;

mod display;
mod widget;
mod window;

pub use display::Display;
pub use widget::Widget;
pub use window::{Event, Window};

#[derive(Default)]
pub struct App {
    window: Window,
    widgets: Vec<Box<Widget>>,
    should_close: bool,
}

impl App {
    pub fn run(&mut self) {
        self.window.map_and_flush();
        for event in self.window.events() {
            for widget in &self.widgets {
                widget.handle(&event);
            }

            if self.should_close {
                break;
            }
        }
    }

    pub fn close(&mut self) {
        self.should_close = true;
    }
}
