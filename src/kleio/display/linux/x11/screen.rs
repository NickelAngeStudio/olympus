use crate::kleio::display::screen::{KScreen, KScreenListError, KScreenResolution};

/// Private struct used to fetch screen details
struct ScreenDetail {

    // Flag to know if we are in resolution details or not.
    pub is_resolution_detail : bool,

    // Desktop screen width and height
    pub desktop_width : u32,
    pub desktop_height : u32,

    // Screen details
    pub identifier : String,
    pub height : u32,
    pub width : u32,
    pub refresh_rate : u32,
    pub primary : bool,
    pub resolution : Vec<KScreenResolution>,

    // Screen resolution details
    pub res_width : u32,
    pub res_height : u32,
}

impl ScreenDetail {
    pub fn new() -> ScreenDetail {
        ScreenDetail { is_resolution_detail : false, desktop_width: 0, desktop_height: 0, identifier: String::from(""), 
        height: 0, width: 0, refresh_rate: 0, primary: false, resolution: Vec::new(), res_width: 0, res_height: 0 }
    }
}

/// Ugly function that fetch x11 display server screens.
pub(crate) fn get_x11_screen() -> Result<(u32,u32,Vec<KScreen>), KScreenListError>{
    use std::process::Command;

    // xrandr command list monitors with current resolution, available resolution and refresh rates.
    let cmd = Command::new("sh")
                .arg("-c")
                .arg("xrandr")
                .output();
            
    match cmd {
        Ok(output) => {
            let out_str = String::from_utf8(output.stdout);

            match out_str {
                Ok(out_str) => {
                    let mut screens : Vec<KScreen> = Vec::new();

                    // Used to get screen details
                    let mut sd = ScreenDetail::new();


                    for line in out_str.split("\n") {
                        // Remove all double spaces
                        let line_trimmed =  &line.replace("  ", " ");
                        let line_trimmed =  &line_trimmed.replace("  ", " ");
                        let line_trimmed =  &line_trimmed.replace("  ", " ");
                        let line_trimmed = line_trimmed.trim();
                        
                        screen_section_end(line_trimmed, &mut screens, &mut sd);

                        match scan_connected_screen(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }

                        match scan_current_screen(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }

                        match scan_screen_supported_resolutions(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }
                    }
                    if sd.is_resolution_detail  {
                        // Section ended, add screen to list
                        screens.push(KScreen::new( sd.identifier.clone(), sd.height, sd.width, sd.refresh_rate, 
                        sd.primary, sd.resolution.clone() ));
                    }

                    Ok((sd.desktop_width,sd.desktop_height,screens))
                },
                Err(_) => Err(KScreenListError::FetchScreenListError),
            }
        },
        Err(_) => Err(KScreenListError::FetchScreenListError),
    }
}

/// End of screen section
fn screen_section_end(line_trimmed:&str, screens : &mut Vec<KScreen>, sd: &mut ScreenDetail){
    if sd.is_resolution_detail  {
        // Scan for section end
        if line_trimmed.contains("connected") {
            // Section ended, add screen to list
            screens.push(KScreen::new( sd.identifier.clone(), sd.height, sd.width, sd.refresh_rate, 
                sd.primary, sd.resolution.clone() ));

            sd.is_resolution_detail = false;
        }
    }
}

/// Scan for connected screens
fn scan_connected_screen(line_trimmed:&str, sd: &mut ScreenDetail) -> Result<u8, KScreenListError>{
    // Scan for connected monitor
    if line_trimmed.contains("connected") && !line_trimmed.contains("disconnected") {
        let mut line_split = line_trimmed.split(" ");
        
        // Get identifier
        match line_split.next(){
            Some(id) => sd.identifier = String::from(id),
            // If no id, return error
            None => return Err(KScreenListError::FetchScreenListError),
        }

        // Skip connected keyword
        line_split.next();

        // Get if primary screen
        if line_trimmed.contains("primary") {
            sd.primary = true;
            // Skip primary keyword
            line_split.next();
        } else {
            sd.primary = false;
        }

        // Get width and height
        match line_split.next() {
            Some(res) => {
                let mut res = res.split("x");

                // Get width
                match res.next() {
                    Some(w) => match w.parse::<u32>() {
                        Ok(w) => sd.width = w,
                        // If width conversion error, return error
                        Err(_) => return Err(KScreenListError::FetchScreenListError),
                    },
                    // If no width, return error
                    None => return Err(KScreenListError::FetchScreenListError),
                }

                // Get height
                match res.next() {
                    Some(h) => {
                        let mut h = h.split("+");

                        match h.next() {
                            Some(h) => match h.parse::<u32>() {
                                Ok(h) => sd.height = h,
                                // If height conversion error, return error
                                Err(_) => return Err(KScreenListError::FetchScreenListError),
                            },
                            None => return Err(KScreenListError::FetchScreenListError),
                        }
                    },
                    None => return Err(KScreenListError::FetchScreenListError),
                }
            },
            // If no resolution, return error
            None => return Err(KScreenListError::FetchScreenListError),
        }

        sd.refresh_rate = 0;
        sd.resolution = Vec::new();
        sd.is_resolution_detail = true;
    }

    Ok(0)
}

