/// # Re-export for Public API
#[doc(inline)]
pub use window::KWindow as KWindow;
pub use window::KWindowError as KWindowError;
pub use window::KWindowMotionMode as KWindowMotionMode;

/// [KWindow] definition.
#[doc(hidden)]
pub mod window;

/// Elements relatives to [KWindow] events and handling.
pub mod event;

/// Linux implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "linux"))]
#[doc(hidden)]
pub mod linux;

/// Windows shell implementations of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "windows"))]
#[doc(hidden)]
pub mod shell;

/// Android implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "android"))]
#[doc(hidden)]
pub mod android;

/// IOS implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "ios"))]
#[doc(hidden)]
pub mod ios;

/// MacOS implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "macos"))]
#[doc(hidden)]
pub mod macos;

/// Web assembly implementation of KWindow
#[cfg(target_family = "wasm")]
#[doc(hidden)]
pub mod wasm;

/*
/// Implementation of [KWindow] [KEventReceiver] handling.
impl KWindow {
    /// Create a new [KWindow] using position and size.
    /// 
    /// Return New [`KWindow`].
    /// 
    /// 
    /// # Error(s)
    /// Returns [KWindowError::NoDisplayServer] if no display server found on Linux.
    /// 
    /// Returns [KWindowError::WindowSizeError] if width and/or height not within allowed boundaries.
    /// 
    /// # Note(s)
    /// On Linux distribution, this will try to create a [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) window first
    /// if #[cfg(wayland)] is defined, else a [x11](https://en.wikipedia.org/wiki/X_Window_System) window if not compatible with Wayland.
    //#[cfg(any(doc, all(not(target_family = "wasm"), target_os = "linux")))]
    #[cfg(any(target_os = "linux", doc))]
    #[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        
        

        // Make sure dimension are within boundaries.
        if KWindow::is_size_within_boundaries(width, height) {

            use self::linux::get_linux_window_manager;
            match get_linux_window_manager(pos_x, pos_y, width, height) {
                Ok(window_manager) => Ok(KWindow{ receivers : Vec::new(), window_manager}),
                Err(err) => Err(err),
            }
            
        } else {
            Err(KWindowError::WindowSizeError)
        }
        
    }

    //#[cfg(any(doc, all(not(target_family = "wasm"), target_os = "windows")))]
    #[cfg(any(target_os = "windows", doc))]
    #[cfg(any(windows, doc))]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        // Make sure dimensions are within boundaries.
        if KWindow::is_size_within_boundaries(width, height) {
            // Dimensions are within boundaries.
            
            Err(KWindowError::NotSupported)
        } else {
            // Dimensions aren't within boundaries.
            Err(KWindowError::WindowSizeError)
        }
    }

    #[cfg(all(not(target_family = "wasm"), target_os = "android"))]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        // Make sure dimensions are within boundaries.
        if KWindow::is_size_within_boundaries(width, height) {
            // Dimensions are within boundaries.
            
            Err(KWindowError::NotSupported)
        } else {
            // Dimensions aren't within boundaries.
            Err(KWindowError::WindowSizeError)
        }
    }

    #[cfg(all(not(target_family = "wasm"), target_os = "ios"))]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        // Make sure dimensions are within boundaries.
        if KWindow::is_size_within_boundaries(width, height) {
            // Dimensions are within boundaries.
            
            Err(KWindowError::NotSupported)
        } else {
            // Dimensions aren't within boundaries.
            Err(KWindowError::WindowSizeError)
        }
    }

    #[cfg(all(not(target_family = "wasm"), target_os = "macos"))]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        // Make sure dimensions are within boundaries.
        if KWindow::is_size_within_boundaries(width, height) {
            // Dimensions are within boundaries.
            
            Err(KWindowError::NotSupported)
        } else {
            // Dimensions aren't within boundaries.
            Err(KWindowError::WindowSizeError)
        }
    }

    #[cfg(target_family = "wasm")]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        // Make sure dimensions are within boundaries.
        if KWindow::is_size_within_boundaries(width, height) {
            // Dimensions are within boundaries.
            
            Err(KWindowError::NotSupported)
        } else {
            // Dimensions aren't within boundaries.
            Err(KWindowError::WindowSizeError)
        }
    }

    /// Create a [KWindow] from a [KWindowManager]. 
    /// 
    /// The [KWindow] will take ownership of the [KWindowManager].
    /// 
    /// Used when porting [KWindow] to another platform.
    pub fn from(window_manager : Box<dyn KWindowManager>) -> KWindow {
        KWindow { receivers : Vec::new(),  window_manager }
    }

    /// Returns the [KWindowManagerId] of the [KWindowManager].
    /// 
    /// Used by [KWindowRenderer] to target the correct display server.
    pub fn get_window_manager_id(&self) -> u8 {
        self.window_manager.get_id()
    }

    /// Used to downcast [KWindowManager] trait to target the correct display server for [KWindowRenderer].
    /// 
    /// Returns some reference to the inner value if it is of type T, or None if it isn’t.
    /// 
    /// # Example(s)
    /// Downcasting to KWindowManagerX11 : 
    /// ```no_run
    /// match window.downcast_window_manager::<KWindowManagerX11>() {
    ///     Some(wmx11 : &KWindowManagerX11) => todo!(),
    ///     None => todo!(),
    /// }
    /// ```
    pub fn downcast_window_manager<T: Any>(&self) -> Option<&T> {
        self.window_manager.as_any().downcast_ref::<T>()
    }

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
*/
