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
    pub mode : KCursorMode,

    /// Current cursor position
    pub position : (i32, i32),

    /// Is cursor visible?
    pub visible : bool,

    /// Is cursor confined?
    pub confined : bool, 
}