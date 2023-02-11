use std::{cell::RefCell, rc::Rc};

use olympus::kleio::window::{KWindowManager, KEvent, KWindow, KEventReceiver, KWindowError, KEventController, KEventKeyboard, KEventMouse, KEventWindow};

use crate::{assert_err, assert_ok};

/*********
* CONSTS *
*********/
/// ID of KWindowManagerHollow
pub const KWINDOW_MANAGER_HOLLOW_ID: u8 = 123;

/// X position of KWindow
pub const POSX:isize = 100;

/// Y position of KWindow
pub const POSY:isize = 200;

/// Width of KWindow
pub const WIDTH:usize = 300;

/// Height of KWindow
pub const HEIGHT:usize = 400;

/// Count of total event given by KWindowManagerHollow
pub const EVENT_COUNT:usize = 25;

/********
* TESTS *
********/
#[test]
/// Create a KWindow from KWindowManagerHollow
/// 
/// # Verification(s)
/// V1 | KWindow::from() create KWindow without error.
fn kwindow_from() {
    // V1 | KWindow::from() create KWindow without error.
    KWindow::from(Box::new(KWindowManagerControl::new(POSX,POSY,WIDTH,HEIGHT)));
}

#[test]
/// Get the manager ID from KWindow
/// 
/// # Verification(s)
/// V1 | KWindow::get_window_manager_id() gives KWINDOW_MANAGER_HOLLOW_ID as ID.
fn kwindow_get_window_manager_id() {

    let w =  KWindow::from(Box::new(KWindowManagerControl::new(POSX,POSY,WIDTH,HEIGHT)));

    // V1 | KWindow::get_window_manager_id() gives KWINDOW_MANAGER_HOLLOW_ID as ID.
    assert!(w.get_window_manager_id() == KWINDOW_MANAGER_HOLLOW_ID, "KWindowManager id error!");
}

#[test]
/// Downcast KWindowManager to KWindowManagerHollow.
/// 
/// # Verification(s)
/// V1 | KWindow::downcast_window_manager() correctly downcast to KWindowManagerHollow.
/// V2 | KWindowManagerHollow::get_true() works.
fn kwindow_downcast_window_manager() {

    let w =  KWindow::from(Box::new(KWindowManagerControl::new(POSX,POSY,WIDTH,HEIGHT)));


    // V1 | KWindow::downcast_window_manager() correctly downcast to KWindowManagerHollow.
    w.downcast_window_manager::<KWindowManagerControl>();
    match KWindow::downcast_window_manager::<KWindowManagerControl>(&w) {
        Some(wm) => {
            // V2 | KWindowManagerHollow::get_true() works.
            assert!(wm.get_true(), "KWindow::downcast_window_manager() error!")
        },
        None => assert!(false, "KWindow::downcast_window_manager() error!"),
    }
}

#[test]
/// Add an event receiver to KWindow
/// 
/// # Verification(s)
/// V1 | KWindow::add_event_receiver() correctly add receiver to KWindow.
/// V2 | Adding the same receiver via KWindow::add_event_receiver() should result in KWindowError::ReceiverAlreadyExists.
fn kwindow_add_event_receiver() {

    let mut w =  KWindow::from(Box::new(KWindowManagerControl::new(POSX,POSY,WIDTH,HEIGHT)));

    // V1 | KWindow::add_event_receiver() correctly add receiver to KWindow.
    let rc1 = Rc::new(RefCell::new(KEventReceiverControl::new(true, true, true, true)));
    assert_ok!(w.add_event_receiver(rc1.clone()), 0);

    // V2 | Adding the same receiver via KWindow::add_event_receiver() should result in KWindowError::ReceiverAlreadyExists.
    assert_err!(w.add_event_receiver(rc1.clone()), KWindowError::ReceiverAlreadyExists);
}

#[test]
/// Remove an event receiver from KWindow
/// 
/// # Verification(s)
/// V1 | KWindow::remove_event_receiver() should return KWindowError::ReceiverNotFound since receiver was not added.
/// V2 | KWindow::add_event_receiver() correctly add receiver to KWindow.
/// V3 | KWindow::remove_event_receiver() should return Ok(0).
fn kwindow_remove_event_receiver() {

    let mut w =  KWindow::from(Box::new(KWindowManagerControl::new(POSX,POSY,WIDTH,HEIGHT)));
    let rc1 = Rc::new(RefCell::new(KEventReceiverControl::new(true, true, true, true)));

    // V1 | KWindow::remove_event_receiver() should return KWindowError::ReceiverNotFound since receiver was not added.
    assert_err!(w.remove_event_receiver(rc1.clone()), KWindowError::ReceiverNotFound);

    // V2 | KWindow::add_event_receiver() correctly add receiver to KWindow.
    assert_ok!(w.add_event_receiver(rc1.clone()), 0);

    // V3 | KWindow::remove_event_receiver() should return Ok(0).
    assert_ok!(w.remove_event_receiver(rc1.clone()), 0);
}



