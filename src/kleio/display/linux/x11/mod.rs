use std::{panic::catch_unwind};
use std::os::raw::{ c_int };
use crate::kleio::display::{ KWindowMotionMode, event::KEventDispatcher, event::KEventDispatcherError, KWindowError};
use crate::kleio::display::{linux::x11::{bind::{XDefaultRootWindow, XCreateSimpleWindow, XMapWindow, XSelectInput, XSync, XEventsQueued}, 
    constant::{KeyPressMask, ButtonPressMask, ExposureMask, KeyPress, KeyRelease, ButtonPress, MotionNotify, LeaveNotify, 
    ButtonRelease, EnterNotify, FocusIn, FocusOut, KeymapNotify, Expose, GraphicsExpose, NoExpose, VisibilityNotify, 
    CreateNotify, DestroyNotify, UnmapNotify, MapNotify, MapRequest, ReparentNotify, ConfigureNotify, ConfigureRequest, 
    GravityNotify, ResizeRequest, CirculateNotify, CirculateRequest, PropertyNotify, SelectionClear, SelectionRequest, SelectionNotify, 
    ColormapNotify, ClientMessage, MappingNotify, GenericEvent}}, event::KEvent, self, event::KEventMouse, event::KEventKeyboard};

use self::{event::{ XEvent }, bind::{XOpenDisplay, XCloseDisplay, XNextEvent}, constant::{KeyReleaseMask, ButtonReleaseMask, LeaveWindowMask, EnterWindowMask, Button1MotionMask, PointerMotionMask, Button3MotionMask, Button2MotionMask, Button5MotionMask, Button4MotionMask, ButtonMotionMask, StructureNotifyMask, ResizeRedirectMask, VisibilityChangeMask, FocusChangeMask, PropertyChangeMask}};

use super::{ Display, Window, KLinuxDisplayServer };

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



/// # X11 display server backend
pub struct X11DisplayServer {
    /// Used to fetch events
    event : XEvent,

    /// X11 Display pointer
    display : *mut Display,

    /// X11 Window pointer
    window : *mut Window,
}

/// Public [KWindowX11] members.
impl X11DisplayServer {
    pub fn new(width:usize, height:usize) -> X11DisplayServer {
        unsafe {
            // Create display connection
            let display = XOpenDisplay(std::ptr::null());
            println!("Display={:?}", display);

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
                                StructureNotifyMask | ResizeRedirectMask |
                                VisibilityChangeMask | FocusChangeMask |
                                PropertyChangeMask | ExposureMask;                        // Window event I guess??


            // Mask of events to receive
            XSelectInput(display, window, mask);

            

            // Return KWindowManagerX11
            X11DisplayServer { event : XEvent { _type: 0}, display : display, window : window }
        }
    }

    




}

impl Drop for X11DisplayServer {
    /// KWindowX11 destructor. Will disconnect display server.
    fn drop(&mut self) {
        unsafe {
            // Close display server connection.
            XCloseDisplay(self.display);
        }
    }
}


impl KLinuxDisplayServer for X11DisplayServer {
    fn is_compatible() -> bool where Self:Sized {
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

    fn pop_event(&mut self) -> KEvent {
        todo!()
    }
}


/*
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

