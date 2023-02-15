use super::event::{ KEvent, KEventDispatcher};

#[allow(unused_imports)]
use super::event::{ KEventMouse, KEventReceiver };

/// Minimum [KWindow] width allowed.
pub const KWINDOW_MIN_WIDTH : usize = 1;

/// Minimum [KWindow] height allowed.
pub const KWINDOW_MIN_HEIGHT : usize = 1;

/// Maximum [KWindow] width allowed.
pub const KWINDOW_MAX_WIDTH : usize = 65535;

/// Maximum [KWindow] height allowed.
pub const KWINDOW_MAX_HEIGHT : usize = 65535;


/// Enumeration of possible [KWindow] errors.
pub enum KWindowError {

    /// Happens when a window manager is not supported.
    NotSupported,

    /// Happens when no display server is found.
    NoDisplayServer,

    /// Happens when trying to resize a [KWindow] outside of allowed boundaries.
    WindowSizeError,


}


impl std::fmt::Debug for KWindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "KWindowError"),
        }
    }
}

/// Enumeration of possible [KWindow] motion mode.
pub enum KWindowMotionMode {
    /// [KEventMouse] events will give the (x,y) location of the cursor on the window. 
    /// 
    /// Usually used for user interfaces.
    Location,

    /// [KEventMouse] events will give the (x,y) acceleration of the cursor instead of the position.
    /// 
    /// Usually used for 3d camera and direct mouse inputs.
    Acceleration,
}

/// Create and manage a window frame for display.
/// 
/// [KWindow] broadcasts [KEvent] to multiple [KEventReceiver] via [KWindow::dispatch_events()].
/// 
/// TODO: More doc about OS, dispatch, and Examples
pub struct KWindow {

    /// X11 Motion mode,
    motion_mode : KWindowMotionMode,

