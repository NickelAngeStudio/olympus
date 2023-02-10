// The Linux KWindow will try to open a connection with Wayland first. If it failed, it will open a connection with X11 then.
// NOTE : If Wayland becomes DE FACTO standard, X11 support might be dropped.

/// Wayland KWindowManager
pub mod wayland;

/// X11 KWindowManager
pub mod x11;