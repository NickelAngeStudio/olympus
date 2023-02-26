use crate::error::OlympusError;
use super::{KWindow, event::KEvent};

/// Implementation of privates elements relatives to MacOS
#[doc(hidden)]
impl KWindow {
    /// Create new KWindow
    pub(super) fn __new(width:u32, height:u32) -> Result<KWindow, OlympusError> {
        todo!()
    }
        
    // Get cursor position
    #[inline(always)]
    pub(super) fn __get_cursor_position(&self) -> (i32, i32){
        todo!()
    }

    // Pop an event from the queue
    #[inline(always)]
    pub(super) fn __poll_event(&mut self) -> KEvent {
        todo!()
    }

    // Sync an event from the queue
    #[inline(always)]
    pub(super) fn __sync_events(&self) {
        todo!()
    }

    /// Get the count of events that need handling.
    #[inline(always)]
    pub(super) fn __get_event_count(&self) -> usize {
        todo!()
    }

    /// Set the cursor position
    #[inline(always)]
    pub(super) fn __set_cursor_position(&mut self, position : (i32, i32)){
        todo!()
    }

    /// Hide system default cursor.
    #[inline(always)]
    pub(super) fn __hide_cursor(&mut self) {
        todo!()
    }

    /// Show system default cursor.
    #[inline(always)]
    pub(super) fn __show_cursor(&mut self) {
        todo!()
    }

    /// Confine cursor to window, preventing it from exiting boundaries.
    #[inline(always)]
    pub(super) fn __confine_cursor(&mut self) {
        todo!()
    }


    /// Release cursor from window, allowing it to exit boundaries.
    #[inline(always)]
    pub(super) fn __release_cursor(&mut self) {
        todo!()
    }


    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    #[inline(always)]
    pub(super) fn __restore(&mut self) {
        todo!()
    }

    /// Set a new title for the [KWindow].
    #[inline(always)]
    pub(super) fn __set_title(&mut self) {
        todo!()
    }

    /// Set position of [KWindow] according to position (x,y).
    #[inline(always)]
    pub(super) fn __set_position(&mut self){
        todo!()
    }

    /// Set dimension of [KWindow] according to size (width, height).
    #[inline(always)]
    pub(super) fn __set_size(&mut self) {
        todo!()
    }

    /// Set the [KWindow] as fullscreen.
    #[inline(always)]
    pub(super) fn __set_fullscreen(&mut self) {
        todo!()
    }
}