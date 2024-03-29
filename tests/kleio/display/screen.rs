use olympus::kleio::display::{screen::{KScreenList, KScreen, KScreenResolution} };

use crate::{ assert_ok};

/*********
* CONSTS *
*********/


/********
* TESTS *
********/
#[test]
#[ignore]   // Ignored by default since it required a screen to test.
/// Create a new KScreenList and test fields values.
/// 
/// # Verification(s)
/// V1 | KScreenList::new() create KScreenList without error.
/// V2 | Function test_kscreen_list_fields doesn't fail asserts.
fn kscreen_list_new() {

    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    {
        // V1 | KScreenList::new() create KScreenList without error. X11
        let kl = assert_ok!(KScreenList::new(olympus::kleio::display::linux::server::KLinuxDisplayServerProvider::X11));
        print_screen_list(&kl);

        // V2 | Function test_kscreen_list_fields doesn't fail asserts.
        test_kscreen_list_fields(&kl);

        /*
        TODO:Disabled until Wayland implementation
        let kl = assert_ok!(KScreenList::new(olympus::kleio::display::linux::LinuxDisplayServerProvider::Wayland));
        print_screen_list(&kl);
        test_kscreen_list_fields(&kl);
        */
    }

    /*
    TODO:Disabled until Windows and MacOS implementation
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "windows", target_os = "macos"))))]
    {
        // V1 | KScreenList::new() create KScreenList without error.
        let kl = assert_ok!(KScreenList::new());
        print_screen_list(&kl);

        // V2 | Function test_kscreen_list_fields doesn't fail asserts.
        test_kscreen_list_fields(&kl);
    }
    */

}

/************
* FUNCTIONS *
************/
/// Test KScreenList, KScreen and KScreenResolution fields value.
/// 
/// # Verification(s)
/// V1 | KScreenList::get_desktop_width() get_desktop_height() > 0.
/// V2 | KScreenList::get_screen_list().len() > 0.
/// V3 | KScreenList::get_primary_screen() != None.
/// V4 | KScreen::get_identifier() is not an empty string.
/// V5 | KScreen::get_current_resolution() width and height > 0.
/// V6 | KScreen::get_current_refresh_rate() > 0.
/// V7 | KScreen::get_supported_resolutions().len() > 0.
/// V8 | KScreenResolution::get_width() and get_height() > 0.
/// V9 | KScreenResolution::get_refresh_rates().len() > 0.
/// V10 | All KScreenResolution refresh rates > 0.
fn test_kscreen_list_fields(kl : &KScreenList) {

    // V1 | KScreenList::get_desktop_width() get_desktop_height() > 0.
    assert!(kl.get_desktop_width() > 0, "Desktop width should be > 0!");
    assert!(kl.get_desktop_height() > 0, "Desktop height should be > 0!");

    // V2 | KScreenList::get_screen_list().len() > 0.
    assert!(kl.get_screen_list().len() > 0, "KScreenList should have >=1 screen!");

    // V3 | KScreenList::get_primary_screen() != None.
    match kl.get_primary_screen(){
        Some(_) => {},
        None => panic!("No primary screen found!"),
    }

    for screen in kl.get_screen_list() {
        // V4 | KScreen::get_identifier() is not an empty string.
        assert_ne!(screen.get_identifier(), "", "Screen identifier must not be an empty string!");

        // V5 | KScreen::get_current_resolution() width and height > 0.
        assert!(screen.get_current_resolution().0 > 0, "Current screen width should be > 0!");
        assert!(screen.get_current_resolution().1 > 0, "Current screen height should be > 0!");

        // V6 | KScreen::get_current_refresh_rate() > 0.
        assert!(screen.get_current_refresh_rate() > 0, "Current screen refresh rate should be > 0!");

        // V7 | KScreen::get_supported_resolutions().len() > 0.
        assert!(screen.get_supported_resolutions().len() > 0, "Current screen supported resolution should be >=1!");

        for res in screen.get_supported_resolutions() {
            // V8 | KScreenResolution::get_width() and get_height() > 0.
            assert!(res.get_width() > 0, "Supported resolution width should be > 0!");
            assert!(res.get_height() > 0, "Supported resolution height should be > 0!");

            // V9 | KScreenResolution::get_refresh_rates().len() > 0.
            assert!(res.get_refresh_rates().len() > 0, "Supported resolution should have >=1 refresh rate!");

            for rf in res.get_refresh_rates(){
            // V10 | All KScreenResolution refresh rates > 0.
                assert!(*rf > 0, "Supported resolution refresh rate should be > 0!");
            }

        }
    }




}


/// Print the screen list to console.
fn print_screen_list(kl : &KScreenList){
    println!("*******************");
    println!("*** Screen List ***");
    println!("*******************");
    println!("Desktop width={}, height={}", kl.get_desktop_width(), kl.get_desktop_height());

    for screen in kl.get_screen_list() {
        print_screen(screen);
    }
}

/// Print the screen to console.
pub fn print_screen(screen : &KScreen) {
    println!("\nScreen {:?} [{}x{}:{}+{}+{}] primary={}", screen.get_identifier(), screen.get_current_resolution().0, 
        screen.get_current_resolution().1, screen.get_current_refresh_rate(), screen.get_extended_position().0, 
        screen.get_extended_position().1, screen.is_primary());

    for res in screen.get_supported_resolutions() {
        print_screen_resolution(res);
    }

}

 /// Print the screen to console.
 pub fn print_screen_resolution(res : &KScreenResolution) {
    print!("{}x{}", res.get_width(), res.get_height());

    for rf in res.get_refresh_rates() {
        print!(" {}", rf);
    }

    print!("\n");

}