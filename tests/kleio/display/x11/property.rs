use std::{rc::Rc, cell::RefCell};
use std::process::exit;

use olympus::kleio::display::{KWindow, KCursorMode, window::{KWINDOW_MIN_WIDTH, KWINDOW_MAX_WIDTH, KWINDOW_MIN_HEIGHT, KWINDOW_MAX_HEIGHT}, KWindowError, linux::server::KLinuxDisplayServerProvider, event::KEventDispatcher};

use crate::assert_err;
use crate::{ assert_ok, kleio::display::KEventReceiverControl, kwindow_x11_prepare, kwindow_x11_step_loop};

/*********
* CONSTS *
*********/
/// Window dimension
pub const KWINDOW_WIDTH:u32 = 320;
pub const KWINDOW_HEIGHT:u32 = 240;

/// New position for cursor tests. Must be center of KWindow.
pub const CURSOR_X:i32 = 160;
pub const CURSOR_Y:i32 = 120;

/// New position for KWindow.
pub const KWINDOW_POS_X:i32 = 151;
pub const KWINDOW_POS_Y:i32 = 262;

// New title for KWindow with special characters
pub const KWINDOW_TITLE : &str = "*Test window title çéàè*&?%!";

/********
* TESTS *
********/
#[test]
#[ignore = "User interaction"]
/// Get X11 KWindow display server elements.
/// 
/// # Verification(s)
/// V1 | KWindow::get_display_server_provider() returns the correct X11 provider.
/// V2 | KWindow::get_display_server_connection() returns a valid connection pointer.
/// V3 | KWindow::get_display_server_window() returns a valid window pointer.
fn kwindow_x11_get_display_server() {
    kwindow_x11_prepare!(wx11, dispatcher, receiver, {
        // V1 | KWindow::get_display_server_provider() returns the correct X11 provider.
        assert_eq!(wx11.get_display_server_provider(), KLinuxDisplayServerProvider::X11, "Wrong provider given!");
        
        // V2 | KWindow::get_display_server_connection() returns a valid connection pointer.
        assert_ne!(wx11.get_display_server_connection(), std::ptr::null_mut(), "KWindow X11 connection pointer error!");

        // V3 | KWindow::get_display_server_window() returns a valid window pointer.
        assert_ne!(wx11.get_display_server_window(), std::ptr::null_mut(), "KWindow X11 window pointer error!");

        kwindow_x11_step_loop!(wx11, dispatcher, receiver);
    });
}

#[test]
#[ignore = "User interaction"]
/// Get X11 KWindow event count.
/// 
/// # Verification(s)
/// V1 | KWindow::get_event_count() returns the event count without error.
fn kwindow_x11_get_event_count() {
    kwindow_x11_prepare!(wx11, dispatcher, receiver, {

        // V1 | KWindow::get_event_count() returns the event count without error.
        let _c = wx11.get_event_count();
    });
}


