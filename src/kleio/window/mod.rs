use std::cell::RefCell;
use std::rc::Rc;

/// # Re-export for Public API
#[doc(inline)]
pub use renderer::KWindowRenderer as KWindowRenderer;
pub use manager::KWindowManager as KWindowManager;
pub use manager::KWindowManagerId as KWindowManagerId;
pub use event::KEvent as KEvent;
pub use event::mouse::KEventMouse as KEventMouse;
pub use event::window::KEventWindow as KEventWindow;
pub use event::controller::KEventController as KEventController;
pub use event::keyboard::KEventKeyboard as KEventKeyboard;
pub use receiver::KEventReceiver as KEventReceiver;

/// [KWindow] event definition.
#[doc(hidden)]
pub mod event;

/// [KWindowManager] definition.
#[doc(hidden)]
pub mod manager;

/// [KWindow] renderer abstraction.
#[doc(hidden)]
pub mod renderer;

// Kwindow global documentation of implementation
#[cfg(doc)]
pub mod doc;

/// [KWindow] receiver abstraction and dispatcher implementation.
#[doc(hidden)]
pub mod receiver;

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


/// Enumeration of possible [KWindow] errors.
pub enum KWindowError {

    /// Happens when a window manager is not supported.
    NotSupported,

    /// Happens when no display server is found.
    NoDisplayServer,

    /// Happens when an error occurred while creating a [KWindow] using KWindow::from().
    FromWindowManagerError,

    /// Happens when trying to add the same [KEventReceiver] twice to a [KWindow].
    ReceiverAlreadyExists,

    /// Happens when trying to remove a  [KEventReceiver] not added to a [KWindow].
    ReceiverNotFound,

    /// Happens when trying to dispatch events when no [KEventReceiver] were added.
    DispatchNoReceiver,

    /// Happens when trying to resize a [KWindow] outside of allowed boundaries.
    WindowSizeError,


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

/// Minimum [KWindow] width allowed.
pub const KWINDOW_MIN_WIDTH : usize = 1;

/// Minimum [KWindow] height allowed.
pub const KWINDOW_MIN_HEIGHT : usize = 1;

/// Maximum [KWindow] width allowed.
pub const KWINDOW_MAX_WIDTH : usize = 65535;

/// Maximum [KWindow] width allowed.
pub const KWINDOW_MAX_HEIGHT : usize = 65535;


impl std::fmt::Debug for KWindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "KWindowError"),
        }
    }
}


/// Create and manage a window frame for display.
/// 
/// [KWindow] broadcasts [KEvent] to multiple [KEventReceiver] via [KWindow::dispatch_events()].
/// 
/// TODO: More doc about OS, dispatch, and Examples
pub struct KWindow {
    /// List of receivers.
    receivers : Vec<Rc<RefCell<dyn KEventReceiver>>>,

    /// Motion mode
    motion_mode : KWindowMotionMode,
    

}

/// Implementation of [KWindow] properties that applies to all platform.
impl KWindow {


    /// Set the motion mode for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    pub fn set_motion_mode(&self, mode : KWindowMotionMode) {
        self.motion_mode = mode;
    }

    /// Get the motion mode for the [KWindow] [KEventMouse](enum.KEventMouse.html) events.
    pub fn get_motion_mode(&self) -> KWindowMotionMode{
        self.motion_mode
    }
}


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
