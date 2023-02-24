use std::os::raw::{ c_int, c_long, c_uint, c_ulong, c_char, c_uchar, c_short, c_void };
use std::ptr::null_mut;
use std::{panic::catch_unwind};
use crate::kleio::display::linux::x11::bind::XGetAtomName;
use crate::kleio::display::{KWindow, KCursorMode};
use crate::kleio::display::event::KEventWindow;
use crate::kleio::display::linux::x11::constant::GrabModeAsync;
use crate::kleio::display::{ event::KEventDispatcher, event::KEventDispatcherError, KWindowError};
use crate::kleio::display::{linux::x11::{bind::{XDefaultRootWindow, XCreateSimpleWindow, XMapWindow, XSelectInput, XSync, XEventsQueued}, 
    constant::{KeyPressMask, ButtonPressMask, ExposureMask, KeyPress, KeyRelease, ButtonPress, MotionNotify, LeaveNotify, 
    ButtonRelease, EnterNotify, FocusIn, FocusOut, KeymapNotify, Expose, GraphicsExpose, NoExpose, VisibilityNotify, 
    CreateNotify, DestroyNotify, UnmapNotify, MapNotify, MapRequest, ReparentNotify, ConfigureNotify, ConfigureRequest, 
    GravityNotify, ResizeRequest, CirculateNotify, CirculateRequest, PropertyNotify, SelectionClear, SelectionRequest, SelectionNotify, 
    ColormapNotify, ClientMessage, MappingNotify, GenericEvent}}, event::KEvent, self, event::KEventMouse, event::KEventKeyboard};

use self::bind::{XWarpPointer, XFixesHideCursor, XGrabPointer, XFixesShowCursor, XUngrabPointer, XGetWindowProperty};
use self::constant::CurrentTime;
use self::event::Atom;
use self::{event::{ XEvent }, bind::{XOpenDisplay, XCloseDisplay, XNextEvent}, constant::{KeyReleaseMask, ButtonReleaseMask, LeaveWindowMask, EnterWindowMask, Button1MotionMask, PointerMotionMask, Button3MotionMask, Button2MotionMask, Button5MotionMask, Button4MotionMask, ButtonMotionMask, StructureNotifyMask, ResizeRedirectMask, VisibilityChangeMask, FocusChangeMask, PropertyChangeMask}};

use super::server::{ Display, Window };

/// Contains X11 contants definition
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
pub mod constant;

/// Contains X11 Event definition
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_snake_case)]            // Imported C members aren't formatted according to convention.
pub mod event;

/// Contains X11 C functions Bind
pub mod bind;

/// Contains X11 screen fetch function
pub mod screen;

/// Implementation of privates elements relatives to X11 display server
#[doc(hidden)]
impl KWindow {

