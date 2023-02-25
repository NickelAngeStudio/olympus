/// Enumeration of possible events for a window
#[derive(Copy, Clone)]
pub enum KEventWindow {

    /// Happens when KWindow is shown.
    Shown(),

    /// Happens when KWindow is hidden.
    Hidden(),

    /// Happens when KWindow is exposed/damaged, meaning part of drawing is lost and need to be redraw.
    /// Provides position (x, y) and size (width, height) of region exposed. 
    Exposed((i32, i32), (u32, u32)),

    /// Happens when KWindow is moved. Provides (x,y) of new position.
    Moved((i32, i32)),

    /// Happens when KWindow is moved and resized. Provides (x,y) of new position and (height, width) of new size.
    MovedResized((i32, i32), (u32, u32)),

    /// Happens when KWindow is Resized. Provides (height, width) of new size.
    Resized((u32, u32)),

    /// Happens when KWindow is minimized.
    /// 
    /// # Known issue(s)
    /// * `(Linux only)` Won't trigger if window is maximized.
    Minimized(),

    /// Happens when KWindow is maximized.
    Maximized(),

    /// Happens when KWindow is set fullscreen.
    Fullscreen(),

    /// Happens when KWindow is restored from minimized, maximized or fullscreen.
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

impl std::fmt::Debug for KEventWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shown() => f.debug_tuple("Shown").finish(),
            Self::Hidden() => f.debug_tuple("Hidden").finish(),
            Self::Exposed(arg0, arg1) => f.debug_tuple("Exposed").field(arg0).field(arg1).finish(),
            Self::Moved(arg0) => f.debug_tuple("Moved").field(arg0).finish(),
            Self::MovedResized(arg0, arg1) => f.debug_tuple("MovedResized").field(arg0).field(arg1).finish(),
            Self::Resized(arg0) => f.debug_tuple("Resized").field(arg0).finish(),
            Self::Minimized() => f.debug_tuple("Minimized").finish(),
            Self::Maximized() => f.debug_tuple("Maximized").finish(),
            Self::Fullscreen() => f.debug_tuple("Fullscreen").finish(),
            Self::Restored() => f.debug_tuple("Restored").finish(),
            Self::CursorEnter() => f.debug_tuple("CursorEnter").finish(),
            Self::CursorLeave() => f.debug_tuple("CursorLeave").finish(),
            Self::Focus() => f.debug_tuple("Focus").finish(),
            Self::Blur() => f.debug_tuple("Blur").finish(),
            Self::Close() => f.debug_tuple("Close").finish(),
        }
    }
}