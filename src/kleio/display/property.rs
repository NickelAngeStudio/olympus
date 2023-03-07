use super::screen::KScreen;
#[allow(unused_imports)]
use super::{ KWindow, event::{ KEvent, KEventMouse }};

/// [KWindow] properties.
#[cfg(any(doc, any(target_os = "linux", target_os = "windows", target_os = "macos")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub struct KWindowProperty {

    /// Window title
    pub(super) title : String,

    /// Cursor mode and properties
    pub(super) cursor : KCursorProperty,

    /// Position of window as pair of i32(x,y)
    pub(super) position : (i32, i32),

    /// Size of window as pair of u32 (width, height).
    pub(super) size : (u32, u32),

    /// Window center,
    pub(super) center : (i32, i32),

    /// Window is minimized
    pub(super) minimized : bool,

    /// Window is maximized
    pub(super) maximized : bool,

    /// Window is fullscreen
    pub(super) fullscreen : bool,

}

/// Enumeration of possible [KWindow] cursor mode used for [KEvent].
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(any(doc, any(target_os = "linux", target_os = "windows", target_os = "macos")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub enum KCursorMode {
    /// [KEventMouse] events will give the (x,y) location of the cursor on the window. 
    /// 
    /// Usually used for user interfaces interactions.
    Pointer,

    /// [KEventMouse] events will give the (x,y) acceleration of the cursor instead of the position.
    /// 
    /// Usually used for 3d camera and direct mouse inputs.
    Acceleration,
}

/// [KWindow] cursor properties.
#[cfg(any(doc, any(target_os = "linux", target_os = "windows", target_os = "macos")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub struct KCursorProperty {
    /// Motion mode of the mouse
    pub(super) mode : KCursorMode,

    /// Current cursor position
    pub(super) position : (i32, i32),

    /// Is cursor visible?
    pub(super) visible : bool,

    /// Is cursor confined?
    pub(super) confined : bool, 
}

/// [KWindow] fullscreen mode enumeration.
#[cfg(any(doc, any(target_os = "linux", target_os = "windows", target_os = "macos")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub enum KWindowFullscreenMode<'a> {
    /// Window will be set fullscreen in the current screen this window belong to.
    CurrentScreen,

    /// Window will be set fullscreen in the primary screen.
    PrimaryScreen,

    /// Window will be set fullscreen in multiple screen that have the same resolution as reference screen.
    /// 
    /// # Example
    /// Take 3 screens with those resolution :
    /// 1. |1920x1080| 2. |1920x1080| 3.|2560x1440|
    /// 
    /// If screen 1 or 2 is given as reference, extended screen will use screen 1 and 2 since they are the same resolution.
    /// If screen 3 is given as reference, only screen 3 will be used for extended screen.
    ExtendedScreen(&'a KScreen),

    /// Window will be set fullscreen in the selected screen.
    SelectScreen(&'a KScreen),



}