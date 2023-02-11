use crate::kleio::window::{ KWindow};

use super::{KEvent, KWindowError};



impl KWindow {
    /// Create a new [KWindow] using position and size with option to set fullscreen or not.
    /// 
    /// Return New [`KWindow`].
    /// 
    /// 
    /// # Error(s)
    /// Returns [KWindowError::NoDisplayServer] if no display server found on Linux.
    /// 
    /// Returns [KWindowError::WindowSizeError] if width and/or height aren't within allowed boundaries.
    /// 
    /// # Note(s)
    /// On Linux distribution, this will try to create a [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) window first
    /// if #[cfg(wayland)] is defined, else a [x11](https://en.wikipedia.org/wiki/X_Window_System) window if not compatible with Wayland.
    //#[cfg(any(doc, all(not(target_family = "wasm"), target_os = "linux")))]
    #[cfg(doc)]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize, fullscreen : bool) -> Result<KWindow, KWindowError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
        // This is unified documentation only. Implementations are in linux, window and macos modules.
    }

    /// Create a new [KWindow] for mobile devices.
    /// 
    /// Return New [`KWindow`] created.
    //#[cfg(any(doc, all(not(target_family = "wasm"), target_os = "linux")))]
    #[cfg(doc)]
    pub fn new() -> Result<KWindow, KWindowError> {
        #![cfg_attr(docsrs, doc(cfg(any(target_os = "android", target_os = "ios"))))]
        // This is unified documentation only. Implementations are in android and ios modules.
    }



    /// Set a new title for the [KWindow].
    #[cfg(doc)]
    pub fn set_title(&self, title : &str) {
        todo!()
    }

    /// Returns the [KWindow] title. 
    #[cfg(doc)]
    pub fn get_title(&self) -> &str {
        todo!()
    }

    /// Set position of [KWindow] according to position (x,y).
    #[cfg(doc)]
    pub fn set_position(&self, position : (isize, isize)){
        todo!()
    }

    /// Returns position (x,y) of the [KWindow].
    #[cfg(doc)]
    pub fn get_position(&self) -> (isize, isize) {
        todo!()
    }

    /// Set dimension of [KWindow] according to size (width, height).
    /// 
    /// Returns Ok(0) if successful.
    /// 
    /// # Error(s)
    /// Returns [KWindowError::WindowSizeError] if width and/or height not within allowed boundaries.
    #[cfg(doc)]
    pub fn set_size(&self, dimension : (usize, usize)) -> Result<u8, KWindowError>{
        todo!()
    }

    /// Returns dimension (width, height) of the [KWindow].
    #[cfg(doc)]
    pub fn get_size(&self) -> (usize, usize) {
        todo!()
    }

    /// Set the [KWindow] as fullscreen according to parameters.
    #[cfg(doc)]
    pub fn set_fullscreen(&self, fullscreen : bool) {
        todo!()
    }

    /// Returns if the [KWindow] is fullscreen or not.
    #[cfg(doc)]
    pub fn is_fullscreen(&self) -> bool {
        todo!()
    }

    /// Set the [KWindow] as minimized according to parameters.
    #[cfg(doc)]
    pub fn set_minimized(&self, minimized : bool) {
        todo!()
    }

    /// Returns if the [KWindow] is minimized or not.
    #[cfg(doc)]
    pub fn is_minimized(&self) -> bool {
        todo!()
    }

    /// Set the [KWindow] as maximized according to parameters.
    #[cfg(doc)]
    pub fn set_maximized(&self, maximized : bool) {
        todo!()
    }

    /// Returns if the [KWindow] is maximized or not.
    #[cfg(doc)]
    pub fn is_maximized(&self) -> bool {
        todo!()
    }

    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    #[cfg(doc)]
    pub fn restore(&self) {
        todo!()
    }

    /// Show the default operating system cursor.
    #[cfg(doc)]
    pub fn show_cursor(&self) {
        todo!()
    }

    /// Hide the default operating system cursor.
    #[cfg(doc)]
    pub fn hide_cursor(&self) {
        todo!()
    }

    /// Get if the default operating system cursor is hidden.
    #[cfg(doc)]
    pub fn is_cursor_hidden(&self) {

    }

    /// Set the cursor position with a pair (x,y).
    #[cfg(doc)]
    pub fn set_cursor_position(&self, position : (i32, i32)){

    }










}