use super::{event::{ KEvent, KEventDispatcher}, screen::KScreenList};
use super::{ KCursorMode, KCursor };

#[allow(unused_imports)]
use super::event::{ KEventMouse, KEventReceiver };

/// Minimum [KWindow] width allowed.
pub const KWINDOW_MIN_WIDTH : u32 = 1;

/// Minimum [KWindow] height allowed.
pub const KWINDOW_MIN_HEIGHT : u32 = 1;

/// Maximum [KWindow] width allowed.
pub const KWINDOW_MAX_WIDTH : u32 = 65535;

/// Maximum [KWindow] height allowed.
pub const KWINDOW_MAX_HEIGHT : u32 = 65535;


/// Enumeration of possible [KWindow] errors.
#[derive(Debug)]
pub enum KWindowError {

    /// Happens when a window manager is not supported.
    NotSupported,

    /// Happens when no display server is found.
    NoDisplayServer,

    /// Happens when trying to resize a [KWindow] outside of allowed boundaries.
    WindowSizeError,

    /// Happens when trying get hardware screen details failed.
    ScreenDetailError,


}

/// Create and manage a window frame for display.
/// 
/// [KWindow] broadcasts [KEvent] to multiple [KEventReceiver] via [KWindow::dispatch_events()].
/// 
/// TODO: More doc about OS, dispatch, and Examples
pub struct KWindow {

    /// Hardware screen list
    pub(super) screen_list : KScreenList,

    #[cfg(any(doc, any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    /// Cursor mode and properties
    pub(super) cursor : KCursor,

    /// Size of window as pair of u32 (width, height).
    pub(super) size : (u32, u32),

    /// Window center,
    pub(super) center : (i32, i32),

    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    /// Linux display server details (Linux only).
    pub(super) display_server : super::linux::server::KLinuxDisplayServer,
}

impl KWindow {
    /// Create a new sized [KWindow] in the middle of the main default screen.
    /// 
    /// Display server provider can be set to preferred display server or default. This will try to create a 
    /// [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) window first then a [x11](https://en.wikipedia.org/wiki/X_Window_System) window if not compatible with Wayland.
    /// 
    /// Return New [`KWindow`].
    /// 
    /// 
    /// # Error(s)
    /// Returns [KWindowError::NoDisplayServer] if no display server found on Linux.
    /// 
    /// Returns [KWindowError::WindowSizeError] if width and/or height aren't within allowed boundaries.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    pub fn new(width:u32, height:u32, provider : super::linux::server::KLinuxDisplayServerProvider) -> Result<KWindow, KWindowError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]

