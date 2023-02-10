use std::any::Any;
use super::KEvent;

// For document references
#[allow(unused_imports)]  
use super::KWindow;


/// Enumeration of possible [KWindowManager] provider id.
/// 
/// Since [KWindowManagerId] are u8, user can use his own value > 6
/// when porting to another platform.
#[allow(non_snake_case)]
pub mod KWindowManagerId {
    /// Linux Wayland display server.
    pub const WAYLAND : u8 = 1;

    /// Linux X11 display server.
    pub const X11 : u8 = 2;

    /// Windows graphical user interface.
    pub const SHELL : u8 = 3;

    /// Android graphical user interface.
    pub const ANDROID : u8 = 4;

    /// MacOS graphical user interface
    pub const MACOS : u8 = 5;

    /// IOS graphical user interface
    pub const IOS : u8 = 6;
}

/// # Abstraction of window manager that supplies window frame.
pub trait KWindowManager {
    /// Create a new instance of [KWindowManager] and open the window frame.
    /// 
    /// Returns [KWindowManager] created.
    fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Self where Self: Sized;

    /// Returns count of events to be polled.
    fn get_event_count(&self) -> usize;

    /// Get an event from window manager as [KEvent].
    fn poll_event(&mut self) -> KEvent;

    /// Sync event with display manager.
    fn sync_event(&self);

    /// Get the [KWindowManagerId] that manage the window.
    fn get_id(&self) -> u8;

    /// Get self as [Any], allowing downcasting.
    /// 
    /// Implementation code should be as follow :
    /// ```no_run
    /// fn as_any(&self) -> &dyn Any {
    ///     self
    /// }
    /// ```
    /// 
    /// # Example(s)
    /// Downcasting to KWindowManagerX11
    /// ```no_run
    /// match window.get_window_manager().as_any().downcast_ref::<KWindowManagerX11>() {
    /// Some(dc) => {
    ///    println!("DC Worked! Proof, display={:?}", dc.get_display());
    ///     },
    ///     None => panic!("DC Failed!"),
    /// }
    /// ```
    fn as_any(&self) -> &dyn Any;

    /// Set a new title for the [KWindow].
    fn set_title(&self, title : &str);

    /// Returns the [KWindow] title. 
    fn get_title(&self) -> &str;

    /// Set position of [KWindow] according to position (x,y).
    fn set_position(&self, position : (isize, isize)){
        todo!()
    }

    /// Returns position (x,y) of the [KWindow].
    fn get_position(&self) -> (isize, isize) {
        todo!()
    }

    /// Set dimension of [KWindow] according to size (width, height).
    fn set_size(&self, dimension : (usize, usize));

    /// Returns dimension (width, height) of the [KWindow].
    fn get_size(&self) -> (usize, usize);

    /// Set the [KWindow] as fullscreen according to parameters.
    fn set_fullscreen(&self, fullscreen : bool);

    /// Returns if the [KWindow] is fullscreen or not.
    fn is_fullscreen(&self) -> bool;

    /// Set the [KWindow] as minimized according to parameters.
    fn set_minimized(&self, minimized : bool);

    /// Returns if the [KWindow] is minimized or not.
    fn is_minimized(&self) -> bool;

    /// Set the [KWindow] as maximized according to parameters.
    fn set_maximized(&self, maximized : bool);

    /// Returns if the [KWindow] is maximized or not.
    fn is_maximized(&self) -> bool;

    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    fn restore(&self);

    /// Show the default operating system cursor.
    fn show_cursor(&self);

    /// Hide the default operating system cursor.
    fn hide_cursor(&self);

    
}