#[test]
#[ignore = "User interaction"]
/// Get and set X11 KWindow motion mode.
/// 
/// # Verification(s)
/// V1 | KWindow::get_cursor_mode() returns the default motion mode.
/// V2 | KWindow::is_cursor_confined() is false by default.
/// V3 | KWindow::is_cursor_visible() is true by default.
/// V4 | KWindow::hide_cursor() hide cursor without error.
/// V5 | Calling KWindow::hide_cursor() again doesn't generate error.
/// V6 | KWindow::is_cursor_visible() is false.
/// V7 | KWindow::show_cursor() show cursor without error.
/// V8 | Calling KWindow::show_cursor() again doesn't generate error.
/// V9 | KWindow::is_cursor_visible() is true.
/// V10 | KWindow::confine_cursor() prevent cursor from exiting boundaries without error.
/// V11 | Calling KWindow::confine_cursor() again doesn't generate error.
/// V12 | KWindow::is_cursor_confined() is true.
/// V13 | KWindow::set_cursor_mode() to acceleration keep the cursor in the middle of window.
/// V14 | KWindow::get_cursor_mode() returns acceleration.
/// V15 | KWindow::set_cursor_mode() to pointer release the cursor from the middle of window.
/// V16 | KWindow::get_cursor_mode() returns pointer.
/// V17 | KWindow::release_cursor() let cursor exit boundaries without error.
/// V18 | Calling KWindow::release_cursor() again without error.
/// V19 | KWindow::is_cursor_confined() is false.
/// V20 | Make cursor hidden. Exiting window must make the cursor reappear and disappear when reentering window.
/// V21 | Make cursor confined. Losing focus should release cursor while gaining focus should confine cursor.
/// V22 | Make cursor mode acceleration. Losing focus should release cursor while gaining focus should confine cursor in center.
fn kwindow_x11_cursor_properties() {
    kwindow_x11_prepare!(wx11, dispatcher, receiver, {

        // V1 | KWindow::get_motion_mode() returns the default motion mode.
        assert_eq!(wx11.get_cursor_mode(), KCursorMode::Pointer, "Wrong default cursor mode!");

        // V2 | KWindow::is_cursor_confined() is false by default.
        assert_eq!(wx11.is_cursor_confined(), false, "Cursor shouldn't be confined by default!");

        // V3 | KWindow::is_cursor_visible() is true by default.
        assert_eq!(wx11.is_cursor_visible(), true, "Cursor should be visible by default!");

        kwindow_x11_step_loop!("Cursor should be visible and not confined...", wx11, dispatcher, receiver);

        // V4 | KWindow::hide_cursor() hide cursor without error.
        wx11.hide_cursor();

        kwindow_x11_step_loop!("Cursor should be hidden and not confined...", wx11, dispatcher, receiver);

        // V5 | Calling KWindow::hide_cursor() again doesn't generate error.
        wx11.hide_cursor();

        // V6 | KWindow::is_cursor_visible() is false.
        assert_eq!(wx11.is_cursor_visible(), false, "Cursor shouldn't be visible!");

        // V7 | KWindow::show_cursor() show cursor without error.
        wx11.show_cursor();
        kwindow_x11_step_loop!("Cursor should be visible and not confined...", wx11, dispatcher, receiver);

        // V8 | Calling KWindow::show_cursor() again doesn't generate error.
        wx11.show_cursor();

        // V9 | KWindow::is_cursor_visible() is true.
        assert_eq!(wx11.is_cursor_visible(), true, "Cursor should be visible!");

        // V10 | KWindow::confine_cursor() prevent cursor from exiting boundaries without error.
        wx11.confine_cursor();
        kwindow_x11_step_loop!("Cursor should be visible and confined...", wx11, dispatcher, receiver);

        // V11 | Calling KWindow::confine_cursor() again doesn't generate error.
        wx11.confine_cursor();

        // V12 | KWindow::is_cursor_confined() is true.
        assert_eq!(wx11.is_cursor_confined(), true, "Cursor should confined!");

        // V13 | KWindow::set_cursor_mode() to acceleration keep the cursor in the middle of window.
        wx11.set_cursor_mode(KCursorMode::Acceleration);

        // V14 | KWindow::get_cursor_mode() returns acceleration.
        assert_eq!(wx11.get_cursor_mode(), KCursorMode::Acceleration, "Cursor mode should be Acceleration!");
        kwindow_x11_step_loop!("Cursor should be visible, confined and stuck in center...", wx11, dispatcher, receiver);

        // V15 | KWindow::set_cursor_mode() to pointer release the cursor from the middle of window.
        wx11.set_cursor_mode(KCursorMode::Pointer);

        // V16 | KWindow::get_cursor_mode() returns pointer.
        assert_eq!(wx11.get_cursor_mode(), KCursorMode::Pointer, "Cursor mode should be Pointer!");
        kwindow_x11_step_loop!("Cursor should be visible, confined and free to move...", wx11, dispatcher, receiver);

        // V17 | KWindow::release_cursor() let cursor exit boundaries without error.
        wx11.release_cursor();
        kwindow_x11_step_loop!("Cursor should be visible, released and free to move...", wx11, dispatcher, receiver);

        // V18 | Calling KWindow::release_cursor() again without error.
        wx11.release_cursor();

        // V19 | KWindow::is_cursor_confined() is false.
        assert_eq!(wx11.is_cursor_confined(), false, "Cursor shouldn't confined!");

        // V20 | Make cursor hidden. Exiting window must make the cursor reappear and disappear when reentering window.
        wx11.hide_cursor();
        kwindow_x11_step_loop!("Exiting window must make the cursor reappear and disappear when reentering window...", wx11, dispatcher, receiver);

        // V21 | Make cursor confined. Losing focus should release cursor while gaining focus should confine cursor.
        wx11.show_cursor();
        wx11.confine_cursor();
        kwindow_x11_step_loop!("Losing focus should release cursor while gaining focus should confine cursor...", wx11, dispatcher, receiver);

        // V22 | Make cursor mode acceleration. Losing focus should release cursor while gaining focus should confine cursor in center.
        wx11.set_cursor_mode(KCursorMode::Acceleration);
        kwindow_x11_step_loop!("Losing focus should release cursor while gaining focus should confine cursor in center...", wx11, dispatcher, receiver);
    });
}
/*
#[test]
#[ignore = "User interaction"]
/// Get and set X11 KWindow cursor position.
/// 
/// # Verification(s)
/// V1 | KWindow::get_cursor_position() returns the current cursor position.
/// V2 | KWindow::set_cursor_position() set the new position without errors.
/// V3 | KWindow::get_cursor_position() returns the new position.
/// V4 | Change motion mode to Acceleration. Window::set_cursor_position() should give center.
fn kwindow_x11_cursor_position() {
    kwindow_x11_prepare!(wx11, dispatcher, receiver, {
        // V1 | KWindow::get_cursor_position() returns the current cursor position.
        let _cp = wx11.get_cursor_position();

        // V2 | KWindow::set_cursor_position() set the new position without errors.
        wx11.set_cursor_position((CURSOR_X / 2, CURSOR_Y / 2));

        // V3 | KWindow::get_cursor_position() returns the new position.
        let _cp = wx11.get_cursor_position();
        assert_eq!(_cp.0, CURSOR_X / 2, "Cursor X expect {} and not {}!", CURSOR_X / 2, _cp.0);
        assert_eq!(_cp.1, CURSOR_Y / 2, "Cursor Y expect {} and not {}!", CURSOR_Y / 2, _cp.1);

        // V4 | Change motion mode to Acceleration. Window::set_cursor_position() should give center.
        wx11.set_cursor_mode(KMouseMode::Acceleration);

        let _cp = wx11.get_cursor_position();
        assert_eq!(_cp.0, CURSOR_X, "Cursor X expect {} and not {}!", CURSOR_X, _cp.0);
        assert_eq!(_cp.1, CURSOR_Y, "Cursor Y expect {} and not {}!", CURSOR_Y, _cp.1);
    });
}
*/

