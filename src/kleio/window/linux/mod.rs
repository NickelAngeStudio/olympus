use std::os::raw::{ c_ulong };
use super::{ KWindowError, event::KEvent, KWindowMotionMode};

/// Wayland KWindowManager
#[cfg(not(feature="no_wayland"))]     // Add Wayland if not remove via feature.
pub mod wayland;

/// X11 KWindowManager
pub mod x11;

/// Type used for display server window pointer.
pub type Window = c_ulong;

/// Type used for display server connection pointer. 
pub type Display = c_ulong;


/// Enumeration of linux display server provider.
/// 
/// Linux can support more than 1 display server so it is important to enumerate
/// supported display server and be ready for future addition.
#[cfg(any(doc, all(not(target_family = "wasm"), target_os = "linux")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]
pub enum LinuxDisplayServerProvider {
    /// [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) display server.
    Wayland,

    /// [X Window System](https://en.wikipedia.org/wiki/X_Window_System) display server.
    X11,
}

/// Abstraction of Linux display server
pub trait KLinuxDisplayServer  {

    /// Return true if display server is compatible with current linux distro.
    fn is_compatible() -> bool where Self:Sized;

    /// Pop an event from the queue. 
    /// 
    /// Warning(s)
    /// Will lock until next event if no events.
    fn pop_event(&mut self) -> KEvent;

}

/// Get the appropriate linux display server. Will try to open Wayland first then X11.
/// 
/// Returns Ok(Box<dyn KLinuxDisplayServer>) if successful.
/// 
/// # Error(s)
/// Returns Err([KWindowError::NoDisplayServer]) if no compatible display server found.
pub fn get_linux_display_server(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<Box<dyn KLinuxDisplayServer>, KWindowError> {
        
        use x11::X11DisplayServer;
        

        #[cfg(not(feature="no_wayland"))]
        {
            use wayland::WaylandDisplayServer;

            // Use Wayland display server if compatible
            if WaylandDisplayServer::is_compatible() {
                Ok(Box::new(WaylandDisplayServer::new(pos_x, pos_y, width, height)))
            } // Else use X11 display server
            else if X11DisplayServer::is_compatible() {
                Ok(Box::new(X11DisplayServer::new(pos_x, pos_y, width, height)))
            } // Return error of NoDisplayServer
            else {
                Err(KWindowError::NoDisplayServer)
            }
        }

        #[cfg(feature="no_wayland")]
        {
            if X11DisplayServer::is_compatible() {
                Ok(Box::new(X11DisplayServer::new(pos_x, pos_y, width, height)))
            } // Return error of NoDisplayServer
            else {
                Err(KWindowError::NoDisplayServer)
            }
        }
}