/// Scan current screen to get desktop resolution
fn scan_current_screen(line_trimmed:&str, sd: &mut ScreenDetail) -> Result<u8, KScreenListError>{
    if line_trimmed.contains("current") {   // desktop screen details
        let mut line_split = line_trimmed.split(" ");

        loop {
            match line_split.next() {
                Some(part) => if part.contains("curr") {
                    break;  // We reached current, exit loop
                },
                None => return Err(KScreenListError::FetchScreenListError),
            }
        }

        // Get width
        match line_split.next() {
            Some(w) => match w.parse::<u32>() {
                Ok(w) => sd.desktop_width = w,
                // If height conversion error, return error
                Err(_) => return Err(KScreenListError::FetchScreenListError),
            },
            None => return Err(KScreenListError::FetchScreenListError),
        }

        // Skip x
        line_split.next();

        // Get height
        match line_split.next() {
            Some(h) => match h.replace(",", "").parse::<u32>() {
                Ok(h) => sd.desktop_height = h,
                // If height conversion error, return error
                Err(_) => return Err(KScreenListError::FetchScreenListError),
            },
            None => return Err(KScreenListError::FetchScreenListError),
        }
    }

    Ok(0)
}

/// Scan screen supported resolutions
fn scan_screen_supported_resolutions(line_trimmed:&str, sd: &mut ScreenDetail) -> Result<u8, KScreenListError>{

    if !line_trimmed.contains("connected") && sd.is_resolution_detail { // Resolution details
        let mut line_split = line_trimmed.split(" ");


        // Get width and height
        match line_split.next() {
            Some(res) => {
                let mut res = res.split("x");

                // Get width
                match res.next() {
                    Some(w) => match w.parse::<u32>() {
                        Ok(w) => sd.res_width = w,
                        // If width conversion error, return error
                        Err(_) => return Err(KScreenListError::FetchScreenListError),
                    },
                    None => return Err(KScreenListError::FetchScreenListError),
                }

                // Get height
                match res.next() {
                    Some(h) => match h.parse::<u32>() {
                        Ok(h) => sd.res_height = h,
                        // If width conversion error, return error
                        Err(_) => return Err(KScreenListError::FetchScreenListError),
                    },
                    None => return Err(KScreenListError::FetchScreenListError),
                }
            },
            None => return Err(KScreenListError::FetchScreenListError),
        }

        let mut res_res = KScreenResolution::new(sd.res_width, sd.res_height);

        // To make sure we really get refresh counts.
        let mut rfcount = 0;

        // Get refresh rates
        loop {
            match line_split.next() {
                Some(rfp) => {
                    rfcount += 1;

                    // Remove extra characters
                    let rf = rfp.replace(".", "").replace("+", "").replace("*", "");

                    match rf.parse::<u32>() {
                        Ok(rf) => {
                            // Is it monitor refresh rate?
                            if rfp.contains("*"){
                                sd.refresh_rate = rf;
                            }

                            res_res.add_refresh_rate(rf);

                        },
                        // If width conversion error, return error
                        Err(_) => return Err(KScreenListError::FetchScreenListError),
                    };                                                
                },
                None => if rfcount > 0 {
                    break;
                } else {
                    // No refresh rate added, return error
                    return Err(KScreenListError::FetchScreenListError);
                },
            }
        }
        // Add to screen resolution
        sd.resolution.push(res_res.clone());

    }    

    Ok(0)
}