#[test]
/// Dispatch event of KWindow to multiple receivers.
/// 
/// # Verification(s)
/// V1 | KWindow::dispatch_events() should return KWindowError::DispatchNoReceiver since no receivers added.
/// V2 | Add 6 different receiver with different handling configuration. 
/// V3 | KWindow::dispatch_events() should now work without issue and return Ok(EVENT_COUNT).
/// V4 | Compare different receiver notification count with control.
/// V5 | KWindow::dispatch_events() should return Ok(0).
/// V6 | KWindow::remove_event_receiver() for each receiver should return Ok(index).
fn kwindow_dispatch_events() {
    let mut w = KWindow::from(Box::new(KWindowManagerControl::new(POSX,POSY,WIDTH,HEIGHT)));

    // V1 | KWindow::dispatch_events() should return KWindowError::DispatchNoReceiver since no receivers added.
    match w.dispatch_events() {
        Ok(_) => assert!(false, "KWindow::dispatch_events() should give an error!"),
        Err(err) => match err {
            KWindowError::DispatchNoReceiver => {},
            _ => assert!(false, "KWindow::dispatch_events() wrong error!")
        },
    }

    // V2 | Add 6 different receiver with different handling configuration. 
    let rc1 = Rc::new(RefCell::new(KEventReceiverControl::new(true, true, true, true)));
    let rc2 = Rc::new(RefCell::new(KEventReceiverControl::new(true, false, false, false)));
    let rc3 = Rc::new(RefCell::new(KEventReceiverControl::new(false, true, false, false)));
    let rc4 = Rc::new(RefCell::new(KEventReceiverControl::new(false, false, true, false)));
    let rc5 = Rc::new(RefCell::new(KEventReceiverControl::new(false, false, false, true)));
    
    let rc6 = Rc::new(RefCell::new(KEventReceiverControl::new(false, true, true, true)));
    rc6.borrow_mut().set_enabled(false);    // Disable rc6.


    assert_ok!(w.add_event_receiver(rc1.clone()), 0);
    assert_ok!(w.add_event_receiver(rc2.clone()), 1);
    assert_ok!(w.add_event_receiver(rc3.clone()), 2);
    assert_ok!(w.add_event_receiver(rc4.clone()), 3);
    assert_ok!(w.add_event_receiver(rc5.clone()), 4);
    assert_ok!(w.add_event_receiver(rc6.clone()), 5);

    // V3 | KWindow::dispatch_events() should now work without issue and return Ok(EVENT_COUNT).
    assert_ok!(w.dispatch_events(), EVENT_COUNT);

    // V4 | Compare different receiver notification count with control.
    assert_eq!(rc1.borrow().get_notification_count(), 0, "RC1 notification count should be 0!");
    assert_eq!(rc2.borrow().get_notification_count(), EVENT_COUNT - 11, "RC2 notification count should be {}!", EVENT_COUNT - 11);
    assert_eq!(rc3.borrow().get_notification_count(), EVENT_COUNT - 9, "RC3 notification count should be {}!", EVENT_COUNT - 9);
    assert_eq!(rc4.borrow().get_notification_count(), EVENT_COUNT - 5, "RC4 notification count should be {}!", EVENT_COUNT - 5);
    assert_eq!(rc5.borrow().get_notification_count(), EVENT_COUNT, "RC5 notification count should be {}!", EVENT_COUNT);
    assert_eq!(rc6.borrow().get_notification_count(), 0, "RC6 notification count should be 0!");

    // V5 | KWindow::dispatch_events() should return Ok(0).
    assert_ok!(w.dispatch_events(), 0);


    // V6 | KWindow::remove_event_receiver() for each receiver should return Ok(index).
    assert_ok!(w.remove_event_receiver(rc6), 5);
    assert_ok!(w.remove_event_receiver(rc5), 4);
    assert_ok!(w.remove_event_receiver(rc4), 3);
    assert_ok!(w.remove_event_receiver(rc3), 2);
    assert_ok!(w.remove_event_receiver(rc2), 1);
    assert_ok!(w.remove_event_receiver(rc1), 0);
}




/**********
* STRUCTS *
**********/
/// Control KWindowManager used for KWindowTest
struct KWindowManagerControl {

    /// Contains a pre-defined list of KEvent for tests
    events : Vec<KEvent>
}

impl KWindowManagerControl {
    /// Function implemented that always return true. Used for downcast tests.
    pub fn get_true(&self) -> bool {
        true
    }
}

