use crate::kleio::display::{event::KEvent, KWindowError};

use self::bind::{wl_display_connect, wl_display, wl_display_disconnect};


/// Waylind C function binds
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
#[allow(non_camel_case_types)]    // Imported C global aren't formatted according to convention.
pub mod bind;

/// Contains Wayland screen fetch function
pub mod screen;

pub struct WaylandDisplayServer {

}

impl WaylandDisplayServer {
    pub fn new(width:usize, height:usize) -> WaylandDisplayServer {
        WaylandDisplayServer {}
    }
}

impl super::KLinuxDisplayServer for WaylandDisplayServer{
    fn is_compatible() -> bool where Self:Sized {
        todo!()
    }

    fn pop_event(&mut self) -> KEvent {
        todo!()
    }
}
/*
/// # Wayland KWindow backend
pub struct KWindowManagerWayland {

    display : *mut wl_display,



}

/// [KWindowManagerWayland] members.
impl KWindowManagerWayland {
    /// Verify if system is compatible with Wayland display server.
    /// 
    /// Returns True if compatible, false otherwise.
    pub(crate) fn is_compatible() -> bool {
        unsafe {
            // Try to call C function with error handling.
            let result = catch_unwind(|| {
                wl_display_connect(std::ptr::null())
            }); 
            match result {
                Ok(display) => {
                    if display == std::ptr::null_mut() {
                        false
                    } else {
                        // Disconnect display before returning true
                        wl_display_disconnect(display);

                        true
                    }

                },
                // C function crashed. Wayland not supported.
                Err(_) => false,
            }
        }
    }
}


impl KWindowManager for KWindowManagerWayland {
    fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Self where Self: Sized {
        


        unsafe {
            // Connect to display
            let display = wl_display_connect(std::ptr::null());

            // TODO:Wayland implementation
            todo!()
            
          
        }

    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn poll_event(&mut self) -> KEvent {
        todo!()
    }

    fn get_id(&self) -> u8 {
        KWindowManagerId::WAYLAND
    }

    fn get_event_count(&self) -> usize {
        todo!()
    }

    fn sync_event(&self) {
        todo!()
    }

    fn set_title(&self, title : &str) {
        todo!()
    }

    fn get_title(&self) -> &str {
        todo!()
    }

    fn set_size(&self, dimension : (usize, usize)) {
        todo!()
    }

    fn get_size(&self) -> (usize, usize) {
        todo!()
    }

    fn set_fullscreen(&self, fullscreen : bool) {
        todo!()
    }

    fn is_fullscreen(&self) -> bool {
        todo!()
    }

    fn set_minimized(&self, minimized : bool) {
        todo!()
    }

    fn is_minimized(&self) -> bool {
        todo!()
    }

    fn set_maximized(&self, maximized : bool) {
        todo!()
    }

    fn is_maximized(&self) -> bool {
        todo!()
    }

    fn restore(&self) {
        todo!()
    }

    fn show_cursor(&self) {
        todo!()
    }

    fn hide_cursor(&self) {
        todo!()
    }

    fn set_cursor_position(&self, position : (i32, i32)) {
        todo!()
    }
}
*/