        // Make sure dimension are within boundaries.
        if KWindow::is_size_within_boundaries(width, height) {
            KWindow::__new(width, height, provider)     // Private platform inline implementation
        } else {
            Err(KWindowError::WindowSizeError)
        }

    }

    /// Create a new sized [KWindow] in the middle of the main default screen.
    /// 
    /// Return New [`KWindow`].
    /// 
    /// 
    /// # Error(s)
    /// Returns [KWindowError::WindowSizeError] if width and/or height aren't within allowed boundaries.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "windows", target_os = "macos"))))]
    pub fn new(width:u32, height:u32) -> Result<KWindow, KWindowError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "windows", target_os = "macos"))))]

        // Make sure dimension are within boundaries.
        if KWindow::is_size_within_boundaries(width, height) {
            KWindow::__new(width, height)     // Private platform inline implementation
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
        
        KWindow::__new()     // Private platform inline implementation
    }


    /// Dispatch [KEvent] to [KEventReceiver] using a [KEventDispatcher].
    /// 
    /// # Note(s)
    /// After dispatching events, [KWindow::sync_events()] will be called automatically if parameter sync is true.
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
    pub fn dispatch_events(&mut self, dispatcher : &mut KEventDispatcher, sync : bool) {

        // First get the event count to poll. This is important to prevent bloking.
        let event_count = self.__get_event_count();  // Private platform inline implementation

        for _ in 0..event_count {
            // Fetch event
            let event = self.__poll_event();    // Private platform inline implementation

            // Let KWindow handle event first.
            if !self.handle_kwindow_event(&event) {
                // Then dispatch event via dispatcher of KWindow didn't handle it.
                dispatcher.dispatch(&event);
            }
        }

        // Sync events with display server
        if sync {
            self.__sync_events();   // Private platform inline implementation
        }
    }

    /// Confine cursor to window, preventing it from exiting boundaries.
    pub fn confine_cursor(&mut self) {
        // Confined only if released.
        if !self.cursor.confined {
            self.__confine_cursor();
            self.cursor.confined = true;
        }
    }

     /// Get the cursor position with as a pair (x,y).
     #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
     pub fn get_cursor_position(&self) -> (i32, i32) {
         #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
         self.cursor.position
     }

    /// Get the display server provider identification.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    pub fn get_display_server_provider(&self) -> super::linux::server::KLinuxDisplayServerProvider{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]
       self.display_server.provider
    }

    /// Get the display server connection.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    pub fn get_display_server_connection(&self) -> *const super::linux::server::Display{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]
        self.display_server.display
    }
    
    /// Get the display server window handle.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    pub fn get_display_server_window(&self) -> *const super::linux::server::Window{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]
        self.display_server.window
    }

    /// Get the count of events that need handling.
    pub fn get_event_count(&self) -> usize {
        self.__get_event_count()    // Private platform inline implementation
    }

    /// Get the [KCursorMode] for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn get_cursor_mode(&self) -> KCursorMode{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.cursor.mode
    }

     /// Returns position (x,y) of the [KWindow].
     #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
     pub fn get_position(&self) -> (i32, i32) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
     }

    /// Returns dimension (width, height) of the [KWindow].
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn get_size(&self) -> (u32, u32) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.size
    }

    /// Returns list of connected screens with details.
    pub fn get_screen_list(&self) -> &KScreenList {
        &self.screen_list
    }

    /// Returns the [KWindow] title. 
    pub fn get_title(&self) -> &str {
        todo!()
    }

    /// Hide system default cursor.
    pub fn hide_cursor(&mut self) {
        // Hide only if visible.
        if self.cursor.visible {
            self.__hide_cursor();
            self.cursor.visible = false;
        }
    }



    /// Get if the cursor is confined to the window, preventing it from going further than window boundaries.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_cursor_confined(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.cursor.confined
    }

    /// Get if the default operating system cursor is visible.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_cursor_visible(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.cursor.visible
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

    /// Release cursor from window, allowing it to exit boundaries.
    /// 
    /// Cursor will ALWAYS be released if the window loses focus.
    pub fn release_cursor(&mut self) {
        // Release only if confined.
        if self.cursor.confined {
            self.__release_cursor();
            self.cursor.confined = false;
        }
    }

    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn restore(&mut self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set a new title for the [KWindow].
    pub fn set_title(&mut self, title : &str) {
        todo!()
    }

    /// Show system default cursor.
    pub fn show_cursor(&mut self) {
        // Show only if not visible.
        if !self.cursor.visible {
            self.__show_cursor();
            self.cursor.visible = true;
        }
    }

    /// Set position of [KWindow] according to position (x,y).
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_position(&mut self, position : (i32, i32)){
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
    pub fn set_size(&mut self, dimension : (u32, u32)) -> Result<u8, KWindowError>{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the [KWindow] as fullscreen.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_fullscreen(&mut self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the [KWindow] as minimized.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_minimized(&mut self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the [KWindow] as maximized.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_maximized(&mut self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the cursor position with a pair (x,y).
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_cursor_position(&mut self, position : (i32, i32)){
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]

        self.__set_cursor_position(position);   // Private platform inline implementation
    }

    /// Set the cursor mode for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_cursor_mode(&mut self, mode : KCursorMode) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]

        if mode != self.cursor.mode {
            self.cursor.mode = mode;
            match self.cursor.mode {
                KCursorMode::Pointer => {},
                // Put cursor in center if acceleration
                KCursorMode::Acceleration => self.set_cursor_position(self.center),
            }
        }
    }

    /// Sync all event between client and display server / window manager. 
    /// 
    /// This need to be called at each loop if [KWindow::dispatch_events()] sync parameter = false..
    pub fn sync_events(&self) {
        self.__sync_events();    // Private platform inline implementation
    }
}

/// Private KWindow members
#[doc(hidden)]
impl KWindow {
    /// Return True if width and size are between boundaries.
    fn is_size_within_boundaries(width:u32, height:u32) -> bool {

        if width >= KWINDOW_MIN_WIDTH && width <= KWINDOW_MAX_WIDTH && height >= KWINDOW_MIN_HEIGHT && height <= KWINDOW_MAX_HEIGHT {
            // Withing boundaries
            true
        } else {
            // Boundaries overflow
            false
        }

    }

    /// Handle events use for [KWindow] like resizing, closing, etc...
    /// 
    /// Returns true if event was handle and should not be given to receivers.
    #[inline(always)]
    fn handle_kwindow_event(&mut self, event : &KEvent) -> bool {

        match event {
            KEvent::None => false,
            KEvent::Window(_) => false,
            KEvent::Keyboard(_) => false,
            KEvent::Mouse(event) => match event {
                KEventMouse::Moved(position) => {
                    match self.cursor.mode {
                        KCursorMode::Pointer => {
                            // Register cursor position.
                            self.cursor.position = *position;
                            false
                        },
                        KCursorMode::Acceleration => {
                            if *position == (0,0) {     // Ignore position reset
                                true
                            } else { // Reset cursor position
                                self.set_cursor_position(self.center);
                                false
                            }
                        },
                    }
                },
                _ => false,
            },
            KEvent::Controller(_) => false,
            KEvent::Unknown => false,
        }
        
        // Set cursor to center if Acceleration
        
        /*
        match self.mouse_mode {
            KWindowMouseMode::Pointer(_,_) => false,
            KWindowMouseMode::Acceleration => match event {
                KEvent::Mouse(event) => match event {
                    // Ignore moved if motion = accel && moved == center
                    KEventMouse::Moved(position) => if *position == (0,0) {
                        true
                    } else { // Reset cursor position
                        self.set_cursor_position(self.center);
                        false
                    },
                    _ => false,
                },
                _ => false,
            },
        }
        */
        
        
        
        

    }
}