impl KWindowManager for KWindowManagerControl {
    fn new(_pos_x:isize, _pos_y:isize, _width:usize, _height:usize) -> Self where Self: Sized {
        //Create events in self.events up to EVENT_COUNT
        let mut events : Vec<KEvent> = Vec::new();

        // Controller events
        events.push(KEvent::Controller(KEventController::Connected(0)));
        events.push(KEvent::Controller(KEventController::ButtonDown(0, 1)));
        events.push(KEvent::Controller(KEventController::ButtonUp(0, 1)));
        events.push(KEvent::Controller(KEventController::Axis(0, 0, -250)));
        events.push(KEvent::Controller(KEventController::Disconnected(0)));

        // Keyboard events
        events.push(KEvent::Keyboard(KEventKeyboard::KeyDown(123)));
        events.push(KEvent::Keyboard(KEventKeyboard::KeyUp(123)));

        // Mouse events
        events.push(KEvent::Mouse(KEventMouse::Moved((10,10),(0,0))));
        events.push(KEvent::Mouse(KEventMouse::ButtonDown(1,(10,10))));
        events.push(KEvent::Mouse(KEventMouse::ButtonUp(1,(10,10))));
        events.push(KEvent::Mouse(KEventMouse::Wheel(-255,255)));

        // Window events
        events.push(KEvent::Window(KEventWindow::Shown()));
        events.push(KEvent::Window(KEventWindow::Hidden()));
        events.push(KEvent::Window(KEventWindow::Exposed()));
        events.push(KEvent::Window(KEventWindow::Moved((10,10))));
        events.push(KEvent::Window(KEventWindow::Resized((100,100))));
        events.push(KEvent::Window(KEventWindow::SizeChanged((100,100))));
        events.push(KEvent::Window(KEventWindow::Minimized()));
        events.push(KEvent::Window(KEventWindow::Maximized()));
        events.push(KEvent::Window(KEventWindow::Restored()));
        events.push(KEvent::Window(KEventWindow::MouseEnter()));
        events.push(KEvent::Window(KEventWindow::MouseLeave()));
        events.push(KEvent::Window(KEventWindow::Focus()));
        events.push(KEvent::Window(KEventWindow::Blur()));
        events.push(KEvent::Window(KEventWindow::Close()));

        KWindowManagerControl { events }
    }

    fn get_event_count(&self) -> usize {
        self.events.len()
    }

    fn poll_event(&mut self) -> olympus::kleio::window::KEvent {
        self.events.pop().unwrap()
    }

    fn sync_event(&self) {
        // Nothing to do here.
    }

    fn get_id(&self) -> u8 {
        KWINDOW_MANAGER_HOLLOW_ID
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn show_cursor(&self, dd:bool) {
        todo!()
    }

    fn hide_cursor(&self) {
        todo!()
    }

    fn set_cursor_position(&self, position : (i32, i32)) {
        todo!()
    }
}



/// ##### Control receiver that count how many time it was notified.
struct KEventReceiverControl {

    /// Is the receiver enabled or not.
    enabled : bool,

    /// Flag returned when handling window event received.
    handle_window : bool,

    /// Flag returned when handling keyboard event received.
    handle_keyboard : bool,

    /// Flag returned when handling mouse event received.
    handle_mouse : bool,

    /// Flag returned when handling controller event received.
    handle_controller : bool,

    // Was listener notified.
    notification_count: usize,
}

impl KEventReceiverControl {
    /// Create a new instance of KEventReceiverHollow which handle event or not.
    /// 
    /// Returns new KEventReceiverHollow created.
    pub fn new(handle_window : bool, handle_keyboard : bool, handle_mouse : bool, handle_controller : bool) -> KEventReceiverControl {
        KEventReceiverControl { enabled: true, handle_window, handle_keyboard, handle_mouse, handle_controller, notification_count : 0 }
    }

    /// Get the count of notifications.
    pub fn get_notification_count(&self) -> usize {
        self.notification_count
    }

    /// Set if the receiver is enabled or not.
    pub fn set_enabled(&mut self, enabled : bool){
        self.enabled = enabled;
    }
}

impl KEventReceiver for KEventReceiverControl {
    fn receive(&mut self, event : &KEvent) -> bool {
        
        // Increment notifications
        self.notification_count += 1;
        
        match event {
            KEvent::Unknown => panic!("Error : Unknown event received!"),

            KEvent::Window(_) => self.handle_window,
            KEvent::Keyboard(_) => self.handle_keyboard,
            KEvent::Mouse(_) => self.handle_mouse,
            KEvent::Controller(_) => self.handle_controller,            
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}