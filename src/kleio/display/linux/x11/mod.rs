use std::ffi::{CStr, CString};
use std::os::raw::{ c_int, c_long, c_uint, c_ulong, c_char, c_uchar, c_short, c_void };
use std::panic::resume_unwind;
use std::ptr::null_mut;
use std::{panic::catch_unwind};
use debug_print::debug_println;


use crate::kleio::display::linux::x11::bind::{XFree, XUnmapWindow, XGetAtomName, XFlush};
use crate::kleio::display::{KWindow, KCursorMode, KWindowFullscreenMode};
use crate::kleio::display::event::KEventWindow;
use crate::kleio::display::linux::x11::constant::{GrabModeAsync, SubstructureRedirectMask, SubstructureNotifyMask};
use crate::kleio::display::{linux::x11::{bind::{XDefaultRootWindow, XCreateSimpleWindow, XMapWindow, XSelectInput, XSync, XEventsQueued}, 
    constant::{KeyPressMask, ButtonPressMask, ExposureMask, KeyPress, KeyRelease, ButtonPress, MotionNotify, LeaveNotify, 
    ButtonRelease, EnterNotify, FocusIn, FocusOut, KeymapNotify, Expose, GraphicsExpose, NoExpose, VisibilityNotify, 
    CreateNotify, DestroyNotify, UnmapNotify, MapNotify, MapRequest, ReparentNotify, ConfigureNotify, ConfigureRequest, 
    GravityNotify, CirculateNotify, CirculateRequest, PropertyNotify, SelectionClear, SelectionRequest, SelectionNotify, 
    ColormapNotify, ClientMessage, MappingNotify, GenericEvent}}, event::KEvent, event::KEventMouse, event::KEventKeyboard};

use self::attributes::{XWindowAttributes};
use self::bind::{XWarpPointer, XFixesHideCursor, XGrabPointer, XFixesShowCursor, XUngrabPointer, XGetWindowProperty, XStoreName, 
    XChangeProperty, XGetWindowAttributes, XTranslateCoordinates, 
    XResizeWindow, XMoveWindow, XSendEvent, XDestroyWindow};
use self::constant::{CurrentTime, VisibilityUnobscured, PropModeReplace};
use self::event::{Atom, XClientMessageEvent_data, XClientMessageEvent};
use self::{ bind::{XOpenDisplay, XCloseDisplay, XNextEvent}, constant::{KeyReleaseMask, ButtonReleaseMask, LeaveWindowMask, EnterWindowMask, Button1MotionMask, PointerMotionMask, Button3MotionMask, Button2MotionMask, Button5MotionMask, Button4MotionMask, ButtonMotionMask, StructureNotifyMask, ResizeRedirectMask, VisibilityChangeMask, FocusChangeMask, PropertyChangeMask}};

use super::server::{ Display, Window, KLinuxDisplayServerX11Property };

/// Contains X11 contants definition
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
pub mod constant;

/// Contains X11 Event definition
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_snake_case)]            // Imported C members aren't formatted according to convention.
pub mod event;

/// Contains X11 Window attributes
pub mod attributes;

/// Contains X11 C functions Bind
pub mod bind;

/// Contains X11 screen fetch function
pub mod screen;



/// Shortcut macro used to change x11 atoms properties
macro_rules! x11_change_property {
    ($display:expr, $window:expr, $x11_property:expr, $property:ident $(,$atoms:ident)+) => {

        // Put atoms in 1 array.
        let atoms_arr = [$($x11_property.$atoms,)+];

        // Push properties change
        XChangeProperty($display, $window, $x11_property.$property,
            $x11_property.xa_atom, 32, PropModeReplace, std::mem::transmute(&atoms_arr), atoms_arr.len() as i32);
    }
}


/// Implementation of privates elements relatives to X11 display server
#[doc(hidden)]
impl KWindow {

    // Sync an event from the queue
    #[inline(always)]
    pub(super) fn x11_sync_events(&self) {
        unsafe {
            XSync(self.display_server.display, false);
        }
    }