#[test]
#[ignore = "User interaction"]
/// Get and set X11 KWindow position.
/// 
/// # Verification(s)
/// V1 | KWindow::get_position() returns the default position.
/// V2 | KWindow::set_position() set the new position without errors.
/// V3 | KWindow::get_position() returns the new position.
/// V4 | KWindow::set_position() set the new position to negative position without errors.
/// V5 | KWindow::get_position() returns the new position.
fn kwindow_x11_position() {
    kwindow_x11_prepare!(wx11, dispatcher, receiver, {
        // V1 | KWindow::get_position() returns the default position.
        match wx11.get_screen_list().get_primary_screen() {
            Some(primary) => {
                let defposx:i32 = (primary.get_current_width() / 2 - KWINDOW_WIDTH / 2).try_into().unwrap();
                let defposy:i32 = (primary.get_current_height() / 2 - KWINDOW_HEIGHT / 2).try_into().unwrap();

                let pos = wx11.get_position();
                assert_eq!(pos.0, defposx, "Position X expect {} and not {}!", defposx, pos.0);
                assert_eq!(pos.1, defposy, "Position Y expect {} and not {}!", defposy, pos.1);

            },
            None => panic!("No screen connected for test!"),
        }        

        // V2 | KWindow::set_position() set the new position without errors.
        wx11.set_position((KWINDOW_POS_X, KWINDOW_POS_Y));

        // V3 | KWindow::get_position() returns the new position.
        let pos = wx11.get_position();
        assert_eq!(pos.0, KWINDOW_POS_X, "Position X expect {} and not {}!", KWINDOW_POS_X, pos.0);
        assert_eq!(pos.1, KWINDOW_POS_Y, "Position Y expect {} and not {}!", KWINDOW_POS_Y, pos.1);

        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V4 | KWindow::set_position() set the new position to negative position without errors.
        wx11.set_position((KWINDOW_POS_X * -1, KWINDOW_POS_Y * -1));

        // V5 | KWindow::get_position() returns the new position.
        let pos = wx11.get_position();
        assert_eq!(pos.0, KWINDOW_POS_X * -1, "Position X expect {} and not {}!", KWINDOW_POS_X * -1, pos.0);
        assert_eq!(pos.1, KWINDOW_POS_Y * -1, "Position Y expect {} and not {}!", KWINDOW_POS_Y * -1, pos.1);
    });
}