    // Get cursor position
    #[inline(always)]
    pub(super) fn x11_get_cursor_position(&self) -> (i32, i32){
        todo!()
    }


    

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
        }
    }

    /// Show system default cursor.
    #[inline(always)]
    pub fn x11_show_cursor(&mut self) {
        unsafe {
            XFixesShowCursor(self.display_server.display, self.display_server.window);
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
    pub(super) fn create_x11_window(width:u32, height:u32) -> (*mut Display, *mut Window) {
        unsafe {
            // Create display connection
            let display = XOpenDisplay(std::ptr::null());

            // Get windows 
            
            // Create window
            let window = XCreateSimpleWindow(display, XDefaultRootWindow(display), 0,0,
                    width as u32, height as u32, 4, 0, 0);

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

            
            // Return display and window pointer
            (display, window)
        }
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


                ButtonPress=> { println!("KWindow({:p}), ButtonPress({})", self, xevent._type); KEvent::Unknown },
                ButtonRelease=> { println!("KWindow({:p}), ButtonRelease({})", self, xevent._type); KEvent::Unknown },

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

                KeymapNotify=> { println!("KWindow({:p}), KeymapNotify({})", self, xevent._type); KEvent::Unknown },
                Expose=> { println!("KWindow({:p}), Expose({})", self, xevent._type); KEvent::Unknown },
                GraphicsExpose=> { println!("KWindow({:p}), GraphicsExpose({})", self, xevent._type); KEvent::Unknown },
                NoExpose=> { println!("KWindow({:p}), NoExpose({})", self, xevent._type); KEvent::Unknown },
                VisibilityNotify=> { println!("KWindow({:p}), VisibilityNotify({})", self, xevent._type); KEvent::Unknown },
                CreateNotify=> { println!("KWindow({:p}), CreateNotify({})", self, xevent._type); KEvent::Unknown },
                DestroyNotify=> { println!("KWindow({:p}), DestroyNotify({})", self, xevent._type); KEvent::Unknown },
                UnmapNotify=> { println!("KWindow({:p}), UnmapNotify({})", self, xevent._type); KEvent::Unknown },
                MapNotify=> { println!("KWindow({:p}), MapNotify({})", self, xevent._type); KEvent::Unknown },
                MapRequest=> { println!("KWindow({:p}), MapRequest({})", self, xevent._type); KEvent::Unknown },
                ReparentNotify=> { println!("KWindow({:p}), ReparentNotify({})", self, xevent._type); KEvent::Unknown },

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

                ConfigureRequest=> { println!("KWindow({:p}), ConfigureRequest({})", self, xevent._type); KEvent::Unknown },
                GravityNotify=> { println!("KWindow({:p}), GravityNotify({})", self, xevent._type); KEvent::Unknown },
                //ResizeRequest=> { 
                //    println!("KWindow({:p}), ResizeRequest({})", self, xevent._type); 
                //    KEvent::Window(KEventWindow::Resized((xevent._xresizerequest._width.try_into().unwrap(), xevent._xresizerequest._width.try_into().unwrap())))
                //},
                CirculateNotify=> { println!("KWindow({:p}), CirculateNotify({})", self, xevent._type); KEvent::Unknown },
                CirculateRequest=> { println!("KWindow({:p}), CirculateRequest({})", self, xevent._type); KEvent::Unknown },
                PropertyNotify=> { 
                    // Window state changed
                    if xevent._xproperty._atom == self.display_server.x11_property.wm_state {
                        let property : Atom = self.display_server.x11_property.wm_state; 
                        let long_offset : c_long = 0; 
                        let long_length : c_long = 1024; 
                        let delete : bool = false; 
                        let req_type : Atom = self.display_server.x11_property.xa_atom; 
                        let mut actual_type_return : Atom = 0;
                        let mut actual_format_return : c_int = 0; 
                        let mut nitems_return : c_ulong = 0; 
                        let mut bytes_after_return : c_ulong = 0; 
                        let mut prop_return : *mut c_char = null_mut();

                        let res = XGetWindowProperty(self.display_server.display, self.display_server.window, property, 
                            long_offset, long_length, delete, req_type, &mut actual_type_return, &mut actual_format_return, 
                            &mut nitems_return, &mut bytes_after_return, &mut prop_return);
                            
                        println!("WMSTATE RES={}, actual_type_return={}, actual_format_return={}, nitems_return={}, bytes_after_return={}, prop_return={:?}", 
                            res, actual_type_return, actual_format_return, nitems_return, bytes_after_return, prop_return);

                        // TODO:Call XFREE
                        //XFree(atoms);
                    }

                    //let a = CStr::from_ptr(XGetAtomName(self.display_server.display ,xevent._xproperty._atom));
                    //println!("Atom {}={:?}", xevent._xproperty._atom, a);

                    //println!("_xproperty = {:?}", xevent._xproperty);
                    //println!("KWindow({:p}), PropertyNotify({}, atom={})", self, xevent._type, xevent._xproperty._atom); 
                    KEvent::Unknown 
                },
                SelectionClear=> { println!("KWindow({:p}), SelectionClear({})", self, xevent._type); KEvent::Unknown },
                SelectionRequest=> { println!("KWindow({:p}), SelectionRequest({})", self, xevent._type); KEvent::Unknown },
                SelectionNotify=> { println!("KWindow({:p}), SelectionNotify({})", self, xevent._type); KEvent::Unknown },
                ColormapNotify=> { println!("KWindow({:p}), ColormapNotify({})", self, xevent._type); KEvent::Unknown },
                ClientMessage=> { println!("KWindow({:p}), ClientMessage({})", self, xevent._type); KEvent::Unknown },
                MappingNotify=> { println!("KWindow({:p}), MappingNotify({})", self, xevent._type); KEvent::Unknown },
                GenericEvent=> { println!("KWindow({:p}), GenericEvent({})", self, xevent._type); KEvent::Unknown },
                _ => { println!("KWindow({:p}), _({})", self, xevent._type); KEvent::Unknown },
            }
        }
    }
    
}



