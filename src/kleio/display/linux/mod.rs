use super::{ KWindowError, event::KEvent, KWindow, screen::KScreenList, KCursor };
use server::KLinuxDisplayServerProvider;

/// Wayland KWindowManager
#[cfg(all(not(git_workflow), not(feature="no_wayland")))]     // Add Wayland if not remove via feature.
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
        // With Wayland support
        #[cfg(all(not(git_workflow), not(feature="no_wayland")))] 
        {
            match $provider {
                KLinuxDisplayServerProvider::Wayland => $if_wayland,
                _ => $else,
            }
        }
        // Without Wayland support
        #[cfg(all(not(git_workflow), feature="no_wayland"))]
        {
            $else
        }
    }
}


/// Implementation of privates elements relatives to linux distributions
#[doc(hidden)]
impl KWindow {
    /// Create new KWindow
    pub(super) fn __new(width:u32, height:u32, provider : super::linux::server::KLinuxDisplayServerProvider) -> Result<KWindow, KWindowError> {
        // Default cursor.
        let cursor = KCursor { mode: super::KCursorMode::Pointer, position: (0,0), visible: true, confined: false };
        
        // Default center position
        let center = ((width as i32 / 2), (height as i32 / 2));

        match  super::linux::server::KLinuxDisplayServer::new(width, height, provider){
            Ok(display_server) => {
                match KScreenList::new(display_server.provider){
                    Ok(screen_list) => {
                        let mut kwindow = KWindow { screen_list, cursor, size : (width, height), center, display_server };
                        Ok(kwindow)
                    },
                    Err(_) => Err(KWindowError::ScreenDetailError),
                }
            },
            Err(err) => Err(err),
        }  
    }
        
    // Get cursor position
    #[inline(always)]
    pub(super) fn __get_cursor_position(&self) -> (i32, i32){
        wayland_or_x11!{self.display_server.provider, { 
                self.wayland_get_cursor_position() 
            }, { 
                self.x11_get_cursor_position() 
            }
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
    pub fn __hide_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_hide_cursor();
            
            }, { 
                self.x11_hide_cursor();
            }
        }
    }

    /// Show system default cursor.
    #[inline(always)]
    pub fn __show_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_show_cursor();
            
            }, { 
                self.x11_show_cursor();
            }
        }
    }

    /// Confine cursor to window, preventing it from exiting boundaries.
    #[inline(always)]
    pub fn __confine_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_confine_cursor();
            
            }, { 
                self.x11_confine_cursor();
            }
        }
    }


    /// Release cursor from window, allowing it to exit boundaries.
    #[inline(always)]
    pub fn __release_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_release_cursor();
            
            }, { 
                self.x11_release_cursor();
            }
        }
    }


    

}


/*
/// Abstraction of Linux display server
pub trait KLinuxDisplayServer  {

    /// Return true if display server is compatible with current linux distro.
    fn is_compatible() -> bool where Self:Sized;

    /// Pop an event from the queue. 
    /// 
    /// Warning(s)
    /// Will lock until next event if no events.
    fn pop_event(&mut self) -> KEvent;

    /// Returns count of x11 events.
    fn get_event_count(&self) -> usize;

    /// Sync all event between client and display server / window manager. 
    fn sync_events(&self);

    /// Get the display server provider identification.
    fn get_display_server_provider(&self) -> LinuxDisplayServerProvider;

    /// Get the display server connection.
    fn get_display_server_connection(&self) -> *const Display;
    
    /// Get the display server window handle.
    fn get_display_server_window(&self) -> *const Window;

    /// Set the cursor position with a pair (x,y).
    fn set_cursor_position(&mut self, position : (i32, i32), size : (u32, u32));

    /// Get the cursor position with as a pair (x,y).
    fn get_cursor_position(&self) -> (i32, i32);

}

/// Get the appropriate linux display server. 
/// 
/// If provider is set to default, Will try to open Wayland first then X11.
/// 
/// Returns Ok(Box<dyn KLinuxDisplayServer>) if successful.
/// 
/// # Error(s)
/// Returns Err([KWindowError::NoDisplayServer]) if no compatible display server found.
pub fn get_linux_display_server(width:u32, height:u32, provider : LinuxDisplayServerProvider) -> Result<Box<dyn KLinuxDisplayServer>, KWindowError> {
        
        use x11::X11DisplayServer;
        

        #[cfg(all(not(git_workflow), not(feature="no_wayland")))] 
        {
            use wayland::WaylandDisplayServer;

            match provider {
                LinuxDisplayServerProvider::Default => // Try Wayland first then X11
                    if WaylandDisplayServer::is_compatible() {
                        Ok(Box::new(WaylandDisplayServer::new(width, height)))
                    } // Else use X11 display server
                    else if X11DisplayServer::is_compatible() {
                        Ok(Box::new(X11DisplayServer::new(width, height)))
                    } // Return error of NoDisplayServer
                    else {
                        Err(KWindowError::NoDisplayServer)
                    },
                LinuxDisplayServerProvider::Wayland => // Only try Wayland.
                    if WaylandDisplayServer::is_compatible() {
                        Ok(Box::new(WaylandDisplayServer::new(width, height)))
                    } else {
                        Err(KWindowError::NoDisplayServer)
                    },
                LinuxDisplayServerProvider::X11 =>  // Only try x11.
                    if X11DisplayServer::is_compatible() {
                    Ok(Box::new(X11DisplayServer::new(width, height)))
                    } // Return error of NoDisplayServer
                    else {
                        Err(KWindowError::NoDisplayServer)
                    },
            }
        }

        #[cfg(any(feature="git_workflow", feature="no_wayland"))]
        {
            if X11DisplayServer::is_compatible() {
                Ok(Box::new(X11DisplayServer::new(width, height)))
            } // Return error of NoDisplayServer
            else {
                Err(KWindowError::NoDisplayServer)
            }
        }
}
*/