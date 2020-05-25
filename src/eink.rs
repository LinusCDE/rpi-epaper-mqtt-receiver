use crate::eink::DisplayMode::{Full, Partial};
use crate::eink::Color::{White, Black};
use libepd::*;
use libepd::MIRROR_IMAGE::MIRROR_HORIZONTAL;
use libepd::DOT_PIXEL::*;
use libepd::LINE_STYLE::LINE_STYLE_SOLID;
use libepd::DRAW_FILL::{DRAW_FILL_FULL, DRAW_FILL_EMPTY};
use libepd::DOT_STYLE::{DOT_FILL_AROUND, DOT_FILL_RIGHTUP};
use std::ffi::CString;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub enum DisplayMode {
    Full,
    Partial
}

pub struct EInk {
    display_mode: DisplayMode,
    width: u16,
    height: u16,
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[repr(u16)]
pub enum Color {
    Black = BLACK,
    White = WHITE,
}

#[derive(Deserialize, Debug, Copy, Clone)]
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

    pub fn refresh(&self) {
        if let Partial = self.display_mode {
            unsafe {
                EPD_2IN13_V2_Init(EPD_2IN13_V2_FULL);
                LIBEPD_DisplayPartialBase();
                EPD_2IN13_V2_Init(EPD_2IN13_V2_PART);
                LIBEPD_SelectImage();
            }
        }
        // No need for refresh in Full display mode
    }


    pub fn clear(&self, color: Color, from: Option<Pos>, to: Option<Pos>) {
        if from.is_none() && to.is_none() {
            unsafe { Paint_Clear(color as u16); }
        } else {
            let from = from.unwrap_or(Pos { x: 0, y: 0 });
            let to = to.unwrap_or(Pos { x: self.get_width()-1, y: self.get_height()-1 });
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

    fn thickness_to_dot_pixel(thickness: u8) -> Result<DOT_PIXEL, String> {
        match thickness {
            1 => Ok(DOT_PIXEL_1X1),
            2 => Ok(DOT_PIXEL_2X2),
            3 => Ok(DOT_PIXEL_3X3),
            4 => Ok(DOT_PIXEL_4X4),
            5 => Ok(DOT_PIXEL_5X5),
            6 => Ok(DOT_PIXEL_6X6),
            7 => Ok(DOT_PIXEL_7X7),
            8 => Ok(DOT_PIXEL_8X8),
            _ => Err(String::from("Thickness must be between 1 and 8")),
        }
    }

    pub fn draw_string(&self, pos: Pos, text: &str, font: u8, color: Color) -> Result<(), String> {
        unsafe {
            let mut s_font = match font {
                8 => Font8,
                12 => Font12,
                16 => Font16,
                20 => Font20,
                24 => Font24,

                _ => return Err(String::from("Font size can only be 8, 12, 16, 20 or 24!"))
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
        Ok(())
    }

    pub fn draw_line(&self, from: Pos, to: Pos, color: Color, thickness: u8) -> Result<(), String> {
        match EInk::thickness_to_dot_pixel(thickness) {
            Ok(line_width) => {
                unsafe {
                    Paint_DrawLine(from.x, from.y, to.x, to.y, color as u16, line_width, LINE_STYLE_SOLID);
                }
                Ok(())
            }
            Err(reason) => Err(reason)
        }
    }

    pub fn draw_rectangle(&self, from: Pos, to: Pos, color: Color, thickness: u8, filled: bool) -> Result<(), String> {
        match EInk::thickness_to_dot_pixel(thickness) {
            Ok(line_width) => {
                let fill_type = if filled { DRAW_FILL_FULL } else { DRAW_FILL_EMPTY };
                unsafe {
                    Paint_DrawRectangle(from.x, from.y, to.x, to.y, color as u16, line_width, fill_type);
                }
                Ok(())
            }
            Err(reason) => Err(reason)
        }
    }

    pub fn draw_circle(&self, pos: Pos, radius: u16, color: Color, thickness: u8, filled: bool) -> Result<(), String> {
        match EInk::thickness_to_dot_pixel(thickness) {
            Ok(line_width) => {
                let fill_type = if filled { DRAW_FILL_FULL } else { DRAW_FILL_EMPTY };
                unsafe {
                    Paint_DrawCircle(pos.x, pos.y, radius, color as u16, line_width, fill_type);
                }
                Ok(())
            },
            Err(reason) => Err(reason)
        }
    }

    pub fn draw_point(&self, pos: Pos, color: Color, thickness: u8) -> Result<(), String> {
        match EInk::thickness_to_dot_pixel(thickness) {
            Ok(line_width) => {
                unsafe {
                    Paint_DrawPoint(pos.x, pos.y, color as u16, line_width, DOT_FILL_RIGHTUP);
                }
                Ok(())
            },
            Err(reason) => Err(reason)
        }
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }
}