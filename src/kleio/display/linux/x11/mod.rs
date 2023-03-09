use std::thread;
use std::ffi::{CStr, CString};
use std::os::raw::{ c_int, c_long, c_uint, c_ulong, c_char, c_uchar, c_short, c_void };
use std::ptr::null_mut;
use std::{panic::catch_unwind};
use debug_print::debug_println;


use crate::error::OlympusError;
use crate::kleio::display::linux::x11::bind::{XFree, XGetAtomName, XFlush, XScreenOfDisplay};
use crate::kleio::display::screen::KScreenList;
use crate::kleio::display::{KWindow, KCursorMode, KWindowFullscreenMode, KWindowManager, KWindowProvider, KWindowProperty};
use crate::kleio::display::event::KEventWindow;
use crate::kleio::display::linux::x11::constant::{GrabModeAsync};
use crate::kleio::display::{linux::x11::{bind::{XDefaultRootWindow, XCreateSimpleWindow, XMapWindow, XSelectInput, XSync, XEventsQueued}, 
    constant::{KeyPressMask, ButtonPressMask, ExposureMask, KeyPress, KeyRelease, ButtonPress, MotionNotify, LeaveNotify, 
    ButtonRelease, EnterNotify, FocusIn, FocusOut, KeymapNotify, Expose, GraphicsExpose, NoExpose, VisibilityNotify, 
    CreateNotify, DestroyNotify, UnmapNotify, MapNotify, MapRequest, ReparentNotify, ConfigureNotify, ConfigureRequest, 
    GravityNotify, CirculateNotify, CirculateRequest, PropertyNotify, SelectionClear, SelectionRequest, SelectionNotify, 
    ColormapNotify, ClientMessage, MappingNotify, GenericEvent}}, event::KEvent, event::KEventMouse, event::KEventKeyboard};

use self::atom::X11Atoms;
use self::attributes::{XWindowAttributes};
use self::bind::{XWarpPointer, XFixesHideCursor, XGrabPointer, XFixesShowCursor, XUngrabPointer, XGetWindowProperty, XStoreName, 
    XChangeProperty, XGetWindowAttributes, XTranslateCoordinates, 
    XResizeWindow, XMoveWindow, XDestroyWindow};
use self::constant::{CurrentTime, VisibilityUnobscured, PropModeReplace};
use self::event::{Atom, Display, Window, XEvent};
use self::{ bind::{XOpenDisplay, XCloseDisplay, XNextEvent}, constant::{KeyReleaseMask, ButtonReleaseMask, LeaveWindowMask, EnterWindowMask, Button1MotionMask, PointerMotionMask, Button3MotionMask, Button2MotionMask, Button5MotionMask, Button4MotionMask, ButtonMotionMask, StructureNotifyMask, ResizeRedirectMask, VisibilityChangeMask, FocusChangeMask, PropertyChangeMask}};

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

/// Contains X11 atoms
pub mod atom;

/// Event mask used with x11 to capture and dispatch event.
const EVENT_MASK : i64 =    KeyPressMask | KeyReleaseMask |             // Keyboard Button Down and Up
                            ButtonPressMask | ButtonReleaseMask |       // Controller button??? TBD 
                            EnterWindowMask | LeaveWindowMask |         // Window focus, blur
                            PointerMotionMask | Button1MotionMask | 
                            Button2MotionMask | Button3MotionMask |
                            Button4MotionMask | Button5MotionMask |
                            ButtonMotionMask |                          // Mouse motion??? TBD
                            StructureNotifyMask |                       // ResizeRedirectMask |
                            VisibilityChangeMask | FocusChangeMask |
                            PropertyChangeMask | ExposureMask;          // Window event I guess??



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

/// [X Window System](https://en.wikipedia.org/wiki/X_Window_System) implementation of [KWindowManager].
pub struct KWindowManagerX11 {

