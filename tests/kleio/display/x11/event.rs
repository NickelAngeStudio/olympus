use std::{rc::Rc, cell::RefCell, process::exit};

use olympus::{kleio::display::{KWindow, window::{KWINDOW_MIN_WIDTH, KWINDOW_MAX_WIDTH, KWINDOW_MIN_HEIGHT, KWINDOW_MAX_HEIGHT}, linux::server::KLinuxDisplayServerProvider, event::KEventDispatcher}, error::{OlympusError, KWindowError}};

use crate::{assert_err, assert_ok, kleio::display::{KEventReceiverControl}, kwindow_x11_prepare, kwindow_x11_step_loop};

/*********
* CONSTS *
*********/
/// Window dimension
pub const KWINDOW_WIDTH:u32 = 320;
pub const KWINDOW_HEIGHT:u32 = 240;

/********
* TESTS *
********/
#[test]
#[ignore]   // Should be run manually for control.
/// Create a new X11 KWindow without error.
/// 
/// # Verification(s)
/// V1 | KWindow::new(x11) width < KWINDOW_MIN_WIDTH should gives OlympusError::KWindowSizeError.
/// V2 | KWindow::new(x11) width > KWINDOW_MAX_WIDTH should gives OlympusError::KWindowSizeError.
/// V3 | KWindow::new(x11) height < KWINDOW_MIN_HEIGHT should gives OlympusError::KWindowSizeError.
/// V4 | KWindow::new(x11) height > KWINDOW_MAX_HEIGHT should gives OlympusError::KWindowSizeError.
/// V5 | KWindow::new(x11) created without error.
fn kwindow_x11_new() {
    // V1 | KWindow::new(x11) width < KWINDOW_MIN_WIDTH should gives KWindowError::WindowSizeError.
    assert_err!(KWindow::new(KWINDOW_MIN_WIDTH - 1, KWINDOW_HEIGHT, KLinuxDisplayServerProvider::X11), OlympusError::KWindow(KWindowError::SizeError));

    // V2 | KWindow::new(x11) width > KWINDOW_MAX_WIDTH should gives KWindowError::WindowSizeError.
    assert_err!(KWindow::new(KWINDOW_MAX_WIDTH + 1, KWINDOW_HEIGHT, KLinuxDisplayServerProvider::X11), OlympusError::KWindow(KWindowError::SizeError));

    // V3 | KWindow::new(x11) height < KWINDOW_MIN_HEIGHT should gives KWindowError::WindowSizeError.
    assert_err!(KWindow::new(KWINDOW_WIDTH, KWINDOW_MIN_HEIGHT - 1, KLinuxDisplayServerProvider::X11), OlympusError::KWindow(KWindowError::SizeError));

    // V4 | KWindow::new(x11) height > KWINDOW_MAX_HEIGHT should gives KWindowError::WindowSizeError.
    assert_err!(KWindow::new(KWINDOW_WIDTH, KWINDOW_MAX_HEIGHT + 1, KLinuxDisplayServerProvider::X11), OlympusError::KWindow(KWindowError::SizeError));

    // V5 | KWindow::new(x11) created without error.
    let _kw = assert_ok!(KWindow::new(KWINDOW_WIDTH, KWINDOW_HEIGHT, KLinuxDisplayServerProvider::X11));
}

#[test]
#[ignore]   // Should be run manually for control.
/// Dispatch x11 KWindow events.
/// 
/// # Verification(s)
/// V1 | KWindow::dispatch_events() must dispatch without errors.
fn kwindow_x11_dispatch_events() {

    kwindow_x11_prepare!(wx11, dispatcher, receiver, {
        // V1 | KWindow::dispatch_events() must dispatch without errors.
        loop {
            wx11.dispatch_events(&mut dispatcher, true);
            match receiver.borrow().get_state() {
                crate::kleio::display::KEventReceiverControlState::Running => {},
                crate::kleio::display::KEventReceiverControlState::NextStep => break,
                crate::kleio::display::KEventReceiverControlState::Exit => exit(0),
            }
        }

    });    

}

#[test]
#[ignore]   // Should be run manually for control.
/// Create and run KWindow in a different thread.
/// 
/// # Verification(s)
/// V1 | KWindow works in a different thread.
fn kwindow_x11_thread() {
    use std::thread;

    // V1 | KWindow works in a different thread.
    let thread_join_handle = thread::spawn(move || {

        kwindow_x11_prepare!(wx11, dispatcher, receiver, {
            kwindow_x11_step_loop!(wx11, dispatcher, receiver);
        });    
    });

    let _res = thread_join_handle.join();
}

