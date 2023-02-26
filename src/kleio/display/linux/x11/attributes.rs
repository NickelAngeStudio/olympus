use std::os::raw::{ c_int };

/// XWindowAttributes fetcher with size needed to get everything (should be around 136 bytes, is padded to 156 to be sure)
pub(crate) struct XWindowAttributes {
    pub x : c_int,
    pub y : c_int,
    pub width : c_int, 
    pub height : c_int,
    pub padding : [u8;140]
}

impl XWindowAttributes {
    pub fn new() -> XWindowAttributes {
        XWindowAttributes { x: 0, y: 0, width: 0, height: 0, padding: [0;140] }
    }
}