/*
/// # X11 display server backend
pub struct X11DisplayServer {
    /// Used to fetch events
    event : XEvent,

    /// X11 Display pointer
    display : *mut Display,

    /// X11 Window pointer
    window : *mut Window,
}



impl KLinuxDisplayServer for X11DisplayServer {
    

    
    fn pop_event(&mut self) -> KEvent {
        self.fetch_event()
    }

    fn get_display_server_provider(&self) -> LinuxDisplayServerProvider {
        LinuxDisplayServerProvider::X11
    }

    fn get_event_count(&self) -> usize {
        unsafe {
            XEventsQueued(self.display, 0).try_into().unwrap()
        }   
    }

    fn sync_events(&self) {
        
    }

    fn get_display_server_connection(&self) -> *const Display {
        self.display
    }

    fn get_display_server_window(&self) -> *const Window {
        self.window
    }

    fn set_cursor_position(&mut self, position : (i32, i32), size : (u32, u32)) {
        
    }

    fn get_cursor_position(&self) -> (i32, i32) {
        todo!()
    }


}

/// Private event fetch function
impl X11DisplayServer {
   
    fn fetch_event(&mut self) -> KEvent {
        
}

impl KLinuxDisplayServer for X11DisplayServer {
    fn get_display_server_provider(&self) -> LinuxDisplayServerProvider{
        LinuxDisplayServerProvider::X11
    }

    fn is_compatible() -> bool {
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
    

    #[allow(non_upper_case_globals)]            // Imported C members aren't formatted according to convention.
    fn poll_event(&mut self) -> KEvent {
        unsafe {
            XNextEvent(self.display, &mut self.event);            
            
            match self.event._type {
                KeyPress => { println!("KWindow({:p}), KeyPress({})", self, self.event._type); 
                    KEvent::Keyboard(KEventKeyboard::KeyDown(self.event._xkey._keycode)) },
                KeyRelease=> { println!("KWindow({:p}), KeyRelease({})", self, self.event._type); KEvent::Unknown },
                ButtonPress=> { println!("KWindow({:p}), ButtonPress({})", self, self.event._type); KEvent::Unknown },
                ButtonRelease=> { println!("KWindow({:p}), ButtonRelease({})", self, self.event._type); KEvent::Unknown },
                MotionNotify=> {
                    KEvent::Mouse(KEventMouse::Moved((self.event._xmotion._x, self.event._xmotion._y)))
                },
                EnterNotify=> { println!("KWindow({:p}), EnterNotify({})", self, self.event._type); KEvent::Unknown },
                LeaveNotify=> { println!("KWindow({:p}), LeaveNotify({})", self, self.event._type); KEvent::Unknown },
                FocusIn=> KEvent::Window(window::KEventWindow::Focus()),
                FocusOut=> KEvent::Window(window::KEventWindow::Blur()),
                KeymapNotify=> { println!("KWindow({:p}), KeymapNotify({})", self, self.event._type); KEvent::Unknown },
                Expose=> { println!("KWindow({:p}), Expose({})", self, self.event._type); KEvent::Unknown },
                GraphicsExpose=> { println!("KWindow({:p}), GraphicsExpose({})", self, self.event._type); KEvent::Unknown },
                NoExpose=> { println!("KWindow({:p}), NoExpose({})", self, self.event._type); KEvent::Unknown },
                VisibilityNotify=> { println!("KWindow({:p}), VisibilityNotify({})", self, self.event._type); KEvent::Unknown },
                CreateNotify=> { println!("KWindow({:p}), CreateNotify({})", self, self.event._type); KEvent::Unknown },
                DestroyNotify=> { println!("KWindow({:p}), DestroyNotify({})", self, self.event._type); KEvent::Unknown },
                UnmapNotify=> { println!("KWindow({:p}), UnmapNotify({})", self, self.event._type); KEvent::Unknown },
                MapNotify=> { println!("KWindow({:p}), MapNotify({})", self, self.event._type); KEvent::Unknown },
                MapRequest=> { println!("KWindow({:p}), MapRequest({})", self, self.event._type); KEvent::Unknown },
                ReparentNotify=> { println!("KWindow({:p}), ReparentNotify({})", self, self.event._type); KEvent::Unknown },
                ConfigureNotify=> { println!("KWindow({:p}), ConfigureNotify({})", self, self.event._type); KEvent::Unknown },
                ConfigureRequest=> { println!("KWindow({:p}), ConfigureRequest({})", self, self.event._type); KEvent::Unknown },
                GravityNotify=> { println!("KWindow({:p}), GravityNotify({})", self, self.event._type); KEvent::Unknown },
                ResizeRequest=> { println!("KWindow({:p}), ResizeRequest({})", self, self.event._type); KEvent::Unknown },
                CirculateNotify=> { println!("KWindow({:p}), CirculateNotify({})", self, self.event._type); KEvent::Unknown },
                CirculateRequest=> { println!("KWindow({:p}), CirculateRequest({})", self, self.event._type); KEvent::Unknown },
                PropertyNotify=> { println!("KWindow({:p}), PropertyNotify({})", self, self.event._type); KEvent::Unknown },
                SelectionClear=> { println!("KWindow({:p}), SelectionClear({})", self, self.event._type); KEvent::Unknown },
                SelectionRequest=> { println!("KWindow({:p}), SelectionRequest({})", self, self.event._type); KEvent::Unknown },
                SelectionNotify=> { println!("KWindow({:p}), SelectionNotify({})", self, self.event._type); KEvent::Unknown },
                ColormapNotify=> { println!("KWindow({:p}), ColormapNotify({})", self, self.event._type); KEvent::Unknown },
                ClientMessage=> { println!("KWindow({:p}), ClientMessage({})", self, self.event._type); KEvent::Unknown },
                MappingNotify=> { println!("KWindow({:p}), MappingNotify({})", self, self.event._type); KEvent::Unknown },
                GenericEvent=> { println!("KWindow({:p}), GenericEvent({})", self, self.event._type); KEvent::Unknown },
                _ => { println!("KWindow({:p}), _({})", self, self.event._type); KEvent::Unknown },
            }
        }
    }


    fn get_event_count(&self) -> usize {
        unsafe {
            XEventsQueued(self.display, 0).try_into().unwrap()
        }   
    }

    fn sync_event(&self) {
        unsafe {
            // The XSync() function flushes the output buffer and then waits until all requests have been received and processed by the X server.
            XSync(self.display, false);
        }
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

    fn show_cursor(&self,keep_inside_window : bool) {
        todo!()
    }

    fn hide_cursor(&self) {
        todo!()
    }

    fn set_cursor_position(&self, position : (i32, i32)) {
        todo!()
    }

    fn is_cursor_hidden(&self) {
        todo!()
    }

    fn set_motion_mode(&self, mode : window::KWindowMotionMode) {
        todo!()
    }

    fn get_motion_mode(&self) -> window::KWindowMotionMode {
        todo!()
    }

    fn get_display_server_connection(&self) -> *const Display {
        todo!()
    }

    fn get_display_server_window(&self) -> *const Window {
        todo!()
    }

    
}
*/

