use std::{cell::RefCell, rc::Rc};

use olympus::kleio::window::event::{ KEvent, KEventReceiver, KEventController, KEventKeyboard, KEventMouse, KEventWindow, KEventDispatcher, KEventDispatcherError};

use crate::{assert_err, assert_ok};

/*********
* CONSTS *
*********/
/// Count of total event given by KWindowManagerHollow
pub const EVENT_COUNT:usize = 25;

/********
* TESTS *
********/
#[test]
/// Create a new KEventDispatcher with and without unhandled logging.
/// 
/// # Verification(s)
/// V1 | KEventDispatcher::new(true) create KEventDispatcher without error.
/// V2 | KEventDispatcher::new(false) create KEventDispatcher without error.
fn kevent_dispatcher_new() {
    // V1 | KEventDispatcher::new(true) create KEventDispatcher without error.
    KEventDispatcher::new(true);

    // V2 | KEventDispatcher::new(false) create KEventDispatcher without error.
    KEventDispatcher::new(false);
}

#[test]
/// Try to dispatch an event to [KEventDispatcher] without receivers.
/// 
/// # Verification(s)
/// V1 | KEventDispatcher::dispatch() should NOT give an error and should NOT be handled.
/// V2 | KEventDispatcher::get_unhandled_event() should have 1 unhandled event.
/// V3 | KEventDispatcher::clear_unhandled_event() should remove KEvent without error.
/// V4 | KEventDispatcher::get_unhandled_event() should have 0 unhandled event.
fn kevent_dispatcher_dispatch_no_receiver() {

    let mut ked = KEventDispatcher::new(true);
    let mut events = KEventDispatcherControl::new();

    // V1 | KEventDispatcher::dispatch() should NOT give an error.
    assert!(ked.dispatch(&events.get_event()) == false, "KEventDispatcher::dispatch() error. Shouldn't be handled!");

    // V2 | KEventDispatcher::get_unhandled_event() should have 1 unhandled event.
    assert!(ked.get_unhandled_events().len() == 1, "KEventDispatcher::dispatch() error. Shouldn't be handled!");

    // V3 | KEventDispatcher::clear_unhandled_event() should remove KEvent without error.
    ked.clear_unhandled_events();

    // V4 | KEventDispatcher::get_unhandled_event() should have 0 unhandled event.
    assert!(ked.get_unhandled_events().len() == 0, "KEventDispatcher::dispatch() error. Should be cleared!");
}

#[test]
/// Add an event receiver to KEventDispatcher
/// 
/// # Verification(s)
/// V1 | KEventDispatcher::add_event_receiver() correctly add receiver to KEventDispatcher.
/// V2 | Adding the same receiver via KEventDispatcher::add_event_receiver() should result in KEventDispatcherError::ReceiverAlreadyExists.
fn kevent_dispatcher_add_event_receiver() {

    let mut ked = KEventDispatcher::new(true);

    // V1 | KEventDispatcher::add_event_receiver() correctly add receiver to KEventDispatcher.
    let rc1 = Rc::new(RefCell::new(KEventReceiverControl::new(true, true, true, true)));
    assert_ok!(ked.add_event_receiver(rc1.clone()), 0);

    // V2 | Adding the same receiver via KEventDispatcher::add_event_receiver() should result in KEventDispatcherError::ReceiverAlreadyExists.
    assert_err!(ked.add_event_receiver(rc1.clone()), KEventDispatcherError::ReceiverAlreadyExists);
}

#[test]
/// Remove an event receiver from KEventDispatcher
/// 
/// # Verification(s)
/// V1 | KEventDispatcher::remove_event_receiver() should return KEventDispatcherError::ReceiverNotFound since receiver was not added.
/// V2 | KEventDispatcher::add_event_receiver() correctly add receiver to KEventDispatcher.
/// V3 | KEventDispatcher::remove_event_receiver() should return Ok(0).
fn kevent_dispatcher_remove_event_receiver() {

    let mut ked = KEventDispatcher::new(true);


    let rc1 = Rc::new(RefCell::new(KEventReceiverControl::new(true, true, true, true)));

    // V1 | KEventDispatcher::remove_event_receiver() should return KEventDispatcherError::ReceiverNotFound since receiver was not added.
    assert_err!(ked.remove_event_receiver(rc1.clone()), KEventDispatcherError::ReceiverNotFound);

    // V2 | KEventDispatcher::add_event_receiver() correctly add receiver to KEventDispatcher.
    assert_ok!(ked.add_event_receiver(rc1.clone()), 0);

    // V3 | KEventDispatcher::remove_event_receiver() should return Ok(0).
    assert_ok!(ked.remove_event_receiver(rc1.clone()), 0);
}