    /// Used to fetch X11 events
    x_event : XEvent,    

    /// C-compatible string for window title
    wm_title : CString,

    /// Display connection pointer
    display : *mut Display,

    /// Window handle pointer
    window : *mut Window,

    /// Atoms for handling x11 window properties
    atoms : X11Atoms,

    /// Flag used to make sure XHideCursor was called prior to XShowCursor to prevent crash
    x_hide_cursor_flag : bool,

    /// Position and size for restoring window.
    restoration_position_size : ((i32,i32),(u32,u32)),

    /// Window properties.
    property : KWindowProperty,
}

/// Public members of [KWindowManagerX11].
impl KWindowManagerX11 {
    /// Create a new instance of KWindowManagerX11.
    pub(crate) fn new(width:u32, height:u32) -> KWindowManagerX11 {
        unsafe {
        // Create display connection
        let display = XOpenDisplay(std::ptr::null());


        }

    }

    /// Return True if X Window System is supported. False otherwise.
    /// 
    /// Test is done in another thread to prevent main thread panic.
    pub fn is_supported() -> bool {
        unsafe {
            let thread_join_handle = thread::spawn(move || {
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
            });

            match thread_join_handle.join() {
                Ok(value) => value,
                Err(_) => {
                    // Not supported
                    false
                },
            }
        }
    }

    /// Get the X system display connection.
    pub fn get_display_server_connection(&self) -> *const Display {
        self.display
    }

    /// Get the X system window handle.
    pub fn get_window_handle(&self) -> *const Window {
        self.window
    }
}

/// [Drop] trait implementation for [KWindowManagerX11].
impl Drop for KWindowManagerX11 {
    fn drop(&mut self) {
        unsafe {
            // Close display server connection.
            XCloseDisplay(self.display);
        }
    }
}

/// [KWindowManager] trait implementation for [KWindowManagerX11].
impl KWindowManager for KWindowManagerX11 {
    fn get_window_provider(&self) -> KWindowProvider {
        KWindowProvider::X11
    }

    fn get_window_property(&self) -> &KWindowProperty {
        &self.property
    }

    fn set_cursor_mode(&mut self, mode : crate::kleio::display::cursor::KCursorMode)  {
         self.property.cursor.mode = mode;

         match mode {
            // Set cursor to center if Acceleration
            crate::kleio::display::cursor::KCursorMode::Acceleration => self.set_cursor_position(self.property.center),
            _ => todo!(),
        }
    }

    fn get_event_count(&self) -> usize {
        unsafe {
            XEventsQueued(self.display, 0).try_into().unwrap()
        }   
    }

    fn set_cursor_position(&mut self, position : (i32, i32)) {
        unsafe {
            XWarpPointer(self.display, self.window, self.window, 0, 0, 
                0, 0, position.0,  position.1);
        }
    }

    fn hide_cursor(&mut self) {
        unsafe {
            XFixesHideCursor(self.display, self.window);
            self.x_hide_cursor_flag = true;
        }
    }

    fn show_cursor(&mut self)  {
        unsafe {
            if self.x_hide_cursor_flag {    // Make sure X hide cursor was called prior to show.
                XFixesShowCursor(self.display, self.window);
                self.x_hide_cursor_flag = false;
            }       
        }
    }

    fn confine_cursor(&mut self) {
        unsafe {
            XGrabPointer(self.display, self.window, true, 
            0, GrabModeAsync.try_into().unwrap(), GrabModeAsync.try_into().unwrap(), self.window, 0, CurrentTime);
        }
    }

    fn release_cursor(&mut self) {
        unsafe {
            XUngrabPointer(self.display, CurrentTime);
        }
    }

    fn restore(&mut self) {
        unsafe {
            let states = self.get_x11_window_states_event();

            // Destroy current window
            XDestroyWindow(self.display, self.window);

            // Recreate window as normal
            self.window = KWindowManagerX11::create_x11_window(self.display, XDefaultRootWindow(self.display), &self.atoms, self.restoration_position_size.0,
                self.restoration_position_size.1, false);   

            self.set_position(self.restoration_position_size.0);
        }        
    }