    /// Get the count of events that need handling.
    #[inline(always)]
    pub(super) fn x11_get_event_count(&self) -> usize {
        unsafe {
            XEventsQueued(self.display_server.display, 0).try_into().unwrap()
        }   
    }

    /// Set the cursor position
    #[inline(always)]
    pub(super) fn x11_set_cursor_position(&mut self, position : (i32, i32)){
        unsafe {
            XWarpPointer(self.display_server.display, self.display_server.window, self.display_server.window, 0, 0, 
                self.property.size.0, self.property.size.1, position.0,  position.1);
        }
    }

    /// Hide system default cursor.
    #[inline(always)]
    pub fn x11_hide_cursor(&mut self) {
        unsafe {
            XFixesHideCursor(self.display_server.display, self.display_server.window);
            self.display_server.x11_property.x_hide_cursor_flag = true;
        }
    }

    /// Show system default cursor.
    #[inline(always)]
    pub fn x11_show_cursor(&mut self) {
        unsafe {
            if self.display_server.x11_property.x_hide_cursor_flag {    // Make sure X hide cursor was called prior to show.
                XFixesShowCursor(self.display_server.display, self.display_server.window);
                self.display_server.x11_property.x_hide_cursor_flag = false;
            }       
        }
    }

    /// Confine cursor to window, preventing it from exiting boundaries.
    #[inline(always)]
    pub fn x11_confine_cursor(&mut self) {
        unsafe {
            XGrabPointer(self.display_server.display, self.display_server.window, true, 
            0, GrabModeAsync.try_into().unwrap(), GrabModeAsync.try_into().unwrap(), self.display_server.window, 0, CurrentTime);
        }
    }

    /// Release cursor from window, allowing it to exit boundaries.
    #[inline(always)]
    pub fn x11_release_cursor(&mut self) {
        unsafe {
            XUngrabPointer(self.display_server.display, CurrentTime);
        }
    }

    /// Set a new title for the [KWindow].
    #[inline(always)]
    pub(super) fn x11_set_title(&mut self) {
        unsafe {
            self.display_server.x11_property.wm_title = CString::from_raw(self.property.title.as_mut_ptr() as *mut i8);
            XStoreName(self.display_server.display, self.display_server.window, self.display_server.x11_property.wm_title.as_ptr() as *mut i8);
        }
    }

    /// Set a size of [KWindow].
    #[inline(always)]
    pub(super) fn x11_set_size(&mut self) {
        unsafe {
            XResizeWindow(self.display_server.display, self.display_server.window, self.property.size.0, self.property.size.1);
        }
    }

    /// Set a position of [KWindow].
    #[inline(always)]
    pub(super) fn x11_set_position(&mut self) {
        unsafe {
            XMoveWindow(self.display_server.display, self.display_server.window, self.property.position.0, self.property.position.1);
        }
    }

