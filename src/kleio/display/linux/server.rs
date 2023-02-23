use std::os::raw::{ c_ulong };
use crate::{kleio::display::{KWindowError, KWindow}, wayland_or_x11};

use super::{x11::{event::XEvent, bind::XCloseDisplay}};

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
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KLinuxDisplayServerProvider {

    /// Try Wayland first, then X Window.
    Default,

    /// [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) display server.
    Wayland,

    /// [X Window System](https://en.wikipedia.org/wiki/X_Window_System) display server.
    X11,
}


/// Contains elements relatives to X11 and Wayland display server.
pub struct KLinuxDisplayServer {

    /// Used to determine which provider is used
    pub provider : KLinuxDisplayServerProvider,

    /// Used to fetch X11 events
    pub x_event : XEvent,

    /// Display connection pointer
    pub display : *mut Display,

    /// Window handle pointer
    pub window : *mut Window,
}

impl Drop for KLinuxDisplayServer {
    fn drop(&mut self) {
        wayland_or_x11!{self.provider, {
            todo!()
        } , {
            unsafe {
                // Close display server connection.
                XCloseDisplay(self.display);
            }
        }}
    }
}

impl KLinuxDisplayServer {
    /// Create a new KLinuxDisplayServer according to provider.
    pub fn new(width:u32, height:u32, provider : KLinuxDisplayServerProvider) -> Result<KLinuxDisplayServer, KWindowError> {
        // Is wayland support activated?
        #[cfg(all(not(git_workflow), not(feature="no_wayland")))] 
        {
            match provider {
                KLinuxDisplayServerProvider::Default => {

                    match KLinuxDisplayServer::new(width, height, KLinuxDisplayServerProvider::Wayland) {
                        Ok(klds) => Ok(klds),
                        Err(_) => KLinuxDisplayServer::new(width, height, KLinuxDisplayServerProvider::X11),
                    }
                },
                KLinuxDisplayServerProvider::Wayland => {
                    if KWindow::wayland_supported() {
                        let provider = KLinuxDisplayServerProvider::Wayland;
                        let x_event = XEvent { _type: 0};
                        let dis_win = KWindow::create_wayland_window(width, height);

                        Ok(KLinuxDisplayServer{ provider, x_event, display : dis_win.0, window : dis_win.1 })

                    } else {
                        // No wayland support.
                        Err(KWindowError::NotSupported)
                    }
                },
                KLinuxDisplayServerProvider::X11 => {
                    if KWindow::x11_supported() {
                        let provider = KLinuxDisplayServerProvider::X11;
                        let x_event = XEvent { _type: 0};
                        let dis_win = KWindow::create_x11_window(width, height);
                        Ok(KLinuxDisplayServer{ provider, x_event, display : dis_win.0, window : dis_win.1 })
                    } else {
                        // No x11 support.
                        Err(KWindowError::NotSupported)
                    }
                },
            }

        }

        // No wayland support
        #[cfg(any(feature="git_workflow", feature="no_wayland"))]
        {
            match provider {
                KLinuxDisplayServerProvider::Wayland => {
                    // No wayland support.
                    Err(KWindowError::NotSupported)
                },
                _ => {
                    if x11_supported() {
                        let provider = KLinuxDisplayServerProvider::X11;
                        let x_event = XEvent { _type: 0};
                        let dis_win = super::x11::create_x11_window(width, height);
                        Ok(KLinuxDisplayServer{ provider, x_event, display : dis_win.0, window : dis_win.1 })
                    } else {
                        // No x11 support.
                        Err(KWindowError::NotSupported)
                    }
                },
            }

        }

    }

}