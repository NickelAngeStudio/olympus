use std::{cell::RefCell, rc::Rc};

use olympus::kleio::window::{KWindow, KEventReceiver, KEvent};

use crate::assert_ok;

#[test]
#[ignore]
/// Temporary test to print event types.
fn kwindow_x11_temp() {

    let mut w1 = assert_ok!(KWindow::new(100,100,640,480));
    let mut w2 = assert_ok!(KWindow::new(100,100,640,480));
    let rc1 = Rc::new(RefCell::new(KEventReceiverControl::new()));
    let rc2 = Rc::new(RefCell::new(KEventReceiverControl::new()));

    assert_ok!(w1.add_event_receiver(rc1.clone()));
    assert_ok!(w2.add_event_receiver(rc2.clone()));

    loop {
        match w1.dispatch_events() {
            Ok(_) => {},
            Err(_) => panic!("Dispatch error!"),
        }

        match w2.dispatch_events() {
            Ok(_) => {},
            Err(_) => panic!("Dispatch error!"),
        }
    }
}



/// ##### Control receiver that count how many time it was notified.
struct KEventReceiverControl {

    /// Is the receiver enabled or not.
    enabled : bool,
}

impl KEventReceiverControl {
    /// Create a new instance of KEventReceiverHollow which handle event or not.
    /// 
    /// Returns new KEventReceiverHollow created.
    pub fn new() -> KEventReceiverControl {
        KEventReceiverControl { enabled: true }
    }

    /// Set if the receiver is enabled or not.
    pub fn set_enabled(&mut self, enabled : bool){
        self.enabled = enabled;
    }
}

impl KEventReceiver for KEventReceiverControl {
    fn handle_event(&mut self, event : &KEvent) -> bool {
        
       match event {
        KEvent::Unknown => println!("Event={:?}", event),
        KEvent::Window(event) => println!("Event={:?}", event),
        KEvent::Keyboard(event) => match event {
            olympus::kleio::window::KEventKeyboard::KeyDown(code) => {
                if *code == 53 {
                    std::process::exit(0);
                } else {
                    println!("Event={:?}", event)
                }
            },
            olympus::kleio::window::KEventKeyboard::KeyUp(_) => println!("Event={:?}", event),
        },
        KEvent::Mouse(event) => match event {
            olympus::kleio::window::KEventMouse::Moved(_) => (),
            _ => println!("Event={:?}", event),

        },
        KEvent::Controller(event) => println!("Event={:?}", event),
    }
        

        true
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

