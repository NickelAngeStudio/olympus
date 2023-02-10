/// Enumeration of possible mouse events
pub enum KEventMouse {

    // Mouse move event. Provides new (x, y) position and old (x, y) position.
    Moved((i32, i32), (i32, i32)),

    // Mouse button down event. Provides button number (up to 255) and cursor position (x,y).
    ButtonDown(u8, (i32, i32)),

    // Mouse button up event. Provides button number (up to 255) and cursor position (x,y).
    ButtonUp(u8, (i32, i32)),

    // Mouse wheel event. Provide amount scrolled horizontally and vertically.
    Wheel(i32, i32),

}

impl std::fmt::Debug for KEventMouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Moved(arg0, arg1) => f.debug_tuple("Moved").field(arg0).field(arg1).finish(),
            Self::ButtonDown(arg0, arg1) => f.debug_tuple("ButtonDown").field(arg0).field(arg1).finish(),
            Self::ButtonUp(arg0, arg1) => f.debug_tuple("ButtonUp").field(arg0).field(arg1).finish(),
            Self::Wheel(arg0, arg1) => f.debug_tuple("Wheel").field(arg0).field(arg1).finish(),
        }
    }
}