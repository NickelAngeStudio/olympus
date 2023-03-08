// Contains bindings for XLib
use std::os::raw::{c_uchar, c_char, c_int, c_long, c_uint, c_ulong, c_void};

use super::attributes::{XWindowAttributes, Visual, XSetWindowAttributes, Screen};
use super::{ Display, Window };
use super::event::{ XEvent, Atom, XClientMessageEvent};


#[link(name = "X11")]
#[allow(dead_code)]
extern {
    
    /// The XCreateWindow function creates an unmapped subwindow for a specified parent window, returns the window ID of the 
    /// created window, and causes the X server to generate a CreateNotify event. The created window is placed 
    /// on top in the stacking order with respect to siblings. 
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XCreateWindow>
    pub(crate) fn XCreateWindow(display : *mut Display, parent : *mut Window, x : c_int, y : c_int, width : c_uint, height : c_uint, 
        border_width : c_uint, depth : c_int, class : c_uint, visual : *mut Visual, valuemask : c_uint, 
        attributes : *mut XSetWindowAttributes) -> *mut Window;

    /// The XOpenDisplay function returns a Display structure that serves as the connection to the 
    /// X server and that contains all the information about that X server.
    /// 
    /// # Reference(s)
    /// <https://www.x.org/releases/X11R7.5/doc/man/man3/XOpenDisplay.3.html>
    pub(crate) fn XOpenDisplay(display_name : *const c_char) -> *mut Display;


    /// The XCloseDisplay() function closes the connection to the X server for the display specified in the Display structure 
    /// and destroys all windows and resources that the client has created on this display.
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/display/XCloseDisplay.html>
    pub(crate) fn XCloseDisplay(display : *mut Display);


    /// The XSync() function flushes the output buffer and then waits until all requests have been received and processed by the X server.
    /// If you passed False, XSync() does not discard the events in the queue. If you passed True, XSync() discards all events in the queue, 
    /// including those events that were on the queue before XSync() was called. Client applications seldom need to call XSync(). 
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/event-handling/XSync.html>
    pub(crate) fn XSync(display : *mut Display, discard : bool);


    /// If mode is QueuedAlready(0), XEventsQueued() returns the number of events already in the event queue (and never performs a system call). 
    /// If mode is QueuedAfterFlush(1), XEventsQueued() returns the number of events already in the queue if the number is nonzero.
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/event-handling/XEventsQueued.html>
    pub(crate) fn XEventsQueued(display : *mut Display, mode : c_int) -> c_int;

    /// Returns the root window for the default screen. 
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/display/display-macros.html>
    pub(crate) fn XDefaultRootWindow(display : *mut Display) -> *mut Window;

    /// The XCreateSimpleWindow function creates an unmapped InputOutput subwindow for a specified parent window, 
    /// returns the window ID of the created window, and causes the X server to generate a CreateNotify event
    /// 
    /// # References(s)
    /// <https://tronche.com/gui/x/xlib/window/XCreateWindow.html>
    pub(crate) fn XCreateSimpleWindow(display : *mut Display, parent : *mut Window, x : c_int, y : c_int, 
        width : c_uint, height : c_uint, border_width : c_uint, border : c_ulong, background : c_ulong) -> *mut Window;


    /// The XMapWindow() function maps the window and all of its subwindows that have had map requests.
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/window/XMapWindow.html>
    pub(crate) fn XMapWindow(display : *mut Display, w : *mut Window);


    /// The XUnmapWindow function unmaps the specified window and causes the X server to generate an UnmapNotify event.
    /// 
    /// # Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XUnmapWindow>
    pub(crate) fn XUnmapWindow(display : *mut Display, w : *mut Window);

    /// The XSelectInput() function requests that the X server report the events associated 
    /// with the specified event mask. Initially, X will not report any of these events.
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/event-handling/XSelectInput.html>
    pub(crate) fn XSelectInput(display : *mut Display, w : *mut Window, event_mask: c_long);

    /// The XNextEvent() function copies the first event from the event queue into the specified 
    /// XEvent structure and then removes it from the queue.
    /// 
    /// # Reference(s)
    /// <https://tronche.com/gui/x/xlib/event-handling/manipulating-event-queue/XNextEvent.html>
    pub(crate) fn XNextEvent(display : *mut Display, event_return : *mut XEvent);

    /// Move the pointer to an arbitrary point in a window.
    /// 
    /// Reference(s)
    /// Moving the Pointer : <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html>
    pub(crate) fn XWarpPointer(display : *mut Display, src_w : *mut Window, dest_w : *mut Window, 
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
    pub(crate) fn XGrabPointer(display : *mut Display, grab_window : *mut Window, owner_events : bool, 
        event_mask : c_uint, intpointer_mode : c_uint, keyboard_mode : c_uint, confine_to : *mut Window, cursor : c_ulong, time : c_long) -> c_int;

    /// The XUngrabPointer function releases the pointer and any queued events if this client has actively grabbed the pointer from XGrabPointer.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XUngrabPointer>
    pub(crate) fn XUngrabPointer(display : *mut Display, time : c_long);

    /// The XInternAtom function returns the atom identifier associated with the specified atom_name string
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XInternAtom>
    pub(crate) fn XInternAtom(display : *mut Display, atom_name : *const c_char, only_if_exists : bool) -> Atom;

    /// The XGetAtomName function returns the name associated with the specified atom.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XGetAtomName>
    pub(crate) fn XGetAtomName(display : *mut Display, atom : Atom) ->  *const c_char;

    /// The XGetWindowProperty function returns the actual type of the property; the actual format of the property; 
    /// the number of 8-bit, 16-bit, or 32-bit items transferred; the number of bytes remaining to be read in the property; 
    /// and a pointer to the data actually returned.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XGetWindowProperty>
    pub(crate) fn XGetWindowProperty(display : *mut Display, w : *mut Window, property : Atom, long_offset : c_long, long_length : c_long, 
        delete : bool, req_type : Atom, actual_type_return : *mut Atom, actual_format_return : *mut c_int, nitems_return : *mut c_ulong, 
        bytes_after_return : *mut c_ulong, prop_return : *mut *mut c_char) -> c_int;

    /// The XChangeProperty function alters the property for the specified window and causes the X server 
    /// to generate a PropertyNotify event on that window.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XChangeProperty>
    pub(crate) fn  XChangeProperty(Ddisplay : *mut Display, w : *mut Window, property : Atom, property_type : Atom, 
        format : c_int, mode : c_int, data : *mut c_uchar, nelements : c_int);
    
    /// The function is a general-purpose Xlib routine that frees the specified data.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XFree>
    pub(crate) fn XFree(data : *mut c_char);

    /// The XGetWindowAttributes function returns the current attributes for the specified window to an XWindowAttributes structure. 
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XGetWindowAttributes>
    pub(crate) fn XGetWindowAttributes(display : *mut Display, w : *mut Window, window_attributes_return : *mut XWindowAttributes) -> c_int;


    /// The XStoreName function assigns the name passed to window_name to the specified window.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XStoreName>
    pub(crate) fn XStoreName(display : *mut Display, w : *mut Window, window_name : *mut c_char);

    /// The XMoveWindow function moves the specified window to the specified x and y coordinates, but it does not change the window's size, 
    /// raise the window, or change the mapping state of the window
    /// 
    /// References(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XMoveWindow>
    pub(crate) fn XMoveWindow(Ddisplay : *mut Display, w : *mut Window, x : c_int, y : c_int);

    /// The XResizeWindow function changes the inside dimensions of the specified window, not including its borders.
    /// 
    /// References(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XResizeWindow>
    pub(crate) fn XResizeWindow(Ddisplay : *mut Display, w : *mut Window, width : c_uint, height : c_uint);

    /// The XMoveResizeWindow function changes the size and location of the specified window without raising it.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XMoveResizeWindow>
    pub(crate) fn XMoveResizeWindow(display : *mut Display, w : *mut Window, x : c_int, y : c_int, width : c_uint, height : c_uint);

    /// Translate a coordinate in one window to the coordinate space of another window.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XTranslateCoordinates>
    pub(crate) fn XTranslateCoordinates(display : *mut Display, w : *mut Window, root : *mut Window, src_x : c_int, src_y : c_int, 
        dest_x_return : *mut c_int, dest_y_return : *mut c_int, child_return : *mut Window) -> bool;

    /// The XSetWindowBorderWidth function sets the specified window's border width to the specified width.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XSetWindowBorderWidth>
    pub(crate) fn XSetWindowBorderWidth(display : *mut Display, w : *mut Window, width : c_uint);

    /// The XSendEvent function identifies the destination window, determines which clients should 
    /// receive the specified events, and ignores any active grabs.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XSendEvent>
    pub(crate) fn XSendEvent(display : *mut Display, w : *mut Window, propagate : bool, event_mask : c_long, event_send : *mut XClientMessageEvent) -> c_uint;


    /// The XFlush function flushes the output buffer.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XFlush>
    pub(crate) fn XFlush(display : *mut Display);


    /// The XDestroyWindow function destroys the specified window as well as all of its subwindows and causes 
    /// the X server to generate a DestroyNotify event for each window. 
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XDestroyWindow>
    pub(crate) fn XDestroyWindow(display : *mut Display, w : *mut Window);

    /// Both return a pointer to the indicated screen.
    /// 
    /// Reference(s)
    /// <https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#XScreenOfDisplay>
    pub(crate) fn XScreenOfDisplay(display : *mut Display, screen_number : c_int) -> *const Screen;
    
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