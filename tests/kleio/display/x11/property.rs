/********
* TESTS *
********/
#[test]
/// Create a new X11 KWindow.
/// 
/// # Verification(s)
/// V1 | KWindow::new(x11) with width < KWINDOW_MIN_WIDTH and width > KWINDOW_MAX_WIDTH must return [KWindowError::WindowSizeError].
/// V2 | KWindow::new(x11) with height < KWINDOW_MIN_HEIGHT and height > KWINDOW_MAX_HEIGHT must return [KWindowError::WindowSizeError].
/// V2 | 
fn kwindow_x11_new() {
    
}