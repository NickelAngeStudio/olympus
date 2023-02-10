use std::{panic::catch_unwind};

use crate::kleio::window::{linux::x11::{bind::{XDefaultRootWindow, XCreateSimpleWindow, XMapWindow, XSelectInput, XSync, XEventsQueued}, constant::{KeyPressMask, ButtonPressMask, ExposureMask, KeyPress, KeyRelease, ButtonPress, MotionNotify, LeaveNotify, ButtonRelease, EnterNotify, FocusIn, FocusOut, KeymapNotify, Expose, GraphicsExpose, NoExpose, VisibilityNotify, CreateNotify, DestroyNotify, UnmapNotify, MapNotify, MapRequest, ReparentNotify, ConfigureNotify, ConfigureRequest, GravityNotify, ResizeRequest, CirculateNotify, CirculateRequest, PropertyNotify, SelectionClear, SelectionRequest, SelectionNotify, ColormapNotify, ClientMessage, MappingNotify, GenericEvent}}, event::KEvent, self, KWindowManager, KWindowManagerId, KEventMouse, KEventKeyboard};

use self::{event::{ Display, Window, XEvent }, bind::{XOpenDisplay, XCloseDisplay, XNextEvent}, constant::{KeyReleaseMask, ButtonReleaseMask, LeaveWindowMask, EnterWindowMask, Button1MotionMask, PointerMotionMask, Button3MotionMask, Button2MotionMask, Button5MotionMask, Button4MotionMask, ButtonMotionMask, StructureNotifyMask, ResizeRedirectMask, VisibilityChangeMask, FocusChangeMask, PropertyChangeMask}};

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


/// # X11 KWindowManager backend
pub struct KWindowManagerX11 {
    /// Used to fetch events
    event : XEvent,

    /// X11 Display pointer
    display : *mut Display,

    /// X11 Window pointer
    window : *mut Window,
}

/// [KWindowManagerX11] members.
impl KWindowManagerX11 {

    /// Verify if system is compatible with X11 display server.
    /// 
    /// Returns True if compatible, false otherwise.
    pub(crate) fn is_compatible() -> bool {
        unsafe {
            // Try to call C function with error handling.
            let result = catch_unwind(|| {
                XOpenDisplay(std::ptr::null())
            }); 

            match result {
                Ok(display) => {
                    println!("DisplayIC={:?}", display);
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

    /// Returns X11 display pointer.
    pub fn get_display(&self) -> *mut Display {
        self.display
    }
}

impl KWindowManager for KWindowManagerX11 {
    fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Self where Self: Sized {
        unsafe {
            // Create display connection
            let display = XOpenDisplay(std::ptr::null());
            println!("Display={:?}", display);
            
            // Create window
            let window = XCreateSimpleWindow(display, XDefaultRootWindow(display), pos_x as i32, pos_y as i32,
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
            KWindowManagerX11 { event : XEvent { _type: 0}, display : display, window : window }
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
                MotionNotify=> KEvent::Mouse(KEventMouse::Moved((self.event._xmotion._x, self.event._xmotion._y), (self.event._xmotion._x, self.event._xmotion._y))),
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
            
            


            /*
            if XEventsQueued(self.display, 0) > 0 {
                

                KEvent::Unknown
            } else {
                // Perform an XSync when no event queued
                

                // Return KEvent::None
                KEvent::Unknown
            }
            */
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_id(&self) -> u8 {
        KWindowManagerId::X11
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

    fn show_cursor(&self) {
        todo!()
    }

    fn hide_cursor(&self) {
        todo!()
    }
}


impl Drop for KWindowManagerX11 {
    /// KWindowX11 destructor. Will disconnect display server.
    fn drop(&mut self) {
        unsafe {
            // Close display server connection.
            XCloseDisplay(self.display);
        }
    }
}