    /// Set the [KWindow] as fullscreen.
    /// 
    /// To change the state of a mapped window, a Client MUST send a _NET_WM_STATE client message to the root window: 
    /// with XSendEvent using XClientMessageEvent.
    /// 
    /// Reference(s)
    /// <https://specifications.freedesktop.org/wm-spec/1.3/ar01s05.html>
    #[inline(always)]
    pub(super) fn x11_set_fullscreen(&mut self, mode : KWindowFullscreenMode) {
        unsafe {

            /*
            let xwa = Self::get_x11_window_attributes(self.display_server.display, self.display_server.window);
            match mode {
                KWindowFullscreenMode::CurrentScreen => {

                },
                KWindowFullscreenMode::PrimaryScreen => {

                },
                KWindowFullscreenMode::ExtendedScreen(screen) => {

                },
                KWindowFullscreenMode::SelectScreen(screen) => {

                },
            }
            let screen = self.screen_list.get_primary_screen().unwrap();
            //XMoveResizeWindow(self.display_server.display, self.display_server.window, -1 * xwa.x, -1 * xwa.y, screen.get_current_resolution().0, 
            //screen.get_current_resolution().1);
        
            //XSetWindowBorderWidth(self.display_server.display, self.display_server.window, 0);
            
            // Unmap window
            XUnmapWindow(self.display_server.display, self.display_server.window);
            XFlush(self.display_server.display);

            // Change properties
            x11_change_property!(self.display_server.display, self.display_server.window, self.display_server.x11_property, 
                _NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_FULLSCREEN);

            

            // Remap window
            XMapWindow(self.display_server.display, self.display_server.window);
            XFlush(self.display_server.display);

            debug_println!("WP = {:?}", self.get_x11_window_states());

                
            self.x11_sync_events();
            */

            // To change the state of a mapped window, a Client MUST send a _NET_WM_STATE client message to the root window: 
            // with XSendEvent using XClientMessageEvent.
            // Reference

                /*
                
            // Message to send via XSendEvent
            let mut message = XClientMessageEvent { 
                _type: ClientMessage ,
                _serial: 0, 
                _send_event: false, 
                _display: self.display_server.display, 
                _window: self.display_server.window as u64, 
                _message_type: self.display_server.x11_property._NET_WM_STATE, 
                _format: 32, 
                _data: XClientMessageEvent_data { _b: [0;20], _s: [0;10], 
                    _l: [self.display_server.x11_property._NET_WM_STATE_ADD as c_long,
                        self.display_server.x11_property._NET_WM_STATE_MAXIMIZED_HORZ as c_long, 
                        self.display_server.x11_property._NET_WM_STATE_MAXIMIZED_VERT as c_long,0,0 ] } };

            debug_println!("MESSAGE BEF={:?}", message);

           let res = XSendEvent(self.display_server.display, XDefaultRootWindow(self.display_server.display), 
            false, SubstructureRedirectMask | SubstructureNotifyMask, &mut message);
          
            debug_println!("RES={}", res);

            let mut message = XClientMessageEvent { 
                _type: ClientMessage ,
                _serial: 0, 
                _send_event: false, 
                _display: self.display_server.display, 
                _window: self.display_server.window as u64, 
                _message_type: self.display_server.x11_property._NET_WM_STATE, 
                _format: 32, 
                _data: XClientMessageEvent_data { _b: [0;20], _s: [0;10], 
                    _l: [self.display_server.x11_property._NET_WM_STATE_ADD as c_long,
                        self.display_server.x11_property._NET_WM_STATE_FULLSCREEN as c_long, 0,0,0 ] } };

            debug_println!("MESSAGE BEF={:?}", message);

           let res = XSendEvent(self.display_server.display, XDefaultRootWindow(self.display_server.display), 
            false, SubstructureRedirectMask | SubstructureNotifyMask, &mut message);


            XFlush(self.display_server.display);

            debug_println!("MESSAGE AFT={:?}", message);

            debug_println!("RES={}", res);
            */

            // Destroy current window
            XDestroyWindow(self.display_server.display, self.display_server.window);

            // Recreate new window as fullscreen
            self.display_server.window = XCreateSimpleWindow(self.display_server.display, XDefaultRootWindow(self.display_server.display), 0,0,
                    self.get_size().0, self.get_size().1, 4, 0, 0);

             // Set window Type to normal
             x11_change_property!(self.display_server.display, self.display_server.window, self.display_server.x11_property, 
                _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL);

             // Allowed actions
             x11_change_property!(self.display_server.display, self.display_server.window, self.display_server.x11_property, 
                _NET_WM_ALLOWED_ACTIONS, _NET_WM_ACTION_FULLSCREEN, _NET_WM_ACTION_MINIMIZE, _NET_WM_ACTION_CHANGE_DESKTOP,
                 _NET_WM_ACTION_CLOSE, _NET_WM_ACTION_ABOVE, _NET_WM_ACTION_BELOW);

            // Set as fullscreen
            x11_change_property!(self.display_server.display, self.display_server.window, self.display_server.x11_property, 
                _NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_FULLSCREEN);
 
            
             // Map window to display
             XMapWindow(self.display_server.display, self.display_server.window);

             let mask : i64    = KeyPressMask | KeyReleaseMask |             // Keyboard Button Down and Up
                                 ButtonPressMask | ButtonReleaseMask |       // Controller button??? TBD 
                                 EnterWindowMask | LeaveWindowMask |         // Window focus, blur
                                 PointerMotionMask | Button1MotionMask | 
                                 Button2MotionMask | Button3MotionMask |
                                 Button4MotionMask | Button5MotionMask |
                                 ButtonMotionMask |                          // Mouse motion??? TBD
                                 StructureNotifyMask | // ResizeRedirectMask |
                                 VisibilityChangeMask | FocusChangeMask |
                                 PropertyChangeMask | ExposureMask;                        // Window event I guess??
 
             // Mask of events to receive
             XSelectInput(self.display_server.display, self.display_server.window, mask);

             XFlush(self.display_server.display);
            
            
        }
    }
        
