use std::process::exit;

use olympus::kleio::display::event::KEventReceiver;

// Contains tests for screen module
#[cfg(test)]
pub mod screen;

// Contains tests for KEventDispatcher
#[cfg(test)]
pub mod dispatcher;

// Contains tests for X11 KWindow Linux Display Server
#[cfg(all(not(target_family = "wasm"), target_os = "linux"))]
pub mod x11;



/*********
* MACROS *
*********/
/// Macro used to create KWindow, dispatcher and control receiver.
/// Keys : Next=Space, Exit=ESC
#[macro_export]
macro_rules! kwindow_x11_prepare {

    // This macro call doesn't create KAssetSourceFolder nor the files
    ($kwindow:ident, $dispatcher:ident, $receiver:ident, $test_body:block) => {{
        // Create KWindow
        #[allow(unused_mut)]
        let mut $kwindow = assert_ok!(KWindow::new(KWINDOW_WIDTH, KWINDOW_HEIGHT, LinuxDisplayServerProvider::X11));

        // Create dispatcher
        #[allow(unused_mut)]
        let mut $dispatcher = KEventDispatcher::new(true);

        // Create and add receiver to dispatcher
        let $receiver = Rc::new(RefCell::new(KEventReceiverControl::new(65, 9)));
        match $dispatcher.add_event_receiver($receiver.clone()){
            Ok(_) => {},
            Err(_) => panic!("Receiver error!"),
        }

        // Test body
        $test_body

        // Last wait loop
        kwindow_x11_step_loop!($kwindow, $dispatcher, $receiver);
    }};
}

/// Loop until next step key is pressed.
#[macro_export]
macro_rules! kwindow_x11_step_loop {

    // This macro call doesn't create KAssetSourceFolder nor the files
    ($kwindow:ident, $dispatcher:ident, $receiver:ident) => {{
        loop {
            $kwindow.dispatch_events(&mut $dispatcher);
            match $receiver.borrow().get_state() {
                crate::kleio::display::KEventReceiverControlState::Running => {},
                crate::kleio::display::KEventReceiverControlState::NextStep => break,
                crate::kleio::display::KEventReceiverControlState::Exit => exit(0),
            }
        }
    }};
}

/*********
* STRUCT *
*********/
/// Enumeration of KEventReceiverControlResult state.
#[derive(Debug, Clone, Copy)]
enum KEventReceiverControlState {

    // Test is running
    Running,

    // Tell receiver to get to next step
    NextStep,

    // Exit program
    Exit,
}


/// Receiver used to control test since all tests are runned manually.
/// 
/// Pressing step_key to exit step loop.
/// Pressing Esc will exit the program.
struct KEventReceiverControl {
    /// Key to press for next step
    step_key : u32,

    /// Key to press to exit
    exit_key : u32,


    /// State
    state : KEventReceiverControlState,
}

impl KEventReceiverControl {
    pub fn new(step_key : u32, exit_key : u32) -> KEventReceiverControl {
        KEventReceiverControl { step_key, exit_key, state : KEventReceiverControlState::Running }
    }

    pub fn get_state(&self) -> KEventReceiverControlState {
        self.state
    }

    pub fn set_state(&mut self, state : KEventReceiverControlState)  {
        self.state = state
    }
}

impl KEventReceiver for KEventReceiverControl {
    fn handle_event(&mut self, event : &olympus::kleio::display::event::KEvent) -> bool {
        match event {
            olympus::kleio::display::event::KEvent::Keyboard(event) => match event {
            olympus::kleio::display::event::KEventKeyboard::KeyDown(keycode) => 
                {
                    self.state = KEventReceiverControlState::Running;

                    println!("KeyCode={}", keycode);
                    if *keycode == self.step_key {
                        self.state = KEventReceiverControlState::NextStep;
                    }


                    if *keycode == self.exit_key {
                        self.state = KEventReceiverControlState::Exit;
                    }
                    false
                },
                olympus::kleio::display::event::KEventKeyboard::KeyUp(_) => { false },
            },
            _ => false,
        }
    }

    fn is_enabled(&self) -> bool {
        true
    }
}