    fn set_title(&mut self, title : &str) {
        unsafe {
            self.wm_title = CString::from_vec_unchecked(title.as_bytes().to_vec());
            XStoreName(self.display, self.window, self.wm_title.as_ptr() as *mut i8);
        }
    }

    fn set_size(&mut self, size : (u32, u32)) {
        unsafe {
            // Keep real window position
            self.restoration_position_size.0 = KWindowManagerX11::get_x11_window_position(self.display, self.window);

            XResizeWindow(self.display, self.window, size.0, size.1);
            
            // Reposition window since resize put it back at 0,0
            self.set_position(self.restoration_position_size.0);
        }
    }

    fn set_position(&mut self, position : (i32, i32)) {
        unsafe {
            XMoveWindow(self.display, self.window, position.0, position.1);
        }
    }

    fn set_fullscreen(&mut self, fs_mode : KWindowFullscreenMode) {
        unsafe {

            if !self.property.is_fullscreen {
                // Save windowed properties for restoration.
                self.restoration_position_size = (KWindowManagerX11::get_x11_window_position(self.display, self.window), self.property.size);
            }

            // Destroy current window
            XDestroyWindow(self.display, self.window);

            match fs_mode {
                KWindowFullscreenMode::CurrentScreen => {
                    // Recreate window as fullscreen
                    self.window = KWindowManagerX11::create_x11_window(self.display, XDefaultRootWindow(self.display),
                     &self.atoms, (0,0),   self.get_primary_screen().unwrap().get_current_resolution(), true);      
                },
                KWindowFullscreenMode::PrimaryScreen => {

                },
                KWindowFullscreenMode::DesktopScreen => {

                },
            }

            // Write stored title
            XStoreName(self.display, self.window, self.wm_title.as_ptr() as *mut i8);
        }
    }

