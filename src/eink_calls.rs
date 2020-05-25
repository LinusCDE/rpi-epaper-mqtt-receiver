use crate::eink::{Pos, Color, EInk};
use serde::Deserialize;
use crate::eink::Color::White;
use crate::eink_calls::Call::{Text, Display, Clear};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum Call {
    Text(TextCall),
    Clear(ClearCall),
    Display(DisplayCall),
}

impl Call {
    pub fn call(&self, eink: &mut EInk) -> Result<(), String> {
        match self {
            Text(call) => call.call(eink),
            Clear(call) => call.call(eink),
            Display(call) => call.call(eink),
        }
    }
}

pub trait Callable {
    fn call(&self, eink: &mut EInk) -> Result<(), String>;
}

#[derive(Deserialize, Debug)]
pub struct TextCall {
    pub pos: Pos,
    pub size: u8,
    pub text: String,
    pub color: Color,
}
impl Callable for TextCall {
    fn call(&self, eink: &mut EInk) -> Result<(), String> {
        eink.draw_string(self.pos.clone(), &self.text, self.size, self.color);
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct ClearCall {
    pub color: Option<Color>,
    pub from: Option<Pos>,
    pub to: Option<Pos>
}
impl Callable for ClearCall {
    fn call(&self, eink: &mut EInk) -> Result<(), String> {
        eink.clear(self.color.unwrap_or(White), self.from.clone(), self.to.clone());
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct DisplayCall { }
impl Callable for DisplayCall {
    fn call(&self, eink: &mut EInk) -> Result<(), String> {
        eink.display();
        Ok(())
    }
}