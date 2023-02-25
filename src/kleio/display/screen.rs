/// Contains list of all hardware display device.
pub struct KScreenList {
    /// Screens combined width
    width : u32,

    /// Screens combined height
    height : u32,

     /// List of screens
     screen_list : Vec<KScreen>,
}

/// Enumeration of possible [KScreenList] errors.
#[derive(Debug)]
pub enum KScreenListError {

    /// Occurs when trying to use default LinuxDisplayServerProvider as provider.
    DefaultLinuxDisplayProviderError,

    ///  Occurs when fetching the system screens details failed.
    FetchScreenListError,

}

impl KScreenList {

    /// Create a new screen list that contains the details of all screens for display provider.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "linux"))))]
    pub fn new(provider : super::linux::server::KLinuxDisplayServerProvider) -> Result<KScreenList, KScreenListError> {

        match provider {
            super::linux::server::KLinuxDisplayServerProvider::Default => Err(KScreenListError::DefaultLinuxDisplayProviderError),
            super::linux::server::KLinuxDisplayServerProvider::Wayland => {
                match super::linux::wayland::screen::get_wayland_screen() {
                    Ok(screen_list) => Ok(KScreenList { screen_list, width: 0, height: 0 }),
                    Err(err) => Err(err),
                }
            },
            super::linux::server::KLinuxDisplayServerProvider::X11 => {
                match super::linux::x11::screen::get_x11_screen() {
                    Ok(sl) => Ok(KScreenList { screen_list : sl.2 , width:  sl.0, height: sl.1 }),
                    Err(err) => Err(err),
                }
            },
        }
        
    }

    /// Create a new screen list that contains the details of all screens.
    #[cfg(any(doc, all(not(target_family = "wasm"), any(target_os = "windows", target_os = "macos"))))]
    pub fn new() -> Result<KScreenList, KScreenListError> {
        todo!()
    }

    /// Returns desktop multi-screen combined width.
    pub fn get_desktop_width(&self) -> u32 {
        self.width
    }

    /// Returns desktop multi-screen combined height.
    pub fn get_desktop_height(&self) -> u32 {
        self.height
    }

    /// Get primary screen reference.
    /// 
    /// Returns Some([KScreen]) with primary screen or None if no screen.
    pub fn get_primary_screen(&self) -> Option<&KScreen> {
        for screen in &self.screen_list {
            if screen.is_primary() {
                return Some(screen);
            }
        }
        None
    }

    /// Get a reference to the list of screens.
    pub fn get_screen_list(&self) -> &Vec<KScreen> {
        &self.screen_list
    }

}


/// Hardware display device details.
/// 
/// # Note(s)
/// Refresh rate is stored as unsigned integer. A 60hz refresh rate is 6000 and a 144hz is 14400. 
pub struct KScreen {
    /// Identifier of that screen
    identifier : String,

    /// Current width resolution of that screen
    width : u32,

    /// Current height resolution of that screen
    height : u32,

    /// Current refresh rate of that screen
    refresh_rate : u32,

    /// Is primary screen?
    primary : bool,

    /// Supported resolutions
    resolution : Vec<KScreenResolution>,
}

impl KScreen {
    /// Create a new [KScreen] with fields.
    pub fn new(identifier : String, height : u32, width : u32, refresh_rate : u32, primary : bool, resolution : Vec<KScreenResolution>) -> KScreen{
        KScreen { identifier, height, width, refresh_rate, primary, resolution }
    }

    /// Returns screen unique identifier as [String].
    pub fn get_identifier(&self) -> &String{
        &self.identifier
    }

    /// Returns current width resolution of screen.
    pub fn get_current_width(&self) -> u32 {
        self.width
    }

    /// Returns current height resolution of screen.
    pub fn get_current_height(&self) -> u32 {
        self.height
    }

    /// Returns current refresh rate of screen.
    /// 
    /// Note(s)
    /// Refresh rate is stored as unsigned integer. A 60hz refresh rate is 6000 and a 144hz is 14400. 
    pub fn get_current_refresh_rate(&self) -> u32 {
        self.refresh_rate
    }

    /// Returns True if screen is currently the primare default screen.
    pub fn is_primary(&self) -> bool{
        self.primary
    }

    /// Returns list of supported resolutions and refresh rates.
    pub fn get_supported_resolutions(&self) -> &Vec<KScreenResolution>{
        &self.resolution
    }
}

/// Hardware screen supported resolution with available refresh rate.
/// 
/// # Note(s)
/// Refresh rate is stored as unsigned integer. A 60hz refresh rate is 6000 and a 144hz is 14400. 
#[derive(Clone)]
pub struct KScreenResolution {
    width : u32,
    height : u32,
    refresh_rate : Vec<u32>,
}

impl KScreenResolution {
    /// Create a new [KScreenResolution] with fields.
    pub fn new(width : u32, height : u32) -> KScreenResolution {
        KScreenResolution { width, height, refresh_rate: Vec::new() }
    }

    /// Add a supported refresh rate to resolution.
    pub(crate) fn add_refresh_rate(&mut self, rate : u32){

        // Don't add if already exists
        for rf in &self.refresh_rate {
            if *rf == rate {
                return;
            }
        }

        self.refresh_rate.push(rate);
    }

    /// Returns width of supported screen resolution.
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Returns height of supported screen resolution.
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Returns supported refresh rates of this screen resolution.
    /// 
    /// # Note(s)
    /// Refresh rates are stored as unsigned integer. A 60hz refresh rate is 6000 and a 144hz is 14400. 
    pub fn get_refresh_rates(&self) -> &Vec<u32> {
        &self.refresh_rate
    }

   
}
