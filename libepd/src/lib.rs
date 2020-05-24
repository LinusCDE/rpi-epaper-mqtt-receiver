#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//#[allow(unused_imports)]
//use libc::*;

use crate::DOT_PIXEL::DOT_PIXEL_1X1;
use crate::DOT_STYLE::DOT_FILL_AROUND;
use crate::MIRROR_IMAGE::MIRROR_NONE;

// From GUI_Paint.h:
pub const WHITE: u16 = 0xFF;
pub const BLACK: u16 = 0x00;
pub const RED: u16 = BLACK;
pub const ROTATE_0: u16 = 0;
pub const ROTATE_90: u16 = 90;
pub const ROTATE_180: u16 = 180;
pub const ROTATE_270: u16 = 270;
pub const GRAY1: u16 = 0x03; //Blackest
pub const GRAY2: u16 = 0x02;
pub const GRAY3: u16 = 0x01; //gray
pub const GRAY4: u16 = 0x00; //white
pub const DOT_PIXEL_DFT: DOT_PIXEL = DOT_PIXEL_1X1;  //Default dot pilex
pub const DOT_STYLE_DFT: DOT_STYLE = DOT_FILL_AROUND;  //Default dot pilex

pub enum MIRROR_IMAGE {
    MIRROR_NONE = 0x00,
    MIRROR_HORIZONTAL = 0x01,
    MIRROR_VERTICAL = 0x02,
    MIRROR_ORIGIN = 0x03,
}
pub const MIRROR_IMAGE_DFT: MIRROR_IMAGE = MIRROR_NONE;

include!("./bindings.rs");