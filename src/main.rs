use libepd::*;
use libepd::MIRROR_IMAGE::MIRROR_HORIZONTAL;
//use libc::*;

fn main() {
    unsafe {
        println!("Initializing...");
        if DEV_Module_Init() != 0 {
            panic!("Failed to initialize device!");
        }

        println!("e-Paper Init and clear...");
        EPD_2IN13_V2_Init(EPD_2IN13_V2_FULL as u8);
        EPD_2IN13_V2_Clear();
        DEV_Delay_ms(500);

        LIBEPD_Init2in32Image();
        Paint_SetMirroring(MIRROR_HORIZONTAL as u8);
        Paint_Clear(WHITE);
        LIBEPD_Display();

        DEV_Delay_ms(1000);

        <//EPD_2in13_V2_test(); // Ensure to have pic folder in working dir!
    }
}
