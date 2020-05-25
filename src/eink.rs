use libepd::*;
use libepd::MIRROR_IMAGE::MIRROR_HORIZONTAL;
use std::ffi::CString;
use libepd::DOT_PIXEL::{DOT_PIXEL_2X2, DOT_PIXEL_1X1, DOT_PIXEL_3X3, DOT_PIXEL_4X4, DOT_PIXEL_5X5, DOT_PIXEL_6X6, DOT_PIXEL_7X7, DOT_PIXEL_8X8};
use libepd::LINE_STYLE::LINE_STYLE_SOLID;
use crate::eink::DisplayMode::{Full, Partial};
use crate::eink::Color::{White, Black};
use serde::{Deserialize, Serialize, Serializer, Deserializer};

pub enum DisplayMode {
    Full,
    Partial
}

pub struct EInk {
    display_mode: DisplayMode,
    width: u16,
    height: u16,
}

pub enum Color {
    Black = BLACK as isize,
    White = WHITE as isize,
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        match self {
            White => serializer.serialize_str("white"),
            Black => serializer.serialize_str("black")
        }
    }
}

/*
impl Deserialize<_> for Color {
    fn deserialize<'de, D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let string: String = (deserializer.into() as String).to_lowercase();
        if string == "white" { Ok(White) }
        else if string == "black" { Ok(Black) }
        else { Err(deserializer) }
    }
}*/

pub struct Pos {
    pub x: u16,
    pub y: u16,
}

impl EInk {
    pub fn new(base_color: Color) -> EInk {
        unsafe {
            if DEV_Module_Init() != 0 {
                panic!("Failed to initialize device!");
            }

            EPD_2IN13_V2_Init(EPD_2IN13_V2_FULL);
            EPD_2IN13_V2_Clear();
            DEV_Delay_ms(500);

            LIBEPD_Init2in32Image();
            Paint_SetMirroring(MIRROR_HORIZONTAL as u8);
            let eink = EInk {
                display_mode: Full,
                width: EPD_2IN13_V2_HEIGHT,
                height: EPD_2IN13_V2_WIDTH
            };
            eink.clear(base_color, None, None);
            eink.display();
            return eink;
        }
    }

    pub fn to_partial_mode(&mut self) -> bool {
        if let Partial = self.display_mode {
            return false;
        }

        unsafe {
            EPD_2IN13_V2_Init(EPD_2IN13_V2_FULL);
            LIBEPD_DisplayPartialBase();
            EPD_2IN13_V2_Init(EPD_2IN13_V2_PART);
            LIBEPD_SelectImage();
        }
        self.display_mode = Partial;
        return true;
    }

    pub fn to_full_mode(&mut self) -> bool {
        if let Full = self.display_mode {
            return false;
        }

        unsafe {
            EPD_2IN13_V2_Init(EPD_2IN13_V2_FULL);
            LIBEPD_SelectImage();
        }
        self.display_mode = Full;
        return true;
    }

    pub fn display(&self) {
        match self.display_mode {
            Full => unsafe { LIBEPD_Display() },
            Partial => unsafe { LIBEPD_DisplayPartial() },
        }
    }

    pub fn clear(&self, color: Color, from: Option<Pos>, to: Option<Pos>) {
        if from.is_none() && to.is_none() {
            unsafe { Paint_Clear(color as u16); }
        } else {
            let from = from.unwrap_or(Pos { x: 0, y: 0 });
            let to = to.unwrap_or(Pos { x: self.get_width(), y: self.get_height() });
            unsafe {
                Paint_ClearWindows(from.x, from.y, to.x, to.y, color as u16);
            }
        }
    }

    pub fn delay(&self, millis: u32) {
        unsafe {
            DEV_Delay_ms(millis)
        }
    }

    pub fn draw_line(&self, from: Pos, to: Pos, color: Color, thickness: u8) {
        let line_width = match thickness {
            1 => DOT_PIXEL_1X1,
            2 => DOT_PIXEL_2X2,
            3 => DOT_PIXEL_3X3,
            4 => DOT_PIXEL_4X4,
            5 => DOT_PIXEL_5X5,
            6 => DOT_PIXEL_6X6,
            7 => DOT_PIXEL_7X7,
            8 => DOT_PIXEL_8X8,
            _ => DOT_PIXEL_1X1,
        };

        unsafe {
            Paint_DrawLine(from.x, from.y, to.x, to.y, color as u16, line_width, LINE_STYLE_SOLID);
        }
    }

    pub fn draw_string(&self, pos: Pos, text: &str, font: u8, color: Color) {
        unsafe {
            let mut s_font = match font {
                1 => Font8,
                2 => Font12,
                3 => Font16,
                4 => Font20,
                5 => Font24,

                8 => Font8,
                12 => Font12,
                16 => Font16,
                20 => Font20,
                24 => Font24,

                _ => Font8,
            };

            let c_string = CString::new(text).unwrap();
            // Color(fg/bg) is inverted in function call for some reason
            let fg = match color {
                Black => WHITE,
                White => BLACK,
            };
            let bg = match color {
                Black => BLACK,
                White => WHITE,
            };
            Paint_DrawString_EN(pos.x, pos.y,  c_string.as_ptr() as *const _, &mut s_font, fg, bg);
        }
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }
}