#[test]
/// Dispatch multiple event of multiple receivers.
/// 
/// # Verification(s)
/// V1 | Add 6 different receiver with different handling configuration. 
/// V2 | Dispatch all control events via KEventDispatcher::dispatch
/// V3 | Compare different receiver notification count with control.
/// V4 | KEventDispatcher shouldn't have any unhandled events.
/// V5 | KEventDispatcher::remove_event_receiver() for each receiver should return Ok(index).
/// V6 | Dispatch all control events via KEventDispatcher::dispatch without any receivers.
/// V7 | KEventDispatcher should have EVENT_COUNT unhandled events.
/// V8 | KEventDispatcher::clear_unhandled_event() should remove KEvent without error.
/// V9 | KEventDispatcher::get_unhandled_event() should have 0 unhandled event.
fn kevent_dispatcher_dispatch_events() {

    let mut ked = KEventDispatcher::new(true);
    let mut events = KEventDispatcherControl::new();

    // V1 | Add 6 different receiver with different handling configuration. 
    let rc1 = Rc::new(RefCell::new(KEventReceiverControl::new(true, true, true, true)));
    let rc2 = Rc::new(RefCell::new(KEventReceiverControl::new(true, false, false, false)));
    let rc3 = Rc::new(RefCell::new(KEventReceiverControl::new(false, true, false, false)));
    let rc4 = Rc::new(RefCell::new(KEventReceiverControl::new(false, false, true, false)));
    let rc5 = Rc::new(RefCell::new(KEventReceiverControl::new(false, false, false, true)));
    
    let rc6 = Rc::new(RefCell::new(KEventReceiverControl::new(false, true, true, true)));
    rc6.borrow_mut().set_enabled(false);    // Disable rc6.

    assert_ok!(ked.add_event_receiver(rc1.clone()), 0);
    assert_ok!(ked.add_event_receiver(rc2.clone()), 1);
    assert_ok!(ked.add_event_receiver(rc3.clone()), 2);
    assert_ok!(ked.add_event_receiver(rc4.clone()), 3);
    assert_ok!(ked.add_event_receiver(rc5.clone()), 4);
    assert_ok!(ked.add_event_receiver(rc6.clone()), 5);

    // V2 | Dispatch all control events via KEventDispatcher::dispatch
    loop {
        let event = events.get_event();

        match event {
            KEvent::Unknown => break,
            _ => {
                assert!(ked.dispatch(&event), "Unhandled event!");
            },
        }
    }


    // V3 | Compare different receiver notification count with control.
    assert_eq!(rc1.borrow().get_notification_count(), 0, "RC1 notification count should be 0!");
    assert_eq!(rc2.borrow().get_notification_count(), EVENT_COUNT - 11, "RC2 notification count should be {}!", EVENT_COUNT - 11);
    assert_eq!(rc3.borrow().get_notification_count(), EVENT_COUNT - 9, "RC3 notification count should be {}!", EVENT_COUNT - 9);
    assert_eq!(rc4.borrow().get_notification_count(), EVENT_COUNT - 5, "RC4 notification count should be {}!", EVENT_COUNT - 5);
    assert_eq!(rc5.borrow().get_notification_count(), EVENT_COUNT, "RC5 notification count should be {}!", EVENT_COUNT);
    assert_eq!(rc6.borrow().get_notification_count(), 0, "RC6 notification count should be 0!");

    // V4 | KEventDispatcher shouldn't have any unhandled events.
    assert!(ked.get_unhandled_events().len() == 0, "KEventDispatcher::dispatch() error. Should be empty!");

    // V5 | KEventDispatcher::remove_event_receiver() for each receiver should return Ok(index).
    assert_ok!(ked.remove_event_receiver(rc6), 5);
    assert_ok!(ked.remove_event_receiver(rc5), 4);
    assert_ok!(ked.remove_event_receiver(rc4), 3);
    assert_ok!(ked.remove_event_receiver(rc3), 2);
    assert_ok!(ked.remove_event_receiver(rc2), 1);
    assert_ok!(ked.remove_event_receiver(rc1), 0);

    // V6 | Dispatch all control events via KEventDispatcher::dispatch without any receivers.
    let mut events = KEventDispatcherControl::new();

    loop {
        let event = events.get_event();

        match event {
            KEvent::Unknown => break,
            _ => {
                assert!(!ked.dispatch(&event), "Handled event!");
            },
        }
    }

    // V7 | KEventDispatcher should have EVENT_COUNT unhandled events.
    assert!(ked.get_unhandled_events().len() == EVENT_COUNT, "KEventDispatcher::dispatch() error. Should have {} events!", EVENT_COUNT);

    // V8 | KEventDispatcher::clear_unhandled_event() should remove KEvent without error.
    ked.clear_unhandled_events();

    // V9 | KEventDispatcher::get_unhandled_event() should have 0 unhandled event.
    assert!(ked.get_unhandled_events().len() == 0, "KEventDispatcher::dispatch() error. Should be cleared!");

}




/**********
* STRUCTS *
**********/
/// Control Events used for KEventDispatcher
struct KEventDispatcherControl {

    /// Contains a pre-defined list of KEvent for tests
    events : Vec<KEvent>
}

impl KEventDispatcherControl {
    fn new() -> KEventDispatcherControl {
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
        events.push(KEvent::Mouse(KEventMouse::Moved((10,10))));
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

        KEventDispatcherControl { events }
    }

    pub fn get_event(&mut self) -> KEvent {
        match self.events.pop(){
            Some(event) => event,
            None => KEvent::Unknown,
        }
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
    fn handle_event(&mut self, event : &KEvent) -> bool {
        
        // Increment notifications
        self.notification_count += 1;
        
        match event {
            KEvent::Unknown => panic!("Error : Unknown event received!"),

            KEvent::Window(_) => self.handle_window,
            KEvent::Keyboard(_) => self.handle_keyboard,
            KEvent::Mouse(_) => self.handle_mouse,
            KEvent::Controller(_) => self.handle_controller,
            KEvent::None => panic!("Error : Unknown event received!"),            
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}