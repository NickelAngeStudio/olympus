use crate::kleio::display::event::{KEventWindow, KEventKeyboard, KEventController};
use debug_print::debug_println;
use crate::error::OlympusError;
use crate::error::KWindowError;
use super::KWindowProvider;

use super::KWindowManager;
use super::{event::{ KEvent, KEventDispatcher}, screen::KScreenList, KWindowProperty};
use super::{ KCursorMode };
use super::KWindowFullscreenMode;

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

/// Create and manage a window frame for display. 
/// 
/// [KWindow] broadcasts [KEvent] to multiple [KEventReceiver] via [KWindow::dispatch_events()].
/// 
/// Act as a window factory.
/// 
/// TODO: More doc about OS, dispatch, and Examples
pub struct KWindow {

    /// KWindow properties
    pub(super) property : KWindowProperty,

    /// [KWindowManager] that manage the window.
    pub(crate) manager : Box<dyn KWindowManager>,
}

impl KWindow {
    /// Create a new sized [KWindow] in the middle of the main default screen.
    /// 
    /// Return New [`KWindow`].
    /// 
    /// 
    /// # Error(s)
    /// Returns [OlympusError::KWindow(KWindowError::NoDisplayServer)] if no display server found on Linux.
    /// 
    /// Returns [OlympusError::KWindow(KWindowError::SizeError)] if width and/or height aren't within allowed boundaries.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn new(width:u32, height:u32) -> Result<KWindow, OlympusError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]

