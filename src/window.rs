use std::fmt::Debug;
use widget::Widget;
use xcb;

pub struct Window {
    connection: xcb::Connection,
    id: u32,
    widgets: Vec<Box<Widget>>,
}

impl Default for Window {
    fn default() -> Self {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let window_id = conn.generate_id();
        {
            let setup = conn.get_setup();
            let screen = setup.roots().nth(screen_num as usize).unwrap();

            let values = [
                (xcb::CW_BACK_PIXEL, screen.white_pixel()),
                (
                    xcb::CW_EVENT_MASK,
xcb::EVENT_MASK_BUTTON_1_MOTION 	|
xcb::EVENT_MASK_BUTTON_2_MOTION 	|
xcb::EVENT_MASK_BUTTON_3_MOTION 	|
xcb::EVENT_MASK_BUTTON_4_MOTION 	|
xcb::EVENT_MASK_BUTTON_5_MOTION 	|
xcb::EVENT_MASK_BUTTON_MOTION 	|
xcb::EVENT_MASK_BUTTON_PRESS 	|
xcb::EVENT_MASK_BUTTON_RELEASE 	|
xcb::EVENT_MASK_COLOR_MAP_CHANGE |	
xcb::EVENT_MASK_ENTER_WINDOW 	|
xcb::EVENT_MASK_EXPOSURE 	|
xcb::EVENT_MASK_FOCUS_CHANGE |	
xcb::EVENT_MASK_KEYMAP_STATE |	
xcb::EVENT_MASK_KEY_PRESS 	|
xcb::EVENT_MASK_KEY_RELEASE |	
xcb::EVENT_MASK_LEAVE_WINDOW |	
xcb::EVENT_MASK_NO_EVENT 	|
xcb::EVENT_MASK_OWNER_GRAB_BUTTON |	
xcb::EVENT_MASK_POINTER_MOTION 	|
xcb::EVENT_MASK_POINTER_MOTION_HINT |	
xcb::EVENT_MASK_PROPERTY_CHANGE 	|
xcb::EVENT_MASK_RESIZE_REDIRECT 	|
xcb::EVENT_MASK_STRUCTURE_NOTIFY 	|
xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY |	
xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT 	|
xcb::EVENT_MASK_VISIBILITY_CHANGE 	     ),
            ];

            xcb::create_window(
                &conn,
                xcb::COPY_FROM_PARENT as u8,
                window_id,
                screen.root(),
                0,
                0,
                800,
                500,
                10,
                xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                screen.root_visual(),
                &values,
            );
        }

        Window {
            connection: conn,
            id: window_id,
            widgets: Vec::new(),
        }
    }
}

impl Window {
    pub fn map_and_flush(&self) {
        xcb::map_window(&self.connection, self.id);
        self.connection.flush();
    }

    pub fn events(&self) -> EventIterator {
        EventIterator { window: self }
    }
}

pub struct EventIterator<'a> {
    window: &'a Window,
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        use self::Event::*;
        let raw_event = self.window.connection.wait_for_event();
        match raw_event {
            None => None,
            Some(event_type) => {
                match event_type.response_type() {
                    2 => {
                        let key_press_event: &xcb::KeyPressEvent =
                            unsafe { xcb::cast_event(&event_type) };
                        Some(KeyPress(key_press_event.detail()))
                    }
                    3 => {
                        let key_press_event: &xcb::KeyPressEvent =
                            unsafe { xcb::cast_event(&event_type) };
                        Some(KeyRelease(key_press_event.detail()))
                    }
                    4 => {
                        let button_press_event: &xcb::ButtonPressEvent =
                            unsafe { xcb::cast_event(&event_type) };
                        let location = Point {
                            x: button_press_event.event_x(),
                            y: button_press_event.event_y(),
                        };
                        Some(ButtonPress(button_press_event.detail(), location))
                    }
                    5 => {
                        let button_press_event: &xcb::ButtonReleaseEvent =
                            unsafe { xcb::cast_event(&event_type) };
                        let location = Point {
                            x: button_press_event.event_x(),
                            y: button_press_event.event_y(),
                        };
                        Some(ButtonRelease(button_press_event.detail(), location))
                    }
                    6 => {                         let motion_notify_event: &xcb::MotionNotifyEvent =
                            unsafe { xcb::cast_event(&event_type) };
                        let location = Point {
                            x: motion_notify_event.event_x(),
                            y: motion_notify_event.event_y(),
                        };
                                                Some(MotionNotify(location))

                    }
                    7 => Some(EnterNotify),
                    8 => Some(LeaveNotify),
                    9 => Some(FocusIn),
                    10 => Some(FocusOut),
                    11 => Some(KeymapNotify),
                    12 => Some(Expose),
                    _ => Some(UnimplementedEvent),
                    //_ => unreachable!("Recieved invalid request from the server"),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Point<T: Debug> {
    x: T,
    y: T,
}

#[derive(Debug)]
pub enum Event {
    KeyPress(u8),
    KeyRelease(u8),
    ButtonPress(u8, Point<i16>),
    ButtonRelease(u8, Point<i16>),
    MotionNotify(Point<i16>),
    EnterNotify,
    LeaveNotify,
    FocusIn,
    FocusOut,
    KeymapNotify,
    Expose,
    GraphicsExposure,
    NoExposure,
    VisibilityNotify,
    CreateNotify,
    DestroyNotify,
    UnmapNotify,
    MapNotify,
    MapRequest,
    ReparentNotify,
    ConfigureRequest,
    GravityNotify,
    ResizeRequest,
    CirculateNotify,
    CirculateRequest,
    PropertyNotify,
    SelectionClear,
    SelectionRequest,
    SelectionNotify,
    ColormapNotify,
    ClientMessage,
    MappingNotify,
    UnimplementedEvent,
}
