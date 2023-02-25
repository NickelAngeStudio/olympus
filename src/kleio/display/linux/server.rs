use std::{os::raw::{ c_char, c_ulong }, ffi::{CString, NulError}};
use debug_print::debug_println;

use crate::{kleio::display::{KWindow}, wayland_or_x11, error::{OlympusError, KWindowError}};

use super::{x11::{event::{XEvent, Atom}, bind::{XCloseDisplay, XInternAtom}}};

/// Type used for display server window pointer.
pub type Window = c_ulong;

/// Type used for display server connection pointer. 
pub type Display = c_ulong;

/// Used to query WMState atom
const  WM_STATE_NAME : &str = "_NET_WM_STATE";

/// Used to query maximized properties
const  WM_STATE_MAX_VERT_NAME: &str = "_NET_WM_STATE_MAXIMIZED_VERT";
const  WM_STATE_MAX_HORZ_NAME : &str = "_NET_WM_STATE_MAXIMIZED_HORZ";

/// Used to query minimized property
const  WM_STATE_HIDDEN_NAME : &str = "_NET_WM_STATE_HIDDEN";

/// Used to quere fullscreen property
const  WM_STATE_FULLSCREEN_NAME : &str = "_NET_WM_STATE_FULLSCREEN";


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

    /// X11 only properties
    pub x11_property : KLinuxDisplayServerX11Property,

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

/// Contains X11 only display server properties
pub struct KLinuxDisplayServerX11Property {
    /// Used to fetch X11 events
    pub(crate) x_event : XEvent,    

    /// Used to query window properties
    pub(crate) wm_state : Atom,

    /// Used to query maximized properties
    pub(crate) wm_state_max_vert : Atom,

    /// Used to query maximized properties
    pub(crate) wm_state_max_horz : Atom,

    /// Used to query minimized property
    pub(crate) wm_state_hidden : Atom,

    /// Used to query fullscreen property
    pub(crate) wm_state_fullscreen : Atom,

    /// Used to query atom type
    pub(crate) xa_atom : Atom,
}

impl KLinuxDisplayServerX11Property{
    /// Fetch atoms value with display
    pub fn new(display : *mut u64) -> KLinuxDisplayServerX11Property {
        #[allow(temporary_cstring_as_ptr)]
        unsafe {
            // Query states atoms
            let wm_state = XInternAtom(display, CString::new(WM_STATE_NAME).unwrap().as_ptr(), false);
            let wm_state_max_vert = XInternAtom(display, CString::new(WM_STATE_MAX_VERT_NAME).unwrap().as_ptr(), false);
            let wm_state_max_horz = XInternAtom(display, CString::new(WM_STATE_MAX_HORZ_NAME).unwrap().as_ptr(), false);
            let wm_state_hidden = XInternAtom(display, CString::new(WM_STATE_HIDDEN_NAME).unwrap().as_ptr(), false);
            let wm_state_fullscreen = XInternAtom(display, CString::new(WM_STATE_FULLSCREEN_NAME).unwrap().as_ptr(), false);
            let xa_atom = 4;

            debug_println!("wm_state={}, wm_state_max_vert={}, wm_state_max_horz={}, wm_state_hidden={}, wm_state_fullscreen={}, xa_atom={}",
                wm_state, wm_state_max_vert, wm_state_max_horz, wm_state_hidden, wm_state_fullscreen, xa_atom);

            KLinuxDisplayServerX11Property { x_event : XEvent { _type: 0}, wm_state, wm_state_max_vert, wm_state_max_horz, wm_state_hidden, wm_state_fullscreen, xa_atom }
        }
    }

    /// Empty X11 Atoms
    pub fn empty() -> KLinuxDisplayServerX11Property {
        KLinuxDisplayServerX11Property { x_event : XEvent { _type: 0}, wm_state: 0, wm_state_max_vert: 0, wm_state_max_horz: 0, wm_state_hidden: 0, wm_state_fullscreen: 0, xa_atom: 0 }
    }
}



impl KLinuxDisplayServer {
    /// Create a new KLinuxDisplayServer according to provider.
    pub fn new(width:u32, height:u32, provider : KLinuxDisplayServerProvider) -> Result<KLinuxDisplayServer, OlympusError> {
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
                    let dis_win = KWindow::create_wayland_window(width, height);

                    Ok(KLinuxDisplayServer{ provider, x11_property : KLinuxDisplayServerX11Property::empty(), display : dis_win.0, window : dis_win.1 })

                } else {
                    // No wayland support.
                    Err(OlympusError::KWindow(KWindowError::NotSupported))
                }
            },
            KLinuxDisplayServerProvider::X11 => {
                if KWindow::x11_supported() {
                    let provider = KLinuxDisplayServerProvider::X11;
                    let dis_win = KWindow::create_x11_window(width, height);
                    Ok(KLinuxDisplayServer{ provider, x11_property : KLinuxDisplayServerX11Property::new(dis_win.0), display : dis_win.0, window : dis_win.1 })
                } else {
                    // No x11 support.
                    Err(OlympusError::KWindow(KWindowError::NotSupported))
                }
            },
        }

    }

}