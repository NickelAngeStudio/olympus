/// Enumeration of possible events for a window
pub enum KEventWindow {

    /// Happens when KWindow is shown.
    Shown(),

    /// Happens when KWindow is hidden.
    Hidden(),

    /// Happens when KWindow is exposed.
    Exposed(),

    /// Happens when KWindow is moved. Provides (x,y) of new position.
    Moved((isize, isize)),

    /// Happens when KWindow is Resized. Provides (height, width) of new size.
    Resized((usize, usize)),

    /// Happens when KWindow size changed without user input. Provides (height, width) of new size.
    SizeChanged((usize, usize)),

    /// Happens when KWindow is minimized.
    Minimized(),

    /// Happens when KWindow is maximized.
    Maximized(),

    /// Happens when KWindow is restored.
    Restored(),

    /// Happens when mouse enter KWindow.
    MouseEnter(),

    /// Happens when mouse leave KWindow.
    MouseLeave(),

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
            Self::Exposed() => f.debug_tuple("Exposed").finish(),
            Self::Moved(arg0) => f.debug_tuple("Moved").field(arg0).finish(),
            Self::Resized(arg0) => f.debug_tuple("Resized").field(arg0).finish(),
            Self::SizeChanged(arg0) => f.debug_tuple("SizeChanged").field(arg0).finish(),
            Self::Minimized() => f.debug_tuple("Minimized").finish(),
            Self::Maximized() => f.debug_tuple("Maximized").finish(),
            Self::Restored() => f.debug_tuple("Restored").finish(),
            Self::MouseEnter() => f.debug_tuple("MouseEnter").finish(),
            Self::MouseLeave() => f.debug_tuple("MouseLeave").finish(),
            Self::Focus() => f.debug_tuple("Focus").finish(),
            Self::Blur() => f.debug_tuple("Blur").finish(),
            Self::Close() => f.debug_tuple("Close").finish(),
        }
    }
}