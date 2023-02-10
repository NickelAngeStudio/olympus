// By default, Wayland implementation is NOT included.


// If added via #[cfg(wayland) the Linux KWindow will try to open a connection with Wayland first. 
// If it failed, it will open a connection with X11 then.



use super::{KWindowManager, KWindowError};

/// Wayland KWindowManager
#[cfg(wayland)]
pub mod wayland;

/// X11 KWindowManager
pub mod x11;


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