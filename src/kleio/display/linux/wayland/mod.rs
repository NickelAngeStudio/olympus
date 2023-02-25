use std::panic::catch_unwind;

use crate::kleio::display::{KWindow, event::KEvent};

use self::bind::{wl_display_connect, wl_display_disconnect};

use super::server::{Display, Window};



/// Waylind C function binds
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
#[allow(non_camel_case_types)]    // Imported C global aren't formatted according to convention.
pub mod bind;

/// Contains Wayland screen fetch function
pub mod screen;

/// Implementation of privates elements relatives to Wayland display server
#[doc(hidden)]
impl KWindow {

    // Get cursor position
    #[inline(always)]
    pub(super) fn wayland_get_cursor_position(&self) -> (i32, i32){
        todo!()
    }


    // Pop an event from the queue
    #[inline(always)]
    pub(super) fn wayland_poll_event(&mut self) -> KEvent {
        todo!()
    }

    // Sync an event from the queue
    #[inline(always)]
    pub(super) fn wayland_sync_events(&self) {
        todo!()
    }

    /// Get the count of events that need handling.
    #[inline(always)]
    pub(super) fn wayland_get_event_count(&self) -> usize {
        todo!()
    }

    /// Set the cursor position
    #[inline(always)]
    pub(super) fn wayland_set_cursor_position(&mut self, position : (i32, i32)){
        todo!()
    }

    /// Confine cursor to window, preventing it from exiting boundaries.
    #[inline(always)]
    pub fn wayland_confine_cursor(&mut self) {
        todo!()
    }

    /// Release cursor from window, allowing it to exit boundaries.
    #[inline(always)]
    pub fn wayland_release_cursor(&mut self) {
        todo!()
    }

    /// Hide system default cursor.
    #[inline(always)]
    pub fn wayland_hide_cursor(&mut self) {
        todo!()
    }

    /// Show system default cursor.
    #[inline(always)]
    pub fn wayland_show_cursor(&mut self) {
        todo!()
    }

    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    #[inline(always)]
    pub fn wayland_restore(&mut self) {
        todo!()
    }

    /// Set a new title for the [KWindow].
    #[inline(always)]
    pub(super) fn wayland_set_title(&mut self) {
        todo!()
    }

    /// Set position of [KWindow] according to position (x,y).
    #[inline(always)]
    pub(super) fn wayland_set_position(&mut self){
        todo!()
    }

    /// Set dimension of [KWindow] according to size (width, height).
    #[inline(always)]
    pub(super) fn wayland_set_size(&mut self) {
        todo!()
    }

    /// Set the [KWindow] as fullscreen.
    #[inline(always)]
    pub(super) fn wayland_set_fullscreen(&mut self) {
        todo!()
    }
  
    /// Get if Wayland is supported.
    #[inline(always)]
    pub(crate) fn wayland_supported() -> bool {
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

    /// Create connection to Wayland and window
    #[inline(always)]
    pub(crate) fn create_wayland_window(width:u32, height:u32) -> (*mut Display, *mut Window) {
        todo!()
    }
}


/*
pub struct WaylandDisplayServer {

}

impl WaylandDisplayServer {
    pub fn new(width:u32, height:u32) -> WaylandDisplayServer {
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

    fn get_display_server_provider(&self) -> super::LinuxDisplayServerProvider {
        LinuxDisplayServerProvider::Wayland
    }

    fn get_event_count(&self) -> usize {
        todo!()
    }

    fn sync_events(&self) {
        todo!()
    }

    fn get_display_server_connection(&self) -> *const super::Display {
        todo!()
    }

    fn get_display_server_window(&self) -> *const super::Window {
        todo!()
    }

    fn set_cursor_position(&mut self, position : (i32, i32), size : (u32, u32)) {
        todo!()
    }

    fn get_cursor_position(&self) -> (i32, i32) {
        todo!()
    }


}

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