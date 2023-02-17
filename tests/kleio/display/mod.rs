use std::process::exit;

use olympus::kleio::display::event::KEventReceiver;

// Contains tests for screen module
#[cfg(test)]
pub mod screen;


// Contains tests for windows KEvent
#[cfg(test)]
pub mod event;

// Contains tests for KEventDispatcher
#[cfg(test)]
pub mod dispatcher;

// Contains tests for X11 KWindow Linux Display Server
#[cfg(all(not(target_family = "wasm"), target_os = "linux"))]
pub mod x11;

/// Receiver used to tell if test is successful or not since all tests are runned manually.
/// 
/// Pressing Y is success.
/// Pressing N is failed.
struct KEventReceiverControl {

}

impl KEventReceiver for KEventReceiverControl {
    fn handle_event(&mut self, event : &olympus::kleio::display::event::KEvent) -> bool {
        match event {
            olympus::kleio::display::event::KEvent::None => todo!(),
            olympus::kleio::display::event::KEvent::Window(_) => todo!(),
            olympus::kleio::display::event::KEvent::Keyboard(event) => match event {
                olympus::kleio::display::event::KEventKeyboard::KeyDown(keycode) => match keycode {
                    // 121 is y. Exit with success.
                    121 => { exit(0); },

                    // 110 is n. Exit with error.
                    110 => { exit(1); },

                    // Unhandled
                    _ => false,
                },
                // Unhandled
                _ => false,
            },
            olympus::kleio::display::event::KEvent::Mouse(_) => todo!(),
            olympus::kleio::display::event::KEvent::Controller(_) => todo!(),
            olympus::kleio::display::event::KEvent::Unknown => todo!(),
        }
    }

    fn is_enabled(&self) -> bool {
        true
    }
}