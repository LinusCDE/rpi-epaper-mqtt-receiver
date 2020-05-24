use crate::eink::Color::{Black, White};
use crate::eink::Pos;

mod eink;

fn main() {
    println!("Initializing...");

    let mut eink = eink::EInk::new(Black);
    eink.to_partial_mode();
    eink.delay(1000);
    eink.draw_line(Pos {x: 0, y: 10}, Pos {x: eink.get_width(), y: 10}, White, 1);
    eink.display();
    eink.delay(500);
    eink.draw_string(Pos {x: 20,y: 40}, "Hello World!", 24, White);
    eink.display();
    eink.delay(1000);

    eink.to_full_mode();
    eink.clear(White);
    eink.draw_string(Pos {x: 20, y: 40}, "Hello World!", 24, Black);
    eink.display();
    //eink.drawLine()

    //EPD_2in13_V2_test(); // Ensure to have pic folder in working dir!
}
