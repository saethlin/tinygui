extern crate tinygui;

use tinygui::{Event, Window};

fn main() {
    let window = Window::default();
    window.map_and_flush();

    window.events().for_each(|event| {
        if let Event::KeyPress(key) = event {
            if key == 9 {
                std::process::exit(0);
            }
        }
        println!("{:?}", event);
    })
}
