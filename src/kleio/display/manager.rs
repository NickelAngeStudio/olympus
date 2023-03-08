use std::any::Any;
use crate::error::OlympusError;

use super::{event::KEvent, KWindowFullscreenMode, screen::KScreenList};

/// Enumeration of [Display server](https://en.wikipedia.org/wiki/Windowing_system#Display_server)
/// and/or [Window manager](https://en.wikipedia.org/wiki/Window_manager) providers.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KWindowProvider {

    /// [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) display server.
    Wayland,

    /// [X Window System](https://en.wikipedia.org/wiki/X_Window_System) display server.
    X11,

    /// Microsoft Windows [Desktop Window Manager](https://en.wikipedia.org/wiki/Desktop_Window_Manager) compositing window manager.
    Windows,

    /// Apple MacOS [Quartz](https://en.wikipedia.org/wiki/Quartz_Compositor) compositor.
    MacOs,

    /// Android [SurfaceFlinger](https://en.wikipedia.org/wiki/Windowing_system#SurfaceFlinger) compositor.
    Android,

    /// Apple IOS [Quartz](https://en.wikipedia.org/wiki/Quartz_Compositor) compositor.
    IOS,

    /// [Web assembly](https://en.wikipedia.org/wiki/WebAssembly) browser compositor.
    WASM,
}

/// Abstraction of a [Display server](https://en.wikipedia.org/wiki/Windowing_system#Display_server)
/// and/or [Window manager](https://en.wikipedia.org/wiki/Window_manager) used to create and manage window.
pub trait KWindowManager {
    /// Get the window provider managing this window.
    fn get_window_provider(&self) -> KWindowProvider;

    /// Static function to generate a screen list.
    /// 
    /// Returns Ok([KScreenList]) if successful.
    /// 
    /// Error(s)
    /// Returns Err[KWindowError::ScreenDetailError] if it failed to fetch list.
    fn get_screen_list() -> Result<KScreenList, OlympusError>;

    /// Pop a window event from the queue.
    fn poll_event(&mut self) -> KEvent;

    /// Get the count of events that need handling.
    fn get_event_count(&self) -> usize;

    /// Set the cursor position
    fn set_cursor_position(&mut self, position : (i32, i32));

    /// Hide system default cursor.
    fn hide_cursor(&mut self);

    /// Show system default cursor.
    fn show_cursor(&mut self) ;

    /// Confine cursor to window, preventing it from exiting boundaries.
    fn confine_cursor(&mut self);

    /// Release cursor from window, allowing it to exit boundaries.
    fn release_cursor(&mut self);

    /// Restore the window, undoing any minimized, maximized and/or fullscreen status.
    fn restore(&mut self);

    /// Set a new title for the window.
    fn set_title(&mut self);

    /// Set a size for window.
    fn set_size(&mut self, size : (u32, u32));

     /// Set a position of window.
    fn set_position(&mut self, position : (i32, i32));

    /// Set the window as fullscreen according to [KWindowFullscreenMode].
    fn set_fullscreen(&mut self, fs_mode : KWindowFullscreenMode);

    /// Perform sync with the display server / window manager.
    fn sync(&self);

    /// Get self as Any, use for downcast. 
    /// 
    /// Implementation only need to return self.
    fn as_any(&self) -> &dyn Any;

    /// Get self as mut Any, use for downcast. 
    /// 
    /// Implementation only need to return mut self.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