    /// Get if x11 display server is supported.
    #[inline(always)]
    pub(super) fn x11_supported() -> bool {
        unsafe {
            // Try to call C function with error handling.
            let result = catch_unwind(|| {
                XOpenDisplay(std::ptr::null())
            }); 

            match result {
                Ok(display) => {
                    if display == std::ptr::null_mut() {
                        false
                    } else {
                        // Disconnect display before returning true
                        XCloseDisplay(display);

                        true
                    }
                },

                // Error occurred, not compatible.
                Err(_) => false,
            }
        }
    }


    /// Create connection to X11 and window
    #[inline(always)]
    pub(super) fn create_x11_window(width:u32, height:u32) -> (KLinuxDisplayServerX11Property, *mut Display, *mut Window) {
        unsafe {
            // Create display connection
            let display = XOpenDisplay(std::ptr::null());

            let window = XCreateSimpleWindow(display, XDefaultRootWindow(display), 0,0,
                    width as u32, height as u32, 4, 0, 0);

            // Create x11 Properties
            let x11_prop = KLinuxDisplayServerX11Property::new(display);

            // Set window Type to normal
            x11_change_property!(display, window, x11_prop, _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL);

            // Allowed actions
            x11_change_property!(display, window, x11_prop, _NET_WM_ALLOWED_ACTIONS, _NET_WM_ACTION_FULLSCREEN, _NET_WM_ACTION_MINIMIZE, _NET_WM_ACTION_CHANGE_DESKTOP,
                _NET_WM_ACTION_CLOSE, _NET_WM_ACTION_ABOVE, _NET_WM_ACTION_BELOW);


            // Map window to display
            XMapWindow(display, window);

            let mask : i64    = KeyPressMask | KeyReleaseMask |             // Keyboard Button Down and Up
                                ButtonPressMask | ButtonReleaseMask |       // Controller button??? TBD 
                                EnterWindowMask | LeaveWindowMask |         // Window focus, blur
                                PointerMotionMask | Button1MotionMask | 
                                Button2MotionMask | Button3MotionMask |
                                Button4MotionMask | Button5MotionMask |
                                ButtonMotionMask |                          // Mouse motion??? TBD
                                StructureNotifyMask | // ResizeRedirectMask |
                                VisibilityChangeMask | FocusChangeMask |
                                PropertyChangeMask | ExposureMask;                        // Window event I guess??

            // Mask of events to receive
            XSelectInput(display, window, mask);

            
            
            // Return properties,  display and window pointer
            (x11_prop, display, window)
        }
    }

    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    #[inline(always)]
    pub(super) fn x11_restore(&mut self) {
        todo!()
    }