    /// X11 Window center,
    center : (i32, i32),

    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    /// Linux display server (Linux only).
    display_server : Box<dyn super::linux::KLinuxDisplayServer>,

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
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize, fullscreen : bool) -> Result<KWindow, KWindowError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]

        // Make sure dimension are within boundaries.
        if KWindow::is_size_within_boundaries(width, height) {
            // Default motion mode
            let motion_mode = KWindowMotionMode::Location;

            // Default center position
            let center = ((width as i32 / 2), (height as i32 / 2));

            // Linux implementation
            #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
            {
                match super::linux::get_linux_display_server(pos_x, pos_y, width, height) {
                    Ok(display_server) => {
                        let kwindow = KWindow { motion_mode, center, display_server };
                        kwindow.set_fullscreen(fullscreen);
                        Ok(kwindow)
                    },
                    Err(err) => Err(err),
                }
            }

        } else {
            Err(KWindowError::WindowSizeError)
        }

    }

    /// Create a new [KWindow] for mobile devices.
    /// 
    /// Return New [`KWindow`] created.
    //#[cfg(any(doc, all(not(target_family = "wasm"), target_os = "linux")))]
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "android", target_os = "ios"))))]
    pub fn new() -> Result<KWindow, KWindowError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "android", target_os = "ios"))))]
        todo!()
    }


    /// Dispatch [KEvent] to [KEventReceiver] using a [KEventDispatcher].
    /// 
    /// Returns a pair of (# event handled, # event unhandled).
    /// 
    /// # Example(s)
    /// Dispatching at each loop call in Main loop
    /// ```
    /// // Create a KWindow
    /// let mut w = KWindow::new(100,100,100,100,true);
    /// 
    /// // Create a dispatcher that doesn't log unhandled events.
    /// let mut ked = KEventDispatcher::new(false);
    /// 
    /// ... add receivers via ked.add_receiver() ...
    /// 
    /// loop {
    ///     // Dispatch events
    ///     w.dispatch_events(&mut ked);
    /// }
    /// ```
    pub fn dispatch_events(&mut self, dispatcher : &mut KEventDispatcher) -> (u32, u32) {
        todo!()

        /*
        let handled = 0;
        let unhandled = 0;

        // First get the event count to poll. This is important to prevent bloking.
        let event_count = self.window_manager.get_event_count();

        // Count of unknown events.
        let mut unknown_count:usize = 0;

        for _ in 0..event_count {
            // Fetch event
            let event = self.window_manager.poll_event();

            match event {
                // Unknown event are ignored and deduced from event_count
                KEvent::Unknown => {
                    unknown_count = unknown_count + 1;
                },
                _ => {
                    // Iterate enabled receivers from newest to oldest
                    for receiver in self.receivers.iter().rev().filter(|x| x.borrow().is_enabled() ) {
                        
                        let mut receiver = receiver.borrow_mut();
                        if receiver.receive(&event) {
                            break;  // Break loop since event was handled
                        }
                    }
                },
            }
         */
    }

    /// Binding cursor prevent cursor from exiting windows boundaries when focused.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn bind_cursor(&mut self){
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Get the display server provider identification.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    pub fn get_display_server_provider(&self) -> super::linux::LinuxDisplayServerProvider{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]
        todo!()
    }

    /// Get the display server connection.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    pub fn get_display_server_connection(&self) -> *const super::linux::Display{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]
        todo!()
    }
    
    /// Get the display server window handle.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    pub fn get_display_server_window(&self) -> *const super::linux::Window{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]
        todo!()
    }

    /// Get the count of events that need handling.
    pub fn get_event_count(&self) -> usize {
        todo!()
    }

    /// Get the motion mode for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn get_motion_mode(&self) -> KWindowMotionMode{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

     /// Returns position (x,y) of the [KWindow].
     #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
     pub fn get_position(&self) -> (isize, isize) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
     }

     /// Returns dimension (width, height) of the [KWindow].
     #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn get_size(&self) -> (usize, usize) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Returns the [KWindow] title. 
    pub fn get_title(&self) -> &str {
        todo!()
    }

    /// Hide the default operating system cursor.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn hide_cursor(&self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Get if the cursor is binded to the window, preventing it from going further than window boundaries.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_cursor_binded(&self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Get if the default operating system cursor is hidden.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_cursor_hidden(&self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Returns if the [KWindow] is fullscreen or not.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_fullscreen(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }
    

    /// Returns if the [KWindow] is maximized or not.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_maximized(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Returns if the [KWindow] is minimized or not.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_minimized(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Pop an event from the [KWindow] event queue.
    /// 
    /// Returns any [KEvent] except [KEvent::None] if any event in queue.
    /// 
    /// Returns [KEvent::None] if no event in queue.
    /// 
    /// # Example(s)
    /// Polling all event of a [KWindow] :
    /// ```no_run
    /// loop {
    ///     let event = kwindow.poll_event();
    /// 
    ///     match event {
    ///         // Exit loop since no event in queue.
    ///         KEvent::None => break,
    ///         _ => todo!(),
    ///     }
    /// }
    /// ```
    pub fn poll_event(&mut self) -> KEvent {
        todo!()
    }

    /// Set a new title for the [KWindow].
    pub fn set_title(&self, title : &str) {
        todo!()
    }

    /// Set position of [KWindow] according to position (x,y).
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_position(&self, position : (isize, isize)){
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set dimension of [KWindow] according to size (width, height).
    /// 
    /// Returns Ok(0) if successful.
    /// 
    /// # Error(s)
    /// Returns [KWindowError::WindowSizeError] if width and/or height not within allowed boundaries.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_size(&self, dimension : (usize, usize)) -> Result<u8, KWindowError>{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the [KWindow] as fullscreen according to parameters.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_fullscreen(&self, fullscreen : bool) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the [KWindow] as minimized according to parameters.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_minimized(&self, minimized : bool) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the [KWindow] as maximized according to parameters.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_maximized(&self, maximized : bool) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn restore(&self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Show the default operating system cursor.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn show_cursor(&self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the cursor position with a pair (x,y).
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_cursor_position(&self, position : (i32, i32)){
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the motion mode for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_motion_mode(&self, mode : KWindowMotionMode) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Unbinding cursor let the cursor exits window boundaries when focused.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn unbind_cursor(&mut self){
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

}

/// Private KWindow members
#[doc(hidden)]
impl KWindow {
    /// Return True if width and size are between boundaries.
    fn is_size_within_boundaries(width:usize, height:usize) -> bool {

        if width >= KWINDOW_MIN_WIDTH && width <= KWINDOW_MAX_WIDTH && height >= KWINDOW_MIN_HEIGHT && height <= KWINDOW_MAX_HEIGHT {
            // Withing boundaries
            true
        } else {
            // Boundaries overflow
            false
        }

    }
}