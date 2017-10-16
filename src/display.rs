use widget::Widget;
use window::Event;

#[derive(Debug)]
pub struct Display {}

impl Widget for Display {
    fn draw(&self) {}
    fn handle(&self, event: &Event) {}
}
