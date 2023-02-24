/// Enumeration of possible events for a window
#[derive(Debug, Copy, Clone)]
pub enum KEventWindow {

    /// Happens when KWindow is shown.
    Shown(),

    /// Happens when KWindow is hidden.
    Hidden(),

    /// Happens when KWindow is exposed.
    Exposed(),

    /// Happens when KWindow is moved. Provides (x,y) of new position.
    Moved((i32, i32)),

    /// Happens when KWindow is moved and resized. Provides (x,y) of new position and (height, width) of new size.
    MovedResized((i32, i32), (u32, u32)),

    /// Happens when KWindow is Resized. Provides (height, width) of new size.
    Resized((u32, u32)),

    /// Happens when KWindow is minimized.
    Minimized(),

    /// Happens when KWindow is maximized.
    Maximized(),

    /// Happens when KWindow is restored.
    Restored(),

    /// Happens when cursor enter KWindow.
    CursorEnter(),

    /// Happens when cursor leave KWindow.
    CursorLeave(),

    /// Happens when KWindow gain focus.
    Focus(),

    /// Happens when KWindow lose focus.
    Blur(),

    /// Happens when KWindow closes.
    Close(),
}