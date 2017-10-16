use window::Event;

pub trait Widget {
    fn draw(&self);
    fn handle(&self, &Event);
}