#[test]
#[ignore = "User interaction"]
/// Get and set X11 KWindow size.
/// 
/// # Verification(s)
/// V1 | KWindow::get_size() returns the default size.
/// V2 | KWindow::set_size() width < KWINDOW_MIN_WIDTH should gives KWindowError::WindowSizeError.
/// V3 | KWindow::set_size() width > KWINDOW_MAX_WIDTH should gives KWindowError::WindowSizeError.
/// V4 | KWindow::set_size() height < KWINDOW_MIN_HEIGHT should gives KWindowError::WindowSizeError.
/// V5 | KWindow::set_size() height > KWINDOW_MAX_HEIGHT should gives KWindowError::WindowSizeError.
/// V6 | KWindow::set_size() set the new size without errors.
/// V7 | KWindow::get_size() returns the new size.
fn kwindow_x11_size() {
    kwindow_x11_prepare!(wx11, dispatcher, receiver, {
        // V1 | KWindow::get_size() returns the default size.
        let size = wx11.get_size();
        assert_eq!(size.0, KWINDOW_WIDTH, "Width expect {} and not {}!", KWINDOW_WIDTH, size.0);
        assert_eq!(size.1, KWINDOW_HEIGHT, "Height expect {} and not {}!", KWINDOW_HEIGHT, size.1);

        // V2 | KWindow::set_size() width < KWINDOW_MIN_WIDTH should gives KWindowError::WindowSizeError.
        assert_err!(wx11.set_size((KWINDOW_MIN_WIDTH - 1, KWINDOW_HEIGHT)), KWindowError::WindowSizeError);

        // V3 | KWindow::set_size() width > KWINDOW_MAX_WIDTH should gives KWindowError::WindowSizeError.
        assert_err!(wx11.set_size((KWINDOW_MAX_WIDTH + 1, KWINDOW_HEIGHT)), KWindowError::WindowSizeError);

        // V4 | KWindow::set_size() height < KWINDOW_MIN_HEIGHT should gives KWindowError::WindowSizeError.
        assert_err!(wx11.set_size((KWINDOW_WIDTH, KWINDOW_MIN_HEIGHT - 1)), KWindowError::WindowSizeError);

        // V5 | KWindow::set_size() height > KWINDOW_MAX_HEIGHT should gives KWindowError::WindowSizeError.
        assert_err!(wx11.set_size((KWINDOW_WIDTH, KWINDOW_MAX_HEIGHT + 1)), KWindowError::WindowSizeError);

        // V6 | KWindow::set_size() set the new size without errors.
        assert_ok!(wx11.set_size((KWINDOW_WIDTH / 2, KWINDOW_HEIGHT / 2)));

        // V7 | KWindow::get_size() returns the new size.
        let size = wx11.get_size();
        assert_eq!(size.0, KWINDOW_WIDTH / 2, "Width expect {} and not {}!", KWINDOW_WIDTH / 2, size.0);
        assert_eq!(size.1, KWINDOW_HEIGHT / 2, "Height expect {} and not {}!", KWINDOW_HEIGHT / 2, size.1);
    });
}

