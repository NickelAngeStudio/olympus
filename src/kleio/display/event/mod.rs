/// # Re-export for Public API
#[doc(inline)]
pub use mouse::KEventMouse as KEventMouse;
pub use window::KEventWindow as KEventWindow;
pub use controller::KEventController as KEventController;
pub use keyboard::KEventKeyboard as KEventKeyboard;
pub use dispatcher::KEventDispatcherError as KEventDispatcherError;
pub use dispatcher::KEventDispatcher as KEventDispatcher;
pub use dispatcher::KEventReceiver as KEventReceiver;

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

// Kleio events dispatcher and receiver
#[doc(hidden)]
pub mod dispatcher;

/// Union of possible events into an enumeration.
#[derive(Copy, Clone)]
pub enum KEvent {

    /// No event.
    None,

    /// Window events
    Window(KEventWindow),

    /// Keyboard events
    Keyboard(KEventKeyboard),

    /// Mouse events
    Mouse(KEventMouse),

    /// Controller events
    Controller(KEventController),

    /// Unknown/Unhandled by Kleio event
    Unknown,
}

impl std::fmt::Debug for KEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Window(arg0) => f.debug_tuple("Window").field(arg0).finish(),
            Self::Keyboard(arg0) => f.debug_tuple("Keyboard").field(arg0).finish(),
            Self::Mouse(arg0) => f.debug_tuple("Mouse").field(arg0).finish(),
            Self::Controller(arg0) => f.debug_tuple("Controller").field(arg0).finish(),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
