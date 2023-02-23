// Contains bindings for XLib
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};

use super::{ Display, Window };
use super::event::{ XEvent};


#[link(name = "X11")]
extern {
    

    /// The XOpenDisplay function returns a Display structure that serves as the connection to the 
    /// X server and that contains all the information about that X server.
    /// 
    /// # Reference(s)
    /// <https://www.x.org/releases/X11R7.5/doc/man/man3/XOpenDisplay.3.html>
    pub fn XOpenDisplay(display_name : *const c_char) -> *mut Display;


    /// The XCloseDisplay() function closes the connection to the X server for the display specified in the Display structure 
    /// and destroys all windows and resources that the client has created on this display.
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/display/XCloseDisplay.html>
    pub fn XCloseDisplay(display : *mut Display);


    /// The XSync() function flushes the output buffer and then waits until all requests have been received and processed by the X server.
    /// If you passed False, XSync() does not discard the events in the queue. If you passed True, XSync() discards all events in the queue, 
    /// including those events that were on the queue before XSync() was called. Client applications seldom need to call XSync(). 
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/event-handling/XSync.html>
    pub fn XSync(display : *mut Display, discard : bool);


    /// If mode is QueuedAlready(0), XEventsQueued() returns the number of events already in the event queue (and never performs a system call). 
    /// If mode is QueuedAfterFlush(1), XEventsQueued() returns the number of events already in the queue if the number is nonzero.
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/event-handling/XEventsQueued.html>
    pub fn XEventsQueued(display : *mut Display, mode : c_int) -> c_int;

    /// Returns the root window for the default screen. 
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/display/display-macros.html>
    pub fn XDefaultRootWindow(display : *mut Display) -> *mut Window;

    /// The XCreateSimpleWindow function creates an unmapped InputOutput subwindow for a specified parent window, 
    /// returns the window ID of the created window, and causes the X server to generate a CreateNotify event
    /// 
    /// # References(s)
    /// <https://tronche.com/gui/x/xlib/window/XCreateWindow.html>
    pub fn XCreateSimpleWindow(display : *mut Display, parent : *mut Window, x : c_int, y : c_int, 
        width : c_uint, height : c_uint, border_width : c_uint, border : c_ulong, background : c_ulong) -> *mut Window;


    /// The XMapWindow() function maps the window and all of its subwindows that have had map requests.
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/window/XMapWindow.html>
    pub fn XMapWindow(display : *mut Display, w : *mut Window);

    /// The XSelectInput() function requests that the X server report the events associated 
    /// with the specified event mask. Initially, X will not report any of these events.
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/event-handling/XSelectInput.html>
    pub fn XSelectInput(display : *mut Display, w : *mut Window, event_mask: c_long);

    /// The XNextEvent() function copies the first event from the event queue into the specified 
    /// XEvent structure and then removes it from the queue.
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/event-handling/manipulating-event-queue/XNextEvent.html>
    pub fn XNextEvent(display : *mut Display, event_return : *mut XEvent);

    /// Move the pointer to an arbitrary point in a window.
    /// 
    /// Reference(s)
    /// Moving the Pointer : <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html>
    pub fn XWarpPointer(display : *mut Display, src_w : *mut Window, dest_w : *mut Window, 
        src_x : c_int, src_y : c_int, src_width : c_uint, src_height : c_uint, dest_x : c_int,  dest_y : c_int);

    
    /// Confine pointer to X11 window.
    /// 
    /// XGrabPointer can generate BadCursor, BadValue, and BadWindow errors.
    /// 
    /// For cursor, use constant None for default system cursor.
    /// For time, use constant CurrentTime.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XGrabPointer>
    pub fn XGrabPointer(display : *mut Display, grab_window : *mut Window, owner_events : bool, 
        event_mask : c_uint, intpointer_mode : c_uint, keyboard_mode : c_uint, confine_to : *mut Window, cursor : c_ulong, time : c_long) -> c_int;

    /// The XUngrabPointer function releases the pointer and any queued events if this client has actively grabbed the pointer from XGrabPointer.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XUngrabPointer>
    pub fn XUngrabPointer(display : *mut Display, time : c_long);

    
}

// XFixes bindings.
#[link(name = "Xfixes")]
extern {
    /// Hide the KWindow cursor.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/current/doc/fixesproto/fixesproto.txt>
    pub fn XFixesHideCursor(display : *mut Display, window : *mut Window);

    /// Show the KWindow cursor.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/current/doc/fixesproto/fixesproto.txt>
    pub fn XFixesShowCursor(display : *mut Display, window : *mut Window);
}