    // Pop an event from the queue
    #[inline(always)]
    #[allow(non_upper_case_globals)]            // Imported C members aren't formatted according to convention.
    pub(super) fn x11_poll_event(&mut self) -> KEvent {
        unsafe {

            XNextEvent(self.display_server.display, &mut self.display_server.x11_property.x_event);
            let xevent = self.display_server.x11_property.x_event; 
            
            match xevent._type {

                // Keyboard key pressed
                KeyPress => KEvent::Keyboard(KEventKeyboard::KeyDown(xevent._xkey._keycode)),

                // Keyboard key release
                KeyRelease=> KEvent::Keyboard(KEventKeyboard::KeyUp(xevent._xkey._keycode)),


                ButtonPress=> { debug_println!("KWindow({:p}), ButtonPress({})", self, xevent._type); KEvent::Unknown },
                ButtonRelease=> { debug_println!("KWindow({:p}), ButtonRelease({})", self, xevent._type); KEvent::Unknown },

                // Cursor moved
                MotionNotify=> {    
                    match self.property.cursor.mode {   
                        KCursorMode::Pointer => KEvent::Mouse(KEventMouse::Moved((xevent._xmotion._x, xevent._xmotion._y))),
                        KCursorMode::Acceleration => KEvent::Mouse(KEventMouse::Moved((xevent._xmotion._x - self.property.center.0, 
                            xevent._xmotion._y - self.property.center.1))),
                    }
                },

                // Cursor entered window
                EnterNotify=> KEvent::Window(KEventWindow::CursorEnter()),

                // Cursor left window
                LeaveNotify=> KEvent::Window(KEventWindow::CursorLeave()),

                // Window got focus
                FocusIn=> KEvent::Window(KEventWindow::Focus()),

                // Window lost focus
                FocusOut=> KEvent::Window(KEventWindow::Blur()),

                KeymapNotify=> { debug_println!("KWindow({:p}), KeymapNotify({})", self, xevent._type); KEvent::Unknown },

                // Part of window need to be redrawed 
                Expose=> { 
                    KEvent::Window(KEventWindow::Exposed((xevent._xexpose._x, xevent._xexpose._y), (xevent._xexpose._width as u32, xevent._xexpose._height as u32)))
                },
                GraphicsExpose=> { debug_println!("KWindow({:p}), GraphicsExpose({})", self, xevent._type); KEvent::Unknown },
                NoExpose=> { debug_println!("KWindow({:p}), NoExpose({})", self, xevent._type); KEvent::Unknown },
                VisibilityNotify=> { 
                    if xevent._xvisibility._state == VisibilityUnobscured {
                        KEvent::Window(KEventWindow::Shown())
                    } else {
                        KEvent::Window(KEventWindow::Hidden())
                    }
                },
                CreateNotify=> { debug_println!("KWindow({:p}), CreateNotify({})", self, xevent._type); KEvent::Unknown },
                DestroyNotify=> { debug_println!("KWindow({:p}), DestroyNotify({})", self, xevent._type); KEvent::Unknown },
                UnmapNotify=> { debug_println!("KWindow({:p}), UnmapNotify({})", self, xevent._type); KEvent::Unknown },
                MapNotify=> { debug_println!("KWindow({:p}), MapNotify({})", self, xevent._type); KEvent::Unknown },
                MapRequest=> { debug_println!("KWindow({:p}), MapRequest({})", self, xevent._type); KEvent::Unknown },
                ReparentNotify=> { debug_println!("KWindow({:p}), ReparentNotify({})", self, xevent._type); KEvent::Unknown },

                // Window position and/or size changed
                ConfigureNotify=> { 
                    let position = (xevent._xconfigure._x, xevent._xconfigure._y);
                    let size = (xevent._xconfigure._width as u32, xevent._xconfigure._height as u32);

                    if position != self.property.position && size != self.property.size {
                        KEvent::Window(KEventWindow::MovedResized(position, size))
                    } else if position != self.property.position {
                        KEvent::Window(KEventWindow::Moved(position))
                    } else if size != self.property.size  {
                        KEvent::Window(KEventWindow::Resized(size))
                    } else {
                        KEvent::Unknown
                    }
                },

                ConfigureRequest=> { debug_println!("KWindow({:p}), ConfigureRequest({})", self, xevent._type); KEvent::Unknown },
                GravityNotify=> { debug_println!("KWindow({:p}), GravityNotify({})", self, xevent._type); KEvent::Unknown },

                CirculateNotify=> { debug_println!("KWindow({:p}), CirculateNotify({})", self, xevent._type); KEvent::Unknown },
                CirculateRequest=> { debug_println!("KWindow({:p}), CirculateRequest({})", self, xevent._type); KEvent::Unknown },
                PropertyNotify=> { // This section could use a lil refactorisation.
                    let states = self.get_x11_window_states();
                    
                    
                    if states.0 {   // Send minimized if not already registered.
                        if !self.property.minimized {
                            self.property.minimized = true;
                            return KEvent::Window(KEventWindow::Minimized());
                        }
                    } else {    // Send restore if not already registered.
                        if self.property.minimized {
                            self.property.minimized = false;
                            return KEvent::Window(KEventWindow::Restored());
                        }
                    }

                    if states.1 {   // Send maximized if not already registered.
                        if !self.property.maximized {
                            self.property.maximized = true;
                            return KEvent::Window(KEventWindow::Maximized());
                        }
                    } else {    // Send restore if not already registered.
                        if self.property.maximized {
                            self.property.maximized = false;
                            return KEvent::Window(KEventWindow::Restored());
                        }
                    }

                    if states.2 {   // Send fullscreen if not already registered.
                        if !self.property.fullscreen {
                            self.property.fullscreen = true;
                            return KEvent::Window(KEventWindow::Fullscreen());
                        }
                    } else {    // Send restore if not already registered.
                        if self.property.fullscreen {
                            self.property.fullscreen = false;
                            return KEvent::Window(KEventWindow::Restored());
                        }
                    }

                    KEvent::None 
                },
                SelectionClear=> { debug_println!("KWindow({:p}), SelectionClear({})", self, xevent._type); KEvent::Unknown },
                SelectionRequest=> { debug_println!("KWindow({:p}), SelectionRequest({})", self, xevent._type); KEvent::Unknown },
                SelectionNotify=> { debug_println!("KWindow({:p}), SelectionNotify({})", self, xevent._type); KEvent::Unknown },
                ColormapNotify=> { debug_println!("KWindow({:p}), ColormapNotify({})", self, xevent._type); KEvent::Unknown },
                ClientMessage=> { debug_println!("KWindow({:p}), ClientMessage({})", self, xevent._type); KEvent::Unknown },
                MappingNotify=> { debug_println!("KWindow({:p}), MappingNotify({})", self, xevent._type); KEvent::Unknown },
                GenericEvent=> { debug_println!("KWindow({:p}), GenericEvent({})", self, xevent._type); KEvent::Unknown },
                _ => { debug_println!("KWindow({:p}), _({})", self, xevent._type); KEvent::Unknown },
            }
        }
    }