#[test]
#[ignore = "User interaction"]
/// Get and set X11 KWindow title.
/// 
/// # Verification(s)
/// V1 | KWindow::get_title() returns the default title.
/// V2 | KWindow::set_title() set the new title without errors.
/// V3 | KWindow::get_title() returns the new title.
fn kwindow_x11_title() {
    kwindow_x11_prepare!(wx11, dispatcher, receiver, {
        // V1 | KWindow::get_title() returns the default title.
        assert_eq!(wx11.get_title(), "", "Default title error!");

        // V2 | KWindow::set_title() set the new title without errors.
        wx11.set_title(KWINDOW_TITLE);

        // V3 | KWindow::get_title() returns the new title.
        assert_eq!(wx11.get_title(), KWINDOW_TITLE, "Title expect {:?} and not {:?}!", KWINDOW_TITLE, wx11.get_title());
    });
}
/*
#[test]
#[ignore = "User interaction"]
/// Bind and unbind X11 KWindow cursor.
/// 
/// # Verification(s)
/// V1 | KWindow::is_cursor_confined() returns false as it is not binded by default.
/// V2 | KWindow::bind_cursor() work without error and cursor stay in boundaries.
/// V3 | KWindow::is_cursor_confined() returns true as it is now binded.
/// V4 | KWindow::unbind_cursor() work without error and cursor can exit boundaries.
/// V5 | KWindow::is_cursor_confined() returns false as it is now unbinded.
fn kwindow_x11_bind_unbind_cursor() {
    kwindow_x11_prepare!(wx11, dispatcher, receiver, {
        // V1 | KWindow::is_cursor_confined() returns false as it is not binded by default.
        assert!(!wx11.is_cursor_confined(), "Cursor should not be binded by default!");

        // V2 | KWindow::bind_cursor() work without error and cursor stay in boundaries.
        wx11.confine_cursor();

        // V3 | KWindow::is_cursor_confined() returns true as it is now binded.
        assert!(wx11.is_cursor_confined(), "Cursor should be binded!");

        // User review if cursor is really binded
        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V4 | KWindow::unbind_cursor() work without error and cursor can exit boundaries.
        wx11.release_cursor();

        // V5 | KWindow::is_cursor_confined() returns false as it is now unbinded.
        assert!(!wx11.is_cursor_confined(), "Cursor should not be binded anymore!");
    });
}

#[test]
#[ignore = "User interaction"]
/// Hide and show X11 KWindow cursor.
/// 
/// # Verification(s)
/// V1 | KWindow::is_cursor_hidden() returns false as it is visible by default.
/// V2 | KWindow::hide_cursor() work without error and cursor is hidden.
/// V3 | KWindow::is_cursor_hidden() returns true as it is now hidden.
/// V4 | KWindow::show_cursor() work without error and cursor is now visible.
/// V5 | KWindow::is_cursor_hidden() returns false as it is now visible.
fn kwindow_x11_hide_show_cursor() {
    kwindow_x11_prepare!(wx11, dispatcher, receiver, {
        // V1 | KWindow::is_cursor_hidden() returns false as it is visible by default.
        assert!(!wx11.is_cursor_hidden(), "Cursor should not be hidden by default!");

        // V2 | KWindow::hide_cursor() work without error and cursor is hidden.
        wx11.hide_cursor();

        // V3 | KWindow::is_cursor_hidden() returns true as it is now hidden.
        assert!(wx11.is_cursor_hidden(), "Cursor should now be hidden!");

        // User review if cursor is really hidden
        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V4 | KWindow::show_cursor() work without error and cursor is now visible.
        wx11.show_cursor();

        // V5 | KWindow::is_cursor_hidden() returns false as it is now visible.
        assert!(!wx11.is_cursor_hidden(), "Cursor should now be visible!");
    });
}
*/
#[test]
#[ignore = "User interaction"]
/// Minimize, Maximize, Fullscreen and restore X11 KWindow test.
/// 
/// # Verification(s)
/// V1 | KWindow::is_fullscreen(), is_maximized(), is_minimized() all returns false as default.
/// V2 | KWindow::set_minimized() work without error and window is minimized.
/// V3 | KWindow::is_fullscreen() = false, is_maximized() = false, is_minimized() = true.
/// V4 | KWindow::restore() work without error and window now restored.
/// V5 | KWindow::is_fullscreen() = false, is_maximized() = false, is_minimized() = false.
/// V6 | KWindow::set_maximized() work without error and window is maximized.
/// V7 | KWindow::is_fullscreen() = false, is_maximized() = true, is_minimized() = false.
/// V8 | KWindow::restore() work without error and window now restored.
/// V9 | KWindow::is_fullscreen() = false, is_maximized() = false, is_minimized() = false.
/// V10 | KWindow::set_fullscreen() work without error and window now fullscreen.
/// V11 | KWindow::is_fullscreen() = true, is_maximized() = false, is_minimized() = false.
/// V12 | KWindow::restore() work without error and window now restored.
/// V13 | KWindow::is_fullscreen() = false, is_maximized() = false, is_minimized() = false.
/// V14 | KWindow::set_minimized() work without error and window is minimized.
/// V15 | KWindow::is_fullscreen() = false, is_maximized() = false, is_minimized() = true.
/// V16 | KWindow::set_fullscreen() work without error and window now fullscreen from minimized.
/// V17 | KWindow::is_fullscreen() = true, is_maximized() = false, is_minimized() = false.
/// V18 | KWindow::set_maximized() work without error and window is maximized and exit fullscreen.
/// V19 | KWindow::is_fullscreen() = false, is_maximized() = true, is_minimized() = false.
/// V20 | KWindow::set_minimized() called multiple time without error.
/// V21 | KWindow::set_maximized() called multiple time without error.
/// V22 | KWindow::set_fullscreen() called multiple time without error.
/// V23 | KWindow::restore() called multiple time without error.
/// V24 | Multiple chain call of set_minimized, set_maximized, set_fullscreen, restore without error.
fn kwindow_x11_min_max_full_res() {
    kwindow_x11_prepare!(wx11, dispatcher, receiver, {
        // V1 | KWindow::is_fullscreen(), is_maximized(), is_minimized() all returns false as default.
        assert!(!wx11.is_fullscreen() && !wx11.is_maximized() && !wx11.is_minimized(), "is_fullscreen(), is_maximized(),is_minimized should all be false!");

        // V2 | KWindow::set_minimized() work without error and window is minimized.
        wx11.set_minimized();

        // V3 | KWindow::is_fullscreen() = false, is_maximized() = false, is_minimized() = true.
        assert!(!wx11.is_fullscreen() && !wx11.is_maximized() && wx11.is_minimized(), "Only is_minimized() should be true!");
        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V4 | KWindow::restore() work without error and window now restored.
        wx11.restore();

        // V5 | KWindow::is_fullscreen() = false, is_maximized() = false, is_minimized() = false.
        assert!(!wx11.is_fullscreen() && !wx11.is_maximized() && !wx11.is_minimized(), "is_fullscreen(), is_maximized(),is_minimized should all be false!");
        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V6 | KWindow::set_maximized() work without error and window is maximized.
        wx11.set_maximized();

        // V7 | KWindow::is_fullscreen() = false, is_maximized() = true, is_minimized() = false.
        assert!(!wx11.is_fullscreen() && wx11.is_maximized() && !wx11.is_minimized(), "Only is_maximized() should be true!");
        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V8 | KWindow::restore() work without error and window now restored.
        wx11.restore();

        // V9 | KWindow::is_fullscreen() = false, is_maximized() = false, is_minimized() = false.
        assert!(!wx11.is_fullscreen() && !wx11.is_maximized() && !wx11.is_minimized(), "is_fullscreen(), is_maximized(),is_minimized should all be false!");
        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V10 | KWindow::set_fullscreen() work without error and window now fullscreen.
        wx11.set_fullscreen();

        // V11 | KWindow::is_fullscreen() = true, is_maximized() = false, is_minimized() = false.
        assert!(wx11.is_fullscreen() && !wx11.is_maximized() && !wx11.is_minimized(), "Only is_fullscreen() should be true!");
        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V12 | KWindow::restore() work without error and window now restored.
        wx11.restore();

        // V13 | KWindow::is_fullscreen() = false, is_maximized() = false, is_minimized() = false.
        assert!(!wx11.is_fullscreen() && !wx11.is_maximized() && !wx11.is_minimized(), "is_fullscreen(), is_maximized(),is_minimized should all be false!");
        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V14 | KWindow::set_minimized() work without error and window is minimized.
        wx11.set_minimized();

        // V15 | KWindow::is_fullscreen() = false, is_maximized() = false, is_minimized() = true.
        assert!(!wx11.is_fullscreen() && !wx11.is_maximized() && wx11.is_minimized(), "Only is_minimized() should be true!");
        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V16 | KWindow::set_fullscreen() work without error and window now fullscreen from minimized.
        wx11.set_fullscreen();

        // V17 | KWindow::is_fullscreen() = true, is_maximized() = false, is_minimized() = false.
        assert!(wx11.is_fullscreen() && !wx11.is_maximized() && !wx11.is_minimized(), "Only is_fullscreen() should be true!");
        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V18 | KWindow::set_maximized() work without error and window is maximized and exit fullscreen.
        wx11.set_maximized();

        // V19 | KWindow::is_fullscreen() = false, is_maximized() = true, is_minimized() = false.
        assert!(!wx11.is_fullscreen() && !wx11.is_maximized() && wx11.is_minimized(), "Only is_maximized() should be true!");
        kwindow_x11_step_loop!(wx11, dispatcher, receiver);

        // V20 | KWindow::set_minimized() called multiple time without error.
        for _ in 0..100 {
            wx11.set_minimized();
        }
        // V21 | KWindow::set_maximized() called multiple time without error.
        for _ in 0..100 {
            wx11.set_maximized();
        }
        // V22 | KWindow::set_fullscreen() called multiple time without error.
        for _ in 0..100 {
            wx11.set_fullscreen();
        }
        // V23 | KWindow::restore() called multiple time without error.
        for _ in 0..100 {
            wx11.restore();
        }

        // V24 | Multiple chain call of set_minimized, set_maximized, set_fullscreen, restore without error.
        for i in 0..255 {
            if i % 7 == 0 {
                wx11.set_minimized();
            }
            if i % 11 == 0 {
                wx11.set_maximized();
            }
            if i % 23 == 0 {
                wx11.set_fullscreen();
            }
            if i % 29 == 0 {
                wx11.restore();
            }
        }

    });
}