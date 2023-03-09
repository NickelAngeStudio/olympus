use std::any::Any;
use crate::error::OlympusError;

use super::{event::KEvent, screen::KScreenList, KWindowProperty, cursor::KCursorMode, KFullscreenMode};

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
/// 
/// Those members have default implementation that needs to be overrided for desktop : 
/// * [KWindowManager::confine_cursor] 
/// * [KWindowManager::release_cursor] 
/// * [KWindowManager::set_cursor_mode] 
/// * [KWindowManager::set_cursor_position] 
/// * [KWindowManager::set_position] 
/// * [KWindowManager::set_size] 
/// * [KWindowManager::hide_cursor] 
/// * [KWindowManager::show_cursor] 
/// * [KWindowManager::restore] 
pub trait KWindowManager {
    /// Get the window provider managing this window.
    fn get_window_provider(&self) -> KWindowProvider;

    /// Pop a window event from the queue.
    fn poll_event(&mut self) -> KEvent;

    /// Get the count of events that need handling.
    fn get_event_count(&self) -> usize;

    /// Get windows properties.
    /// 
    /// The [KWindowManager] is responsible for updating this struct.
    fn get_window_property(&self) -> &KWindowProperty;

    /// Set the cursor position
    /// 
    /// Must be overridden for desktop implementation.
    fn set_cursor_position(&mut self, position : (i32, i32))  { todo!( )}

    /// Set the cursor mode for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    /// 
    /// Must be overridden for desktop implementation.
    fn set_cursor_mode(&mut self, mode : KCursorMode)  { todo!( )}

    /// Hide system default cursor.
    /// 
    /// Must be overridden for desktop implementation.
    fn hide_cursor(&mut self)  { todo!( )}

    /// Show system default cursor.
    /// 
    /// Must be overridden for desktop implementation.
    fn show_cursor(&mut self) { todo!( )}

    /// Confine cursor to window, preventing it from exiting boundaries.
    /// 
    /// Must be overridden for desktop implementation.
    fn confine_cursor(&mut self) { todo!( )}

    /// Release cursor from window, allowing it to exit boundaries.
    /// 
    /// Must be overridden for desktop implementation.
    fn release_cursor(&mut self)  { todo!( )}

    /// Restore the window, undoing any minimized, maximized and/or fullscreen status.
    /// 
    /// Must be overridden for desktop implementation.
    fn restore(&mut self)  { todo!( )}

    /// Set a new title for the window.
    fn set_title(&mut self, title : &str);

    /// Set a size for window.
    /// 
    /// Must be overridden for desktop implementation.
    fn set_size(&mut self, size : (u32, u32))  { todo!( )}

     /// Set a position of window.
     /// 
     /// Must be overridden for desktop implementation.
    fn set_position(&mut self, position : (i32, i32))  { todo!( )}

    /// Set the window as fullscreen according to [KFullscreenMode].
    fn set_fullscreen(&mut self, fs_mode : KFullscreenMode);

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
