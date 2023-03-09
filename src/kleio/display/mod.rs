/// # Re-export for Public API
#[doc(inline)]
pub use window::KWindow as KWindow;
pub use cursor::KCursorProperty as KCursorProperty;
pub use manager::KWindowProvider as KWindowProvider;
pub use manager::KWindowManager as KWindowManager;


/// [KWindowManager] definition.
#[doc(hidden)]
pub mod manager;

/// [KWindow] definition.
#[doc(hidden)]
pub mod window;

/// [KCursor] mode and properties.
#[cfg(any(doc, any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub mod cursor;

/// Hardware screen details and supported resolutions.
pub mod screen;

/// Elements relatives to [KWindow] events and handling.
pub mod event;

/// Linux implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "linux"))]
#[doc(hidden)]
pub mod linux;

/// Windows shell implementations of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "windows"))]
#[doc(hidden)]
pub mod windows;

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


/// [KWindow] fullscreen mode enumeration.
#[cfg(any(doc, any(target_os = "linux", target_os = "windows", target_os = "macos")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub enum KFullscreenMode {
    /// Window will be set fullscreen in the current screen this window belong to.
    CurrentScreen,

    /// Window will be set fullscreen in the primary screen.
    PrimaryScreen,

    /// Window will be set fullscreen for entire desktop which can be set across multiple physical screen.
    DesktopScreen,
}

/// [KWindow] properties.
#[cfg(any(doc, any(target_os = "linux", target_os = "windows", target_os = "macos")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub struct KWindowProperty {

    /// Window title
    pub title : String,

    /// Cursor mode and properties
    pub cursor : KCursorProperty,

    /// Position of window as pair of i32(x,y)
    pub position : (i32, i32),

    /// Size of window as pair of u32 (width, height).
    pub size : (u32, u32),

    /// Window center,
    pub center : (i32, i32),

    /// Window is minimized
    pub is_minimized : bool,

    /// Window is maximized
    pub is_maximized : bool,

    /// Window is fullscreen
    pub is_fullscreen : bool,

}