    /// Get the real, translated position of KWindow.
    /// 
    /// Reference(s)
    /// <https://stackoverflow.com/questions/3806872/window-position-in-xlib>
    pub(super) fn get_x11_window_position(display : *mut Display, window: *mut Window) -> (i32, i32){
        unsafe {
            let mut x : c_int = 0;
            let mut y : c_int = 0;
            let mut child : Window = 0;
            
            XTranslateCoordinates(display, window, 
                XDefaultRootWindow(display), 0, 0, &mut x, &mut y, &mut child );
            let xwa = Self::get_x11_window_attributes(display, window);
            (x - xwa.x, y - xwa.y )
        }
    }

    /// Get the XWindowAttributes from display connection and window handle.
    fn get_x11_window_attributes(display : *mut Display, window: *mut Window) -> XWindowAttributes {
        unsafe {
            let mut xwa = XWindowAttributes::empty();
            XGetWindowAttributes( display, window, &mut xwa );
            debug_println!("XWA={:?}", xwa);
            xwa
        }
    }


    /// Get X11 window state as a triplet booleans (minimized, maximized, fullscreen).
    /// 
    /// This function query XGetWindowProperty() to get Atoms used to identify those properties.
    #[inline(always)]
    fn get_x11_window_states(&self) -> (bool,bool,bool){
        unsafe {
            // State values returned
            let mut hidden : bool = false;
            let mut maximized = false;
            let mut fullscreen = false;

            // Used to capture XGetWindowProperty
            let mut actual_type_return : Atom = 0;
            let mut actual_format_return : c_int = 0; 
            let mut nitems_return : c_ulong = 0; 
            let mut bytes_after_return : c_ulong = 0; 
            let mut prop_return : *mut c_char = null_mut();

            XGetWindowProperty(self.display_server.display, self.display_server.window, self.display_server.x11_property._NET_WM_STATE, 
                0, 1024, false, self.display_server.x11_property.xa_atom, &mut actual_type_return, &mut actual_format_return, 
                &mut nitems_return, &mut bytes_after_return, &mut prop_return);
            
            // Only query if count of items > 0
            if nitems_return > 0 {
                // Converting according to actual_format_return
                match actual_format_return {
                    // 8 bits
                    8 => {
                        // Convert properties to u8
                        let states: &mut [u8] = core::slice::from_raw_parts_mut(prop_return as *mut u8, nitems_return as usize);
                        for state in states{
                            match *state as Atom {
                                state if self.display_server.x11_property._NET_WM_STATE_HIDDEN == state => {
                                    hidden = true;
                                },
                                state if self.display_server.x11_property._NET_WM_STATE_FULLSCREEN == state => {
                                    fullscreen = true;
                                },
                                state if self.display_server.x11_property._NET_WM_STATE_MAXIMIZED_HORZ == state => {
                                    maximized = true;
                                },
                                state if self.display_server.x11_property._NET_WM_STATE_MAXIMIZED_VERT == state => {
                                    maximized = true;
                                },
                                0 => {},   // Do nothing with 0 atoms
                                _ => {},
                            }
                        }
                        
                    },
                    // 16 bits
                    16 => {
                        // Convert properties to u16
                        let states: &mut [u16] = core::slice::from_raw_parts_mut(prop_return as *mut u16, nitems_return as usize);
                        for state in states{
                            match *state as Atom {
                                state if self.display_server.x11_property._NET_WM_STATE_HIDDEN == state => {
                                    hidden = true;
                                },
                                state if self.display_server.x11_property._NET_WM_STATE_FULLSCREEN == state => {
                                    fullscreen = true;
                                },
                                state if self.display_server.x11_property._NET_WM_STATE_MAXIMIZED_HORZ == state => {
                                    maximized = true;
                                },
                                state if self.display_server.x11_property._NET_WM_STATE_MAXIMIZED_VERT == state => {
                                    maximized = true;
                                },
                                0 => {},   // Do nothing with 0 atoms
                                _ => {},
                            }
                        }
                    },

                    // 32 bits
                    32 => {
                        // Convert properties to u32
                        let states: &mut [u32] = core::slice::from_raw_parts_mut(prop_return as *mut u32, nitems_return as usize);
                        debug_println!("States={:?}", states);
                        for state in states{
                            match *state as Atom {
                                state if self.display_server.x11_property._NET_WM_STATE_HIDDEN == state => {
                                    hidden = true;
                                },
                                state if self.display_server.x11_property._NET_WM_STATE_FULLSCREEN == state => {
                                    fullscreen = true;
                                },
                                state if self.display_server.x11_property._NET_WM_STATE_MAXIMIZED_HORZ == state => {
                                    maximized = true;
                                },
                                state if self.display_server.x11_property._NET_WM_STATE_MAXIMIZED_VERT == state => {
                                    maximized = true;
                                },
                                0 => {},   // Do nothing with 0 atoms
                                // Print unknown state name
                                state => { 
                                    debug_println!("State={:?}", CStr::from_ptr(XGetAtomName(self.display_server.display, state)).to_str().unwrap());
                                }
                            }
                        }
                    },

                    // Anything else is an error
                    _ => panic!("Wrong `actual_format_return` format size!"),
                }
            }

            // Free data returned.
            XFree(prop_return);

            // Return state
            (hidden, maximized, fullscreen)
        }
    }
    
}