use super::{ KWindow, KWindowError };

/// Minimum [KWindow] width allowed.
pub const KWINDOW_MIN_WIDTH : usize = 1;

/// Minimum [KWindow] height allowed.
pub const KWINDOW_MIN_HEIGHT : usize = 1;

/// Maximum [KWindow] width allowed.
pub const KWINDOW_MAX_WIDTH : usize = 65535;

/// Maximum [KWindow] width allowed.
pub const KWINDOW_MAX_HEIGHT : usize = 65535;


/// Implementation of [KWindow] properties.
impl KWindow {   

    /// Set a new title for the [KWindow].
    pub fn set_title(&self, title : &str) {
        todo!()
    }

    /// Returns the [KWindow] title. 
    pub fn get_title(&self) -> &str {
        todo!()
    }

    /// Set position of [KWindow] according to position (x,y).
    pub fn set_position(&self, position : (isize, isize)){
        todo!()
    }

    /// Returns position (x,y) of the [KWindow].
    pub fn get_position(&self) -> (isize, isize) {
        todo!()
    }

    /// Set dimension of [KWindow] according to size (width, height).
    /// 
    /// Returns Ok(0) if successful.
    /// 
    /// # Error(s)
    /// Returns [KWindowError::WindowSizeError] if width and/or height not within allowed boundaries.
    pub fn set_size(&self, dimension : (usize, usize)) -> Result<u8, KWindowError>{
        todo!()
    }

    /// Returns dimension (width, height) of the [KWindow].
    pub fn get_size(&self) -> (usize, usize) {
        todo!()
    }

    /// Set the [KWindow] as fullscreen according to parameters.
    pub fn set_fullscreen(&self, fullscreen : bool) {
        todo!()
    }

    /// Returns if the [KWindow] is fullscreen or not.
    pub fn is_fullscreen(&self) -> bool {
        todo!()
    }

    /// Set the [KWindow] as minimized according to parameters.
    pub fn set_minimized(&self, minimized : bool) {
        todo!()
    }

    /// Returns if the [KWindow] is minimized or not.
    pub fn is_minimized(&self) -> bool {
        todo!()
    }

    /// Set the [KWindow] as maximized according to parameters.
    pub fn set_maximized(&self, maximized : bool) {
        todo!()
    }

    /// Returns if the [KWindow] is maximized or not.
    pub fn is_maximized(&self) -> bool {
        todo!()
    }

    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    pub fn restore(&self) {
        todo!()
    }

    /// Show the default operating system cursor.
    pub fn show_cursor(&self) {
        todo!()
    }

    /// Hide the default operating system cursor.
    pub fn hide_cursor(&self) {
        todo!()
    }

}