        // Make sure dimension are within boundaries.
        if KWindow::is_size_within_boundaries(width, height) {
            // Linux only implementation
            #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
            {
                // Try Wayland first
                match KWindow::from_provider(KWindowProvider::Wayland, width, height) {
                    Ok(window) => {
                        Ok(window)
                    },
                    Err(_) => {
                        // Then try x11
                        match KWindow::from_provider(KWindowProvider::X11, width, height) {
                            Ok(window) => {
                                Ok(window)
                            },
                            Err(_) => {
                                // Return error that no display server were found.
                                Err(OlympusError::KWindow(KWindowError::NoDisplayServer))
                            },
                        }
                    },
                }
            }

            // Windows only implementation
            #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "windows"))))]
            {
                todo!()
            }

            // MacOs only implementation
            #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "macos"))))]
            {
                todo!()
            }

        } else {
            Err(OlympusError::KWindow(KWindowError::SizeError))
        }

    }


    /// Create a new [KWindow] for mobile devices.
    /// 
    /// Return New [`KWindow`] created.
    //#[cfg(any(doc, all(not(target_family = "wasm"), target_os = "linux")))]
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "android", target_os = "ios"))))]
    pub fn new() -> Result<KWindow, OlympusError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "android", target_os = "ios"))))]
        
        todo!()
    }

    /// Create a [KWindow] from specified provider in the middle of primary screen.
    /// 
    /// Used exclusively for operating system with multiple display server and/or window manager.
    /// 
    /// # Error(s)
    /// Returns [OlympusError::KWindow(KWindowError::NotSupported)] if display server not supported.
    /// 
    /// Returns [OlympusError::KWindow(KWindowError::SizeError)] if width and/or height aren't within allowed boundaries.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    pub fn from_provider(provider : KWindowProvider, width:u32, height:u32) -> Result<KWindow, OlympusError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]

        
        if KWindow::is_size_within_boundaries(width, height) {
            todo!()   

        } else {
            Err(OlympusError::KWindow(KWindowError::SizeError))
        }
    }




    /// Dispatch [KEvent] to [KEventReceiver] using a [KEventDispatcher].
    /// 
    /// # Note(s)
    /// After dispatching events, [KWindow::sync()] will be called automatically if parameter sync is true.
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
        let event_count = self.manager.get_event_count();

        for _ in 0..event_count {
            // Fetch event
            let event = self.manager.poll_event();

            // Let KWindow handle event first.
            if !self.handle_kwindow_event(&event) {
                // Then dispatch event via dispatcher of KWindow didn't handle it.
                dispatcher.dispatch(&event);
            }
        }

        // Sync events with window manager
        if sync {
            self.manager.sync();
        }
    }

    /// Confine cursor to window, preventing it from exiting boundaries.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]        
    pub fn confine_cursor(&mut self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        // Confined only if released.
        if !self.property.cursor.confined {
            self.manager.confine_cursor();
            self.property.cursor.confined = true;
        }
    }

    /// Get the cursor position with as a pair (x,y).
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn get_cursor_position(&self) -> (i32, i32) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.property.cursor.position
    }

    /// Get immutable reference to [KWindowManager].
    /// 
    /// Karen want to speak to your [KWindowManager]...
    pub fn get_window_manager(&self) -> &Box<dyn KWindowManager>{
        &self.manager
    }


    /// Get the [KCursorMode] for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn get_cursor_mode(&self) -> KCursorMode{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.property.cursor.mode
    }

     /// Returns position (x,y) of the [KWindow].
     #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
     pub fn get_position(&self) -> (i32, i32) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.property.position
     }

    /// Returns dimension (width, height) of the [KWindow].
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn get_size(&self) -> (u32, u32) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.property.size
    }

    /// Returns a list of connected screens with details.
    pub fn get_screen_list() -> KScreenList {
        // Linux only implementation
        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
        {
            todo!()
        }

        // Windows only implementation
        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "windows"))))]
        {
            todo!()
        }

        // MacOs only implementation
        #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "macos"))))]
        {
            todo!()
        }
    }

    /// Returns the [KWindow] title. 
    pub fn get_title(&self) -> &str {
        &self.property.title.as_str()
    }

    /// Hide system default cursor.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn hide_cursor(&mut self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        // Hide only if visible.
        if self.property.cursor.visible {
            self.manager.hide_cursor();
            self.property.cursor.visible = false;
        }
    }

    /// Get if the cursor is confined to the window, preventing it from going further than window boundaries.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_cursor_confined(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.property.cursor.confined
    }

    /// Get if the default operating system cursor is visible.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_cursor_visible(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.property.cursor.visible
    }

    /// Returns if the [KWindow] is fullscreen or not.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_fullscreen(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.property.fullscreen
    }
    

    /// Returns if the [KWindow] is maximized or not.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_maximized(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.property.maximized
    }

    /// Returns if the [KWindow] is minimized or not.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn is_minimized(&self) -> bool {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.property.minimized
    }

    /// Release cursor from window, allowing it to exit boundaries.
    /// 
    /// Cursor will ALWAYS be released if the window loses focus.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn release_cursor(&mut self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        // Release only if confined.
        if self.property.cursor.confined {
            self.release_cursor();
            self.property.cursor.confined = false;
        }
    }

    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn restore(&mut self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.restore();
    }

    /// Set a new title for the [KWindow].
    pub fn set_title(&mut self, title : &str) {
        self.property.title = String::from(title);
        self.set_title(self.property.title.as_str());
    }

    /// Show system default cursor.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn show_cursor(&mut self) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        // Show only if not visible.
        if !self.property.cursor.visible {
            self.show_cursor();
            self.property.cursor.visible = true;
        }
    }

    /// Set position of [KWindow] according to position (x,y).
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_position(&mut self, position : (i32, i32)){
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        self.property.position = position;
        self.set_position(position);
    }

    /// Set dimension of [KWindow] according to size (width, height).
    /// 
    /// Returns Ok(0) if successful.
    /// 
    /// # Error(s)
    /// Returns [OlympusError::KWindow(KWindowError::SizeError)] if width and/or height not within allowed boundaries.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_size(&mut self, size : (u32, u32)) -> Result<u8, OlympusError>{
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        // Make sure dimension are within boundaries.
        if KWindow::is_size_within_boundaries(size.0, size.1) {
            self.property.size = size;
            self.set_size(size);
            Ok(0)
        } else {
            Err(OlympusError::KWindow(KWindowError::SizeError))
        }
    }

    /// Set the [KWindow] as fullscreen according to [KWindowFullscreenMode] parameter.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_fullscreen(&mut self, fs_mode : KWindowFullscreenMode) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]

        if !self.property.fullscreen {
            self.manager.set_fullscreen(fs_mode);
        }
    }

    /// Set the cursor position with a pair (x,y).
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_cursor_position(&mut self, position : (i32, i32)){
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]

        self.property.cursor.position = position;
        self.set_cursor_position(position); 
    }

    /// Set the cursor mode for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn set_cursor_mode(&mut self, mode : KCursorMode) {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]

        if mode != self.property.cursor.mode {
            self.property.cursor.mode = mode;
            match self.property.cursor.mode {
                KCursorMode::Pointer => {},
                // Put cursor in center if acceleration
                KCursorMode::Acceleration => self.set_cursor_position(self.property.center),
            }
        }
    }

    /// Sync all event between client and display server / window manager. 
    /// 
    /// This need to be called at each loop if [KWindow::dispatch_events()] sync parameter = false..
    pub fn sync(&self) {
        self.sync();  
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
            KEvent::None => true,   // Any event None must not pass
            KEvent::Window(event) => self.handle_kwindow_window_event(event),
            KEvent::Keyboard(event) => self.handle_kwindow_keyboard_event(event),
            KEvent::Mouse(event) => self.handle_kwindow_mouse_event(event),
            KEvent::Controller(event) => self.handle_kwindow_controller_event(event),
            KEvent::Unknown => false,
        }
    }

    /// Handle KEventWindow for KWindow.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    #[inline(always)]
    fn handle_kwindow_window_event(&mut self, event : &KEventWindow) -> bool {
        
        debug_println!("\x1b[92mKEventWindow::{:?}\x1b[0m", event);
        match event {
            KEventWindow::Moved(position) => {
                self.property.position = *position;
                false
            },
            KEventWindow::Resized(size) => {
                self.property.size = *size;
                self.property.center = (self.property.size.0 as i32 / 2, self.property.size.1 as i32 / 2);
                false
            },
            KEventWindow::MovedResized(position, size) => {
                self.property.position = *position;
                self.property.size = *size;
                self.property.center = (self.property.size.0 as i32 / 2, self.property.size.1 as i32 / 2);
                false
            },
            KEventWindow::CursorEnter() => {
                // Hide cursor if supposed to be hidden.
                if !self.property.cursor.visible {
                    self.manager.hide_cursor();
                }
                false
            },
            KEventWindow::CursorLeave() => {
                // Show hidden cursor when out of window.
                if !self.property.cursor.visible {
                    self.manager.show_cursor();
                }
                false
            },
            KEventWindow::Focus() => {
                // If cursor is confined, confine cursor on focus.
                if self.property.cursor.confined {
                    self.manager.confine_cursor();
                }
                false
            },
            KEventWindow::Blur() => {
                // If cursor is confined, release cursor on blur.
                if self.property.cursor.confined {
                    self.manager.release_cursor();
                }
                false
            },
            KEventWindow::Close() => {
                false
            },

            _ => false,
            
        }
    }

    /// Handle KEventKeyboard for KWindow.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    #[inline(always)]
    fn handle_kwindow_keyboard_event(&mut self, event : &KEventKeyboard) -> bool {
        debug_println!("\x1b[93mKEventKeyboard::{:?}\x1b[0m", event);
        match event {
            KEventKeyboard::KeyDown(_) => {},
            KEventKeyboard::KeyUp(_) => {},
        }
        false
    }

    /// Handle KEventMouse for KWindow.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    #[inline(always)]
    fn handle_kwindow_mouse_event(&mut self, event : &KEventMouse) -> bool {
        debug_println!("\x1b[94mKEventMouse::{:?}\x1b[0m", event);
        match event {
            KEventMouse::Moved(position) => match self.property.cursor.mode {
                KCursorMode::Pointer => {
                    // Register cursor position.
                    self.property.cursor.position = *position;
                    false
                },
                KCursorMode::Acceleration => {
                    if *position == (0,0) {     // Ignore position reset
                        true
                    } else { // Reset cursor position
                        self.property.cursor.position = self.property.center;
                        self.set_cursor_position(self.property.center);
                        false
                    }
                }
            },
            _ => false,
        }
    }
    
    /// Handle KWindowController for KWindow.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    #[inline(always)]
    fn handle_kwindow_controller_event(&mut self, event : &KEventController) -> bool {
        debug_println!("\x1b[95mKEventController::{:?}\x1b[0m", event);
        match event {
            _=> false,
        }
    }
}