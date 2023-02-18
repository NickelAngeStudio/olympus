use super::{event::{ KEvent, KEventDispatcher}, screen::KScreenList};

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

/// Enumeration of possible [KWindow] motion mode.
#[derive(Debug, PartialEq)]
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

    /// Hardware screen list
    screen_list : KScreenList,

    /// Motion mode,
    motion_mode : KWindowMotionMode,

    /// Window center,
    center : (i32, i32),

    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    /// Linux display server (Linux only).
    display_server : Box<dyn super::linux::KLinuxDisplayServer>,

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
    pub fn new(width:u32, height:u32, provider : super::linux::LinuxDisplayServerProvider) -> Result<KWindow, KWindowError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]

        // Make sure dimension are within boundaries.
        if KWindow::is_size_within_boundaries(width, height) {
            // Default motion mode
            let motion_mode = KWindowMotionMode::Location;

            // Default center position
            let center = ((width as i32 / 2), (height as i32 / 2));

            // Linux implementation
            #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
            {
                match super::linux::get_linux_display_server(width, height, provider) {
                    Ok(display_server) => {
                        match KScreenList::new(display_server.get_provider()){
                            Ok(screen_list) => {
                                let kwindow = KWindow { screen_list, motion_mode, center, display_server };
                                Ok(kwindow)
                            },
                            Err(_) => Err(KWindowError::ScreenDetailError),
                        }
                        
                    },
                    Err(err) => Err(err),
                }
            }

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
            // Default motion mode
            let motion_mode = KWindowMotionMode::Location;

            // Default center position
            let center = ((width as i32 / 2), (height as i32 / 2));

            todo!()

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
    /// # Note(s)
    /// After dispatching events, [KWindow::sync_events()] will be called automatically.
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

        // Counter of handled/unhandled events
        let mut handled:(u32,u32) = (0,0);

        // First get the event count to poll. This is important to prevent bloking.
        let event_count = self.get_event_count();

        for _ in 0..event_count {
            // Fetch event
            let event = self.pop_event();

            // Let KWindow handle event first.
            if !self.handle_kwindow_event(&event){

                // Then dispatch event via dispatcher.
                if dispatcher.dispatch(&event) {
                    handled.0 += 1;
                } else {
                    handled.1 += 1;
                }
            }
        }

        // Sync events with display server
        self.sync_events();

        handled
    }

    /// Binding cursor prevent cursor from exiting windows boundaries when focused.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn bind_cursor(&mut self){
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

     /// Get the cursor position with as a pair (x,y).
     #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
     pub fn get_cursor_position(&self) -> (i32, i32) {
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
        
        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
        {
            self.display_server.get_event_count()
        }

        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "android", target_os = "ios", target_os = "windows", target_os = "macos"))))]
        {
            todo!()
        }

    }

    /// Get the motion mode for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn get_motion_mode(&self) -> KWindowMotionMode{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
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
        todo!()
    }

    /// Returns list of connected screens with details.
    pub fn get_screen_list(&self) -> &KScreenList {
        &self.screen_list
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
    pub fn is_cursor_binded(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Get if the default operating system cursor is hidden.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_cursor_hidden(&self) -> bool {
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
    pub fn set_position(&self, position : (i32, i32)){
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
    pub fn set_size(&self, dimension : (u32, u32)) -> Result<u8, KWindowError>{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the [KWindow] as fullscreen.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_fullscreen(&self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the [KWindow] as minimized.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_minimized(&self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        todo!()
    }

    /// Set the [KWindow] as maximized.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_maximized(&self) {
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

    /// Sync all event between client and display server / window manager. 
    /// 
    /// This need to be called at each loop if using [KWindow::poll_event()] instead of [KWindow::dispatch_events()].
    pub fn sync_events(&self) {
        
        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
        {
            self.display_server.sync_events()
        }

        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "android", target_os = "ios", target_os = "windows", target_os = "macos"))))]
        {
            todo!()
        }

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
    fn is_size_within_boundaries(width:u32, height:u32) -> bool {

        if width >= KWINDOW_MIN_WIDTH && width <= KWINDOW_MAX_WIDTH && height >= KWINDOW_MIN_HEIGHT && height <= KWINDOW_MAX_HEIGHT {
            // Withing boundaries
            true
        } else {
            // Boundaries overflow
            false
        }

    }

    /// Pop an event from event queue
    fn pop_event(&mut self) -> KEvent {
        
        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
        {
            self.display_server.pop_event()
        }

        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "android", target_os = "ios", target_os = "windows", target_os = "macos"))))]
        {
            todo!()
        }
    }

    /// Handle events use for [KWindow] like resizing, closing, etc...
    /// 
    /// Returns true if event was handle and should not be given to receivers.
    fn handle_kwindow_event(&mut self, event : &KEvent) -> bool {
        // TODO:Handle
        false
    }
}