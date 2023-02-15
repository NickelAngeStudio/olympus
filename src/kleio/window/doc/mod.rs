use std::rc::Rc;
use std::cell::RefCell;
use super::{KEvent, KWindowError, KWindowMotionMode, KEventDispatcher, KEventReceiver, LinuxDisplayServerProvider};
use super::linux::{ Window, Display };

use super::KEventDispatcherError;

/// Local macro that tell user that this is documentation only
#[doc(hidden)]
macro_rules! documentation_only {
    () => {
        panic!("This is unified documentation only. Implementations are in another modules."),
    }
}

// This module is used exclusively for documentation and unification of all KWindow implementation.

/// Create and manage a window frame for display.
/// 
/// [KWindow] broadcasts [KEvent] to multiple [KEventReceiver] via [KWindow::dispatch_events()].
/// 
/// TODO: More doc about OS, dispatch, and Examples
pub struct KWindow {
    

}

impl KWindow {
    /// Create a new [KWindow] using position and size with option to set fullscreen or not.
    /// 
    /// Return New [`KWindow`].
    /// 
    /// 
    /// # Error(s)
    /// Returns [KWindowError::NoDisplayServer] if no display server found on Linux.
    /// 
    /// Returns [KWindowError::WindowSizeError] if width and/or height aren't within allowed boundaries.
    /// 
    /// # Note(s)
    /// On Linux distribution, this will try to create a [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) window first
    /// then a [x11](https://en.wikipedia.org/wiki/X_Window_System) window if not compatible with Wayland.
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize, fullscreen : bool) -> Result<KWindow, KWindowError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }

    /// Create a new [KWindow] for mobile devices.
    /// 
    /// Return New [`KWindow`] created.
    //#[cfg(any(doc, all(not(target_family = "wasm"), target_os = "linux")))]
    pub fn new() -> Result<KWindow, KWindowError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "android", target_os = "ios"))))]
        documentation_only()
    }


    /// Dispatch [KEvent] to [KEventReceiver] using a [KEventDispatcher].
    /// 
    /// This function should be called at the beginning of each main loop.
    /// 
    /// Returns the count of [KEvent] dispatched.
    /// 
    /// # Example(s)
    /// Dispatching at each loop call in Main loop
    /// ```
    /// // Create a KWindow
    /// let mut w = KWindow::new(100,100,100,100,true);
    /// 
    /// ... add receivers via w.get_event_dispatcher().add_receiver() ...
    /// 
    /// loop {
    ///     match w.dispatch_events(){
    ///         Ok(event_count) => println!("{} events dispatched!", event_count),
    ///         Err(_) => println!("No receivers added for dispatch!"),
    ///     }
    /// }
    /// ```
    /// 
    /// # Error(s)
    /// Returns `Err(`[KEventDispatcherError::DispatchNoReceiver]`)` if no receiver added to handle events.
    pub fn dispatch_events(&mut self, dispatcher : KEventDispatcher) -> Result<usize, KEventDispatcher> {
        documentation_only()
    }

    /// Binding cursor prevent cursor from exiting windows boundaries when focused.
    pub fn bind_cursor(&mut self){
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }

    /// Get the display server provider identification.
    fn get_display_server_provider(&self) -> LinuxDisplayServerProvider{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]
        documentation_only()
    }

    /// Get the display server connection.
    fn get_display_server_connection(&self) -> *const Display{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]
        documentation_only()
    }
    
    /// Get the display server window handle.
    fn get_display_server_window(&self) -> *const Window{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]
        documentation_only()
    }

    /// Get the motion mode for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    pub fn get_motion_mode(&self) -> KWindowMotionMode{
        documentation_only()
    }

     /// Returns position (x,y) of the [KWindow].
     pub fn get_position(&self) -> (isize, isize) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
     }

     /// Returns dimension (width, height) of the [KWindow].
    pub fn get_size(&self) -> (usize, usize) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }

    /// Returns the [KWindow] title. 
    pub fn get_title(&self) -> &str {
        documentation_only()
    }

    /// Hide the default operating system cursor.
    pub fn hide_cursor(&self) {
        documentation_only()
    }

     /// Get if the cursor is binded to the window, preventing it from going further than window boundaries.
     pub fn is_cursor_binded(&self) {
        documentation_only()
    }

    /// Get if the default operating system cursor is hidden.
    pub fn is_cursor_hidden(&self) {
        documentation_only()
    }

    /// Returns if the [KWindow] is fullscreen or not.
    pub fn is_fullscreen(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }
    

    /// Returns if the [KWindow] is maximized or not.
    pub fn is_maximized(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }

     /// Returns if the [KWindow] is minimized or not.
     pub fn is_minimized(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
     }

    /// Set a new title for the [KWindow].
    pub fn set_title(&self, title : &str) {
        documentation_only()
    }

    /// Set position of [KWindow] according to position (x,y).
    pub fn set_position(&self, position : (isize, isize)){
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }

    /// Set dimension of [KWindow] according to size (width, height).
    /// 
    /// Returns Ok(0) if successful.
    /// 
    /// # Error(s)
    /// Returns [KWindowError::WindowSizeError] if width and/or height not within allowed boundaries.
    pub fn set_size(&self, dimension : (usize, usize)) -> Result<u8, KWindowError>{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }

    /// Set the [KWindow] as fullscreen according to parameters.
    pub fn set_fullscreen(&self, fullscreen : bool) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }

    /// Set the [KWindow] as minimized according to parameters.
    pub fn set_minimized(&self, minimized : bool) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }

    /// Set the [KWindow] as maximized according to parameters.
    pub fn set_maximized(&self, maximized : bool) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }

    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    pub fn restore(&self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }

    /// Show the default operating system cursor.
    pub fn show_cursor(&self) {
        documentation_only()
    }

    /// Set the cursor position with a pair (x,y).
    pub fn set_cursor_position(&self, position : (i32, i32)){
        documentation_only()
    }

    /// Set the motion mode for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    pub fn set_motion_mode(&self, mode : KWindowMotionMode) {
        documentation_only()
    }

    /// Unbinding cursor let the cursor exits window boundaries when focused.
    pub fn unbind_cursor(&mut self){
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        documentation_only()
    }

}