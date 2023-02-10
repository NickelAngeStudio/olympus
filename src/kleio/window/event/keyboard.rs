/// Enumeration of possible Keyboard events
pub enum KEventKeyboard {

    // Keyboard key down event. Provides unicode of key pressed.
    KeyDown(u32),

    // Keyboard key up event. Provides unicode of key released.
    KeyUp(u32),
}

impl std::fmt::Debug for KEventKeyboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyDown(arg0) => f.debug_tuple("KeyDown").field(arg0).finish(),
            Self::KeyUp(arg0) => f.debug_tuple("KeyUp").field(arg0).finish(),
        }
    }
}