    fn sync(&self) {
        unsafe {
            XSync(self.display, false);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        &mut self
    }

    fn poll_event(&mut self) -> KEvent {
        unsafe {

            XNextEvent(self.display, &mut self.x_event);
            let xevent = self.x_event; 
            
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
                        KCursorMode::Acceleration => {
                            let position = (xevent._xmotion._x - self.property.center.0, 
                                xevent._xmotion._y - self.property.center.1);
                            // Report acceleration only if movement occurred
                            if position.0 != 0 || position.1 != 0 {
                                KEvent::Mouse(KEventMouse::Moved(position))
                            } else {
                                KEvent::None
                            }
                        }
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
                ConfigureNotify=> { self.get_window_configuration_event() },

                ConfigureRequest=> { debug_println!("KWindow({:p}), ConfigureRequest({})", self, xevent._type); KEvent::Unknown },
                GravityNotify=> { debug_println!("KWindow({:p}), GravityNotify({})", self, xevent._type); KEvent::Unknown },

                CirculateNotify=> { debug_println!("KWindow({:p}), CirculateNotify({})", self, xevent._type); KEvent::Unknown },
                CirculateRequest=> { debug_println!("KWindow({:p}), CirculateRequest({})", self, xevent._type); KEvent::Unknown },
                PropertyNotify=> { self.get_x11_window_states_event() },
                    
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

    
}


/// Private members of [KWindowManagerX11].
impl KWindowManagerX11 {
    /// Create x11 Window according to position, size and if fullscreen or not.
    #[inline(always)]
    pub fn create_x11_window(display : *mut Display, root : *mut Window, atoms : &X11Atoms, position : (i32, i32), 
        size : (u32,u32), fullscreen : bool) -> *mut Window {
        unsafe {
            let window = XCreateSimpleWindow(display, root, position.0,position.1,
                    size.0, size.1, 4, 0, 0);

            // Set window Type to normal
            x11_change_property!(display, window, atoms, _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL);

            // Allowed actions
            x11_change_property!(display, window, atoms, _NET_WM_ALLOWED_ACTIONS, _NET_WM_ACTION_FULLSCREEN, _NET_WM_ACTION_MINIMIZE, _NET_WM_ACTION_CHANGE_DESKTOP,
                _NET_WM_ACTION_CLOSE, _NET_WM_ACTION_ABOVE, _NET_WM_ACTION_BELOW);

            if fullscreen {
                // Set as fullscreen
                 x11_change_property!(display, window, atoms, _NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_FULLSCREEN);
            }

            // Map window to display
            XMapWindow(display, window);

            // Mask of events to receive
            XSelectInput(display, window, EVENT_MASK);

            // Flush buffer
            XFlush(display);
            
            // Return window pointer
            window
        }
    }

    /// Get the real, translated position of KWindow.
    /// 
    /// Reference(s)
    /// <https://stackoverflow.com/questions/3806872/window-position-in-xlib>
    pub fn get_x11_window_position(display : *mut Display, window: *mut Window) -> (i32, i32){
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
            xwa
        }
    }

    /// Get and event from the X11 window configuration state.
    fn get_window_configuration_event(&mut self) -> KEvent {
        let position = (self.x_event._xconfigure._x, self.x_event._xconfigure._y);
        let size = (self.x_event._xconfigure._width as u32, self.x_event._xconfigure._height as u32);

        // By default, set event as none.
        let mut event = KEvent::None;

        if position != self.property.position && size != self.property.size {
            event = KEvent::Window(KEventWindow::MovedResized(position, size));
        } else if position != self.property.position {
            event = KEvent::Window(KEventWindow::Moved(position));
        } else if size != self.property.size  {
            event = KEvent::Window(KEventWindow::Resized(size));
        }

        // Update window properties
        self.property.position = position;
        self.property.size = size;

        event
    }

    /// Get and event from the X11 window state.
    /// 
    /// This function query XGetWindowProperty() to get Atoms used to identify min, max and fullscreen properties.
    #[inline(always)]
    fn get_x11_window_states_event(&mut self) -> KEvent {
        unsafe {
            // State values returned
            let hidden = false;
            let  maximized = false;
            let fullscreen = false;

            // Used to capture XGetWindowProperty
            let mut actual_type_return : Atom = 0;
            let mut actual_format_return : c_int = 0; 
            let mut nitems_return : c_ulong = 0; 
            let mut bytes_after_return : c_ulong = 0; 
            let mut prop_return : *mut c_char = null_mut();

            XGetWindowProperty(self.display, self.window, self.atoms._NET_WM_STATE, 
                0, 1024, false, self.atoms.xa_atom, &mut actual_type_return, &mut actual_format_return, 
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
                                state if self.atoms._NET_WM_STATE_HIDDEN == state => {
                                    hidden = true;
                                },
                                state if self.atoms._NET_WM_STATE_FULLSCREEN == state => {
                                    fullscreen = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_HORZ == state => {
                                    maximized = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_VERT == state => {
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
                                state if self.atoms._NET_WM_STATE_HIDDEN == state => {
                                    hidden = true;
                                },
                                state if self.atoms._NET_WM_STATE_FULLSCREEN == state => {
                                    fullscreen = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_HORZ == state => {
                                    maximized = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_VERT == state => {
                                    maximized = true;
                                },
                                0 => {},   // Do nothing with 0 atoms
                                _ => {},
                            }
                        }
                    },

                    // 32 bits
                    32 => {
                        // Convert properties to Atom
                        let states: &mut [Atom] = core::slice::from_raw_parts_mut(prop_return as *mut Atom, nitems_return as usize);
                        debug_println!("States={:?}", states);
                        
                        for state in states{
                            match *state as Atom {
                                state if self.atoms._NET_WM_STATE_HIDDEN == state => {
                                    hidden = true;
                                },
                                state if self.atoms._NET_WM_STATE_FULLSCREEN == state => {
                                    fullscreen = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_HORZ == state => {
                                    maximized = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_VERT == state => {
                                    maximized = true;
                                },
                                0 => {},   // Do nothing with 0 atoms
                                // Print unknown state name
                                state => { 
                                    debug_println!("State={:?}", CStr::from_ptr(XGetAtomName(self.display, state)).to_str().unwrap());
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

            // Event to return
            let mut event = KEvent::None;

            // Return event. By priority > Fullscreen > Minimized > Maximized > Restored > None
            if fullscreen {   // Send fullscreen if not already registered.
                if !self.property.is_fullscreen {
                    event = KEvent::Window(KEventWindow::Fullscreen());
                }
            } else if hidden {   // Send minimized if not already registered.
                    if !self.property.is_minimized {
                        event = KEvent::Window(KEventWindow::Minimized());
                    }
            } else if maximized {   // Send maximized if not already registered.
                if !self.property.is_maximized {
                    event = KEvent::Window(KEventWindow::Maximized());
                }
            } else {    // Send restore if not already registered.
                if self.property.is_fullscreen != fullscreen || 
                    self.property.is_maximized != maximized || 
                    self.property.is_minimized != hidden {
                        event = KEvent::Window(KEventWindow::Restored());
                    }
            }

            // Update window properties
            self.property.is_fullscreen = fullscreen;
            self.property.is_maximized = maximized;
            self.property.is_minimized = hidden;

            event
        }
    }
    
}

/*
/// Implementation of KWindow elements.
#[doc(hidden)]
impl KWindow {


    

    

    



    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    #[inline(always)]
    pub(super) fn x11_restore(&mut self) {
        

    }

    // Pop an event from the queue
    #[inline(always)]
    #[allow(non_upper_case_globals)]            // Imported C members aren't formatted according to convention.
    pub(super) fn x11_poll_event(&mut self) -> KEvent {
        
    }


    
    
}

/*** PRIVATE ***/
/// Implementation of private / intern x11 functions
impl KWindow {
    

    /// Create x11 Window according to position, size and if fullscreen or not.
    #[inline(always)]
    pub(super) fn create_x11_window(display : *mut Display, root : *mut Window, x11_prop : &KLinuxDisplayServerX11Property, position : (i32, i32), 
        size : (u32,u32), fullscreen : bool) -> *mut Window {
        unsafe {
            let window = XCreateSimpleWindow(display, root, position.0,position.1,
                    size.0, size.1, 4, 0, 0);

            // Set window Type to normal
            x11_change_property!(display, window, x11_prop, _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL);

            // Allowed actions
            x11_change_property!(display, window, x11_prop, _NET_WM_ALLOWED_ACTIONS, _NET_WM_ACTION_FULLSCREEN, _NET_WM_ACTION_MINIMIZE, _NET_WM_ACTION_CHANGE_DESKTOP,
                _NET_WM_ACTION_CLOSE, _NET_WM_ACTION_ABOVE, _NET_WM_ACTION_BELOW);

            if fullscreen {
                // Set as fullscreen
                 x11_change_property!(display, window, x11_prop, _NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_FULLSCREEN);
            }

            // Write stored title
            XStoreName(display, window, x11_prop.wm_title.as_ptr() as *mut i8);

            // Map window to display
            XMapWindow(display, window);

            // Mask of events to receive
            XSelectInput(display, window, EVENT_MASK);

            // Flush buffer
            XFlush(display);
            
            // Return window pointer
            window
        }
    }

    /// Get default root window of display
    pub(super) fn get_x11_default_root_window(display : *mut Display) -> *mut Window {
        unsafe {
            XDefaultRootWindow(display)
        }
    }

    
}
*/