/// Enumeration of possible [KWindow] cursor mode used for [KEvent].
#[derive(Debug, Clone, Copy, PartialEq)]
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
pub struct KCursor {
    /// Motion mode of the mouse
    pub(super) mode : KCursorMode,

    /// Current cursor position
    pub(super) position : (i32, i32),

    /// Is cursor visible?
    pub(super) visible : bool,

    /// Is cursor confined?
    pub(super) confined : bool, 
}