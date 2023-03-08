use crate::error::{OlympusError, KWindowError};

use self::x11::{attributes::XWindowAttributes, bind::XGetWindowAttributes};

use super::{ event::KEvent, KWindow, screen::KScreenList, KCursorProperty, KWindowProperty, KWindowFullscreenMode };
use debug_print::debug_println;
use server::KLinuxDisplayServerProvider;

/// Wayland KWindowManager
pub mod wayland;

/// X11 KWindowManager
pub mod x11;

/// Linux display server details
pub mod server;


/// Macro shortcut to execute either wayland or x11 function.
#[doc(hidden)]
#[macro_export]
macro_rules! wayland_or_x11 {
    ($provider:expr, $if_wayland:block, $else:block) => {
        match $provider {
            KLinuxDisplayServerProvider::Wayland => $if_wayland,
            _ => $else,
        }
    }
}


/// Implementation of privates elements relatives to linux distributions
#[doc(hidden)]
impl KWindow {
    /// Create new KWindow
    pub(super) fn __new(width:u32, height:u32, provider : super::linux::server::KLinuxDisplayServerProvider) -> Result<KWindow, OlympusError> {
        // Default cursor.
        let cursor = KCursorProperty { mode: super::KCursorMode::Pointer, position: (0,0), visible: true, confined: false };
        
        // Default center position
        let center = ((width as i32 / 2), (height as i32 / 2));

        match  super::linux::server::KLinuxDisplayServer::new(width, height, provider){
            Ok(display_server) => {
                match KScreenList::new(display_server.provider){
                    Ok(screen_list) => {
                        let mut property = KWindowProperty { title : String::from(""), cursor, position: (0,0), size: (width, height), center, minimized: false, maximized: false, fullscreen: false };
                        match display_server.provider {     // Fetch window position according to provider
                            KLinuxDisplayServerProvider::Wayland => todo!(),
                            KLinuxDisplayServerProvider::X11 => {
                                // Set correct x11 window position
                                property.position = KWindow::get_x11_window_position(display_server.display, display_server.window);
                                Ok(KWindow { screen_list, property, display_server })
                            },
                            _ => Err(OlympusError::KWindow(KWindowError::NoDisplayServer)),
                        }
                    },
                    Err(_) => Err(OlympusError::KWindow(KWindowError::ScreenDetailError)),
                }
            },
            Err(err) => Err(err),
        }  
    }
        

    // Pop an event from the queue
    #[inline(always)]
    pub(super) fn __poll_event(&mut self) -> KEvent {
        wayland_or_x11!{self.display_server.provider, { 
                self.wayland_poll_event() 
            }, { 
                self.x11_poll_event() 
            }
        }
    }

    // Sync an event from the queue
    #[inline(always)]
    pub(super) fn __sync_events(&self) {
        wayland_or_x11!{self.display_server.provider, { 
                self.wayland_sync_events();
            }, { 
                self.x11_sync_events();
            }
        }
    }

    /// Get the count of events that need handling.
    #[inline(always)]
    pub(super) fn __get_event_count(&self) -> usize {
        wayland_or_x11!{self.display_server.provider, { 
                self.wayland_get_event_count() 
            }, { 
                self.x11_get_event_count() 
            }
        }
    }

    /// Set the cursor position
    #[inline(always)]
    pub(super) fn __set_cursor_position(&mut self, position : (i32, i32)){
        wayland_or_x11!{self.display_server.provider, { 
                self.wayland_set_cursor_position(position);
                
            }, { 
                self.x11_set_cursor_position(position);
            }
        }
    }

    /// Hide system default cursor.
    #[inline(always)]
    pub(super) fn __hide_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_hide_cursor();
            
            }, { 
                self.x11_hide_cursor();
            }
        }
    }

    /// Show system default cursor.
    #[inline(always)]
    pub(super) fn __show_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_show_cursor();
            
            }, { 
                self.x11_show_cursor();
            }
        }
    }

    /// Confine cursor to window, preventing it from exiting boundaries.
    #[inline(always)]
    pub(super) fn __confine_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_confine_cursor();
            
            }, { 
                self.x11_confine_cursor();
            }
        }
    }


    /// Release cursor from window, allowing it to exit boundaries.
    #[inline(always)]
    pub(super) fn __release_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_release_cursor();
            
            }, { 
                self.x11_release_cursor();
            }
        }
    }


    /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
    #[inline(always)]
    pub(super) fn __restore(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_restore();
            
            }, { 
                self.x11_restore();
            }
        }
    }

    /// Set a new title for the [KWindow].
    #[inline(always)]
    pub(super) fn __set_title(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_set_title();
            
            }, { 
                self.x11_set_title();
            }
        }
    }

    /// Set a size of [KWindow].
    #[inline(always)]
    pub(super) fn __set_size(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_set_size();
            
            }, { 
                self.x11_set_size();
            }
        }
    }

    /// Set a position of [KWindow].
    #[inline(always)]
    pub(super) fn __set_position(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_set_position();
            
            }, { 
                self.x11_set_position();
            }
        }
    }

    /// Set the [KWindow] as fullscreen.
    #[inline(always)]
    pub(super) fn __set_fullscreen(&mut self, mode : KWindowFullscreenMode) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_set_fullscreen(mode);
            
            }, { 
                self.x11_set_fullscreen(mode);
            }
        }
    }
    

}