use crate::eink::{Pos, Color, EInk, DisplayMode as EInkDisplayMode};
use crate::eink::Color::{White, Black};
use crate::eink_calls::Call::*;
use crate::eink::DisplayMode::{Full, Partial};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[serde(tag = "call")]
pub enum Call {
    Text(TextCall),
    Clear(ClearCall),
    Display(DisplayCall),
    DisplayMode(DisplayModeCall),
    Refresh(RefreshCall),
    Line(LineCall),
    Rectangle(RectangleCall),
    Circle(CircleCall),
    Point(PointCall)
}

impl Call {
    pub fn call(&self, eink: &mut EInk) -> Result<(), String> {
        match self {
            Text(call) => call.call(eink),
            Clear(call) => call.call(eink),
            Display(call) => call.call(eink),
            DisplayMode(call) => call.call(eink),
            Refresh(call) => call.call(eink),
            Line(call) => call.call(eink),
            Rectangle(call) => call.call(eink),
            Circle(call) => call.call(eink),
            Point(call) => call.call(eink),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TextCall {
    pub pos: Pos,
    pub size: u8,
    pub text: String,
    pub color: Option<Color>,
}
impl TextCall {
    pub fn call(&self, eink: &mut EInk) -> Result<(), String> {
        eink.draw_string(self.pos,
                         &self.text,
                         self.size,
                         self.color.unwrap_or(Black))
    }
}

#[derive(Deserialize, Debug)]
pub struct ClearCall {
    pub color: Option<Color>,
    pub from: Option<Pos>,
    pub to: Option<Pos>
}
impl ClearCall {
    pub fn call(&self, eink: &mut EInk) -> Result<(), String> {
        eink.clear(self.color.unwrap_or(White),
                   self.from,
                   self.to);
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct DisplayCall { }
impl DisplayCall {
    pub fn call(&self, eink: &mut EInk) -> Result<(), String> {
        eink.display();
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct DisplayModeCall {
    pub mode: EInkDisplayMode
}
impl DisplayModeCall {
    pub fn call(&self, eink: &mut EInk) -> Result<(), String> {
        match &self.mode {
            Full => eink.to_full_mode(),
            Partial => eink.to_partial_mode()
        };
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct RefreshCall { }
impl RefreshCall {
    pub fn call(&self, eink: &mut EInk) -> Result<(), String> {
        eink.refresh();
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct LineCall {
    pub from: Pos,
    pub to: Pos,
    pub color: Option<Color>,
    pub thickness: u8
}
impl LineCall {
    pub fn call(&self, eink: &mut EInk) -> Result<(), String> {
        eink.draw_line(self.from,
                       self.to,
                       self.color.unwrap_or(Black),
                       self.thickness)
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Size {
    width: i16,
    height: i16,
}
#[derive(Deserialize, Debug)]
pub struct RectangleCall {
    pub from: Pos,
    pub to: Option<Pos>,
    pub size: Option<Size>,
    pub color: Option<Color>,
    pub thickness: Option<u8>,
    pub filled: bool
}
impl RectangleCall {
    pub fn call(&self, eink: &mut EInk) -> Result<(), String> {
        if self.to.is_none() && self.size.is_none() || self.to.is_some() && self.size.is_some() {
            return Err(String::from("Rectcall must either have key \"to\" or \"size\" defined"))
        }

        let to_pos = if self.to.is_some() { self.to.unwrap() } else {
            Pos {
                x: (self.from.x as i16 + self.size.unwrap().width) as u16,
                y: (self.from.y as i16 + self.size.unwrap().height) as u16
            }
        };

        eink.draw_rectangle(self.from,
                            to_pos,
                            self.color.unwrap_or(Black),
                            self.thickness.unwrap_or(1),
                            self.filled)
    }
}

#[derive(Deserialize, Debug)]
pub struct CircleCall {
    pub pos: Pos,
    pub radius: u16,
    pub color: Option<Color>,
    pub thickness: Option<u8>,
    pub filled: bool
}
impl CircleCall {
    pub fn call(&self, eink: &mut EInk) -> Result<(), String> {
        eink.draw_circle(self.pos,
                         self.radius,
                         self.color.unwrap_or(Black),
                         self.thickness.unwrap_or(1),
                         self.filled)
    }
}

#[derive(Deserialize, Debug)]
pub struct PointCall {
    pub pos: Pos,
    pub color: Option<Color>,
    pub thickness: Option<u8>
}
impl PointCall {
    pub fn call(&self, eink: &mut EInk) -> Result<(), String> {
        eink.draw_point(self.pos,
                        self.color.unwrap_or(Black),
                        self.thickness.unwrap_or(1))
    }
}