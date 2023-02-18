use std::os::raw::{ c_ulong };
use super::{ KWindowError, event::KEvent};

/// Wayland KWindowManager
#[cfg(all(not(git_workflow), not(feature="no_wayland")))]     // Add Wayland if not remove via feature.
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
#[derive(Debug, PartialEq)]
pub enum LinuxDisplayServerProvider {

    /// Used when specifying provider with [get_linux_display_server].
    Default,

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

    /// Return display server provider id.
    fn get_provider(&self) -> LinuxDisplayServerProvider;

    /// Returns count of x11 events.
    fn get_event_count(&self) -> usize;

    /// Sync all event between client and display server / window manager. 
    fn sync_events(&self);

}

/// Get the appropriate linux display server. 
/// 
/// If provider is set to default, Will try to open Wayland first then X11.
/// 
/// Returns Ok(Box<dyn KLinuxDisplayServer>) if successful.
/// 
/// # Error(s)
/// Returns Err([KWindowError::NoDisplayServer]) if no compatible display server found.
pub fn get_linux_display_server(width:u32, height:u32, provider : LinuxDisplayServerProvider) -> Result<Box<dyn KLinuxDisplayServer>, KWindowError> {
        
        use x11::X11DisplayServer;
        

        #[cfg(all(not(git_workflow), not(feature="no_wayland")))] 
        {
            use wayland::WaylandDisplayServer;

            match provider {
                LinuxDisplayServerProvider::Default => // Try Wayland first then X11
                    if WaylandDisplayServer::is_compatible() {
                        Ok(Box::new(WaylandDisplayServer::new(width, height)))
                    } // Else use X11 display server
                    else if X11DisplayServer::is_compatible() {
                        Ok(Box::new(X11DisplayServer::new(width, height)))
                    } // Return error of NoDisplayServer
                    else {
                        Err(KWindowError::NoDisplayServer)
                    },
                LinuxDisplayServerProvider::Wayland => // Only try Wayland.
                    if WaylandDisplayServer::is_compatible() {
                        Ok(Box::new(WaylandDisplayServer::new(width, height)))
                    } else {
                        Err(KWindowError::NoDisplayServer)
                    },
                LinuxDisplayServerProvider::X11 =>  // Only try x11.
                    if X11DisplayServer::is_compatible() {
                    Ok(Box::new(X11DisplayServer::new(width, height)))
                    } // Return error of NoDisplayServer
                    else {
                        Err(KWindowError::NoDisplayServer)
                    },
            }
        }

        #[cfg(any(feature="git_workflow", feature="no_wayland"))]
        {
            if X11DisplayServer::is_compatible() {
                Ok(Box::new(X11DisplayServer::new(width, height)))
            } // Return error of NoDisplayServer
            else {
                Err(KWindowError::NoDisplayServer)
            }
        }
}