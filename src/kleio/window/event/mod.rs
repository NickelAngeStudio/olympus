use super::{KEventWindow, KEventKeyboard, KEventMouse, KEventController};

// Kleio window events
#[doc(hidden)]
pub mod window;

// Kleio keyboard events
#[doc(hidden)]
pub mod keyboard;

// Kleio mouse events
#[doc(hidden)]
pub mod mouse;

// Kleio controller events
#[doc(hidden)]
pub mod controller;

/// Union of possible events into an enumeration.
pub enum KEvent {

    /// Unknown Event
    Unknown,

    /// Window events
    Window(KEventWindow),

    /// Keyboard events
    Keyboard(KEventKeyboard),

    /// Mouse events
    Mouse(KEventMouse),

    /// Controller events
    Controller(KEventController),
}

impl std::fmt::Debug for KEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Window(arg0) => f.debug_tuple("Window").field(arg0).finish(),
            Self::Keyboard(arg0) => f.debug_tuple("Keyboard").field(arg0).finish(),
            Self::Mouse(arg0) => f.debug_tuple("Mouse").field(arg0).finish(),
            Self::Controller(arg0) => f.debug_tuple("Controller").field(arg0).finish(),
        }
    }
}
