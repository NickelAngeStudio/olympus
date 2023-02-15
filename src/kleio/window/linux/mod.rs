use std::os::raw::{ c_ulong };
use super::{ KWindowError, LinuxDisplayServerProvider, KEvent, KWindowMotionMode};

/// Wayland KWindowManager
#[cfg(not(feature="git_workflow"))]     // The github workflow doesn't have wayland-client.
pub mod wayland;

/// X11 KWindowManager
pub mod x11;

/// Type used for display server window pointer.
pub type Window = c_ulong;

/// Type used for display server connection pointer. 
pub type Display = c_ulong;

/// Linux definition of KWindow.
/// 
/// The Linux KWindow will try to open a connection with Wayland first. 
/// If it failed, it will open a connection with X11 then.
pub struct KWindow {



    // List of receivers.
    //receivers : Vec<Rc<RefCell<dyn KEventReceiver>>>,

    // Motion mode
    //motion_mode : KWindowMotionMode,
    

}

/// Abstraction of Linux display server
pub trait KDisplayServer  {

    /// Returns True if this display server is compatible.
    fn is_compatible() -> bool;
    
    fn get_display_server_provider(&self) -> LinuxDisplayServerProvider;

    fn get_display_server_connection(&self) -> *const Display;
    
    fn get_display_server_window(&self) -> *const Window;

    fn poll_event(&mut self) -> KEvent;

    fn get_event_count(&self) -> usize;

    fn sync_event(&self);

    fn set_title(&self, title : &str);

    fn get_title(&self) -> &str;

    fn set_size(&self, dimension : (usize, usize));

    fn get_size(&self) -> (usize, usize);

    fn set_fullscreen(&self, fullscreen : bool);

    fn is_fullscreen(&self) -> bool;

    fn set_minimized(&self, minimized : bool);

    fn is_minimized(&self) -> bool;

    fn set_maximized(&self, maximized : bool);

    fn is_maximized(&self) -> bool;

    fn restore(&self);

    fn show_cursor(&self,keep_inside_window : bool);

    fn hide_cursor(&self);

    fn is_cursor_hidden(&self);

    fn set_cursor_position(&self, position : (i32, i32));

    fn set_motion_mode(&self, mode : KWindowMotionMode);

    fn get_motion_mode(&self) -> KWindowMotionMode;

}



/*
/// Get the appropriate linux window manager. Will try to open Wayland first then X11.
/// 
/// Returns Ok([KWindowManager]) if successful.
/// 
/// # Error(s)
/// Returns Err([KWindowError::NoDisplayServer]) if no compatible display server found.
#[cfg(wayland)]
pub fn get_linux_window_manager(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<Box<dyn KWindowManager>, KWindowError> {
    use crate::kleio::window::linux::x11::KWindowManagerX11;

        
    use crate::kleio::window::{ linux::wayland::KWindowManagerWayland};

        // Use Wayland display server if compatible
        if KWindowManagerWayland::is_compatible() {
            Ok(Box::new(KWindowManagerWayland::new(pos_x, pos_y, width, height)))
        } // Else use X11 display server
        else if KWindowManagerX11::is_compatible() {
            Ok(Box::new(KWindowManagerX11::new(pos_x, pos_y, width, height)))
        } // Return error of NoDisplayServer
        else {
            Err(KWindowError::NoDisplayServer)
        }
}

/// Get the X11 linux display server window manager.
/// 
/// Returns Ok([KWindowManager]) if successful.
/// 
/// # Error(s)
/// Returns Err([KWindowError::NoDisplayServer]) if not compatible with X11.
#[cfg(not(wayland))]
pub fn get_linux_window_manager(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<Box<dyn KWindowManager>, KWindowError> {
    use crate::kleio::window::linux::x11::KWindowManagerX11;

        if KWindowManagerX11::is_compatible() {
            Ok(Box::new(KWindowManagerX11::new(pos_x, pos_y, width, height)))
        } // Return error of NoDisplayServer
        else {
            Err(KWindowError::NoDisplayServer)
        }
}
*/