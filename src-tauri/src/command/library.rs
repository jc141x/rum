use gilrs::Gilrs;
use serde::Serialize;
use std::{thread, time::Duration};

#[derive(Debug, Serialize)]
pub struct TauriRumError {
    message: String,
}

impl TauriRumError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<T: std::error::Error> From<T> for TauriRumError {
    fn from(error: T) -> TauriRumError {
        TauriRumError {
            message: format!("{}", error),
        }
    }
}

pub fn misc_init_bg_process() {
    static mut STARTED: bool = false;
    let window = Window::new();
    unsafe {
        if STARTED {
            return;
        }
    }
    thread::spawn(move || {
        let mut gilrs = Gilrs::new().unwrap();
        unsafe {
            STARTED = true;
        }
        loop {
            while let Some(ev) = gilrs.next_event() {
                if let gilrs::ev::EventType::ButtonPressed(..) = ev.event {
                    // change to emit event
                    println!("gamepad event: {:?}", ev.event);
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
}
