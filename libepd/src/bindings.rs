/* automatically generated by rust-bindgen */

pub const DEV_HARDWARE_SPI_DEBUG: u32 = 0;
pub const EPD_2IN13_WIDTH: u16 = 122;
pub const EPD_2IN13_HEIGHT: u16 = 250;
pub const EPD_2IN13_FULL: u8 = 0;
pub const EPD_2IN13_PART: u8 = 1;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
extern "C" {
    pub static mut EPD_RST_PIN: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut EPD_DC_PIN: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut EPD_CS_PIN: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut EPD_BUSY_PIN: ::std::os::raw::c_int;
}
extern "C" {
    pub fn DEV_Digital_Write(Pin: u16, Value: u8);
}
extern "C" {
    pub fn DEV_Digital_Read(Pin: u16) -> u8;
}
extern "C" {
    pub fn DEV_SPI_WriteByte(Value: u8);
}
extern "C" {
    pub fn DEV_SPI_Write_nByte(pData: *mut u8, Len: u32);
}
extern "C" {
    pub fn DEV_Delay_ms(xms: u32);
}
extern "C" {
    pub fn DEV_Module_Init() -> u8;
}
extern "C" {
    pub fn DEV_Module_Exit();
}
#[doc = "< CPOL = 0, CPHA = 0"]
pub const SPIMode_SPI_MODE0: SPIMode = 0;
#[doc = "< CPOL = 0, CPHA = 1"]
pub const SPIMode_SPI_MODE1: SPIMode = 1;
#[doc = "< CPOL = 1, CPHA = 0"]
pub const SPIMode_SPI_MODE2: SPIMode = 2;
#[doc = "< CPOL = 1, CPHA = 1"]
pub const SPIMode_SPI_MODE3: SPIMode = 3;
pub type SPIMode = u32;
pub const SPICSEN_DISABLE: SPICSEN = 0;
pub const SPICSEN_ENABLE: SPICSEN = 1;
pub type SPICSEN = u32;
#[doc = "< Chip Select 0"]
pub const SPIChipSelect_SPI_CS_Mode_LOW: SPIChipSelect = 0;
#[doc = "< Chip Select 1"]
pub const SPIChipSelect_SPI_CS_Mode_HIGH: SPIChipSelect = 1;
#[doc = "< No CS, control it yourself"]
pub const SPIChipSelect_SPI_CS_Mode_NONE: SPIChipSelect = 3;
pub type SPIChipSelect = u32;
#[doc = "< LSB First"]
pub const SPIBitOrder_SPI_BIT_ORDER_LSBFIRST: SPIBitOrder = 0;
#[doc = "< MSB First"]
pub const SPIBitOrder_SPI_BIT_ORDER_MSBFIRST: SPIBitOrder = 1;
pub type SPIBitOrder = u32;
pub const BusMode_SPI_3WIRE_Mode: BusMode = 0;
pub const BusMode_SPI_4WIRE_Mode: BusMode = 1;
pub type BusMode = u32;
extern "C" {
    pub fn DEV_HARDWARE_SPI_begin(SPI_device: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn DEV_HARDWARE_SPI_beginSet(
        SPI_device: *mut ::std::os::raw::c_char,
        mode: SPIMode,
        speed: u32,
    );
}
extern "C" {
    pub fn DEV_HARDWARE_SPI_end();
}
extern "C" {
    pub fn DEV_HARDWARE_SPI_setSpeed(speed: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn DEV_HARDWARE_SPI_TransferByte(buf: u8) -> u8;
}
extern "C" {
    pub fn DEV_HARDWARE_SPI_Transfer(buf: *mut u8, len: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn DEV_HARDWARE_SPI_SetDataInterval(us: u16);
}
extern "C" {
    pub fn DEV_HARDWARE_SPI_SetBusMode(mode: BusMode) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn DEV_HARDWARE_SPI_SetBitOrder(Order: SPIBitOrder) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn DEV_HARDWARE_SPI_ChipSelect(CS_Mode: SPIChipSelect) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn DEV_HARDWARE_SPI_CSEN(EN: SPICSEN) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn DEV_HARDWARE_SPI_Mode(mode: SPIMode) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2IN13_Init(Mode: u8);
}
extern "C" {
    pub fn EPD_2IN13_Clear();
}
extern "C" {
    pub fn EPD_2IN13_Display(Image: *mut u8);
}
extern "C" {
    pub fn EPD_2IN13_Sleep();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _tFont {
    pub table: *const u8,
    pub Width: u16,
    pub Height: u16,
}
#[test]
fn bindgen_test_layout__tFont() {
    assert_eq!(
        ::std::mem::size_of::<_tFont>(),
        16usize,
        concat!("Size of: ", stringify!(_tFont))
    );
    assert_eq!(
        ::std::mem::align_of::<_tFont>(),
        8usize,
        concat!("Alignment of ", stringify!(_tFont))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_tFont>())).table as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_tFont),
            "::",
            stringify!(table)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_tFont>())).Width as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_tFont),
            "::",
            stringify!(Width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_tFont>())).Height as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(_tFont),
            "::",
            stringify!(Height)
        )
    );
}
pub type sFONT = _tFont;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CH_CN {
    pub index: [::std::os::raw::c_char; 2usize],
    pub matrix: [::std::os::raw::c_char; 166usize],
}
#[test]
fn bindgen_test_layout_CH_CN() {
    assert_eq!(
        ::std::mem::size_of::<CH_CN>(),
        168usize,
        concat!("Size of: ", stringify!(CH_CN))
    );
    assert_eq!(
        ::std::mem::align_of::<CH_CN>(),
        1usize,
        concat!("Alignment of ", stringify!(CH_CN))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CH_CN>())).index as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CH_CN),
            "::",
            stringify!(index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CH_CN>())).matrix as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(CH_CN),
            "::",
            stringify!(matrix)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cFONT {
    pub table: *const CH_CN,
    pub size: u16,
    pub ASCII_Width: u16,
    pub Width: u16,
    pub Height: u16,
}
#[test]
fn bindgen_test_layout_cFONT() {
    assert_eq!(
        ::std::mem::size_of::<cFONT>(),
        16usize,
        concat!("Size of: ", stringify!(cFONT))
    );
    assert_eq!(
        ::std::mem::align_of::<cFONT>(),
        8usize,
        concat!("Alignment of ", stringify!(cFONT))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cFONT>())).table as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cFONT),
            "::",
            stringify!(table)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cFONT>())).size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cFONT),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cFONT>())).ASCII_Width as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(cFONT),
            "::",
            stringify!(ASCII_Width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cFONT>())).Width as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(cFONT),
            "::",
            stringify!(Width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cFONT>())).Height as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(cFONT),
            "::",
            stringify!(Height)
        )
    );
}
extern "C" {
    pub static mut Font24: sFONT;
}
extern "C" {
    pub static mut Font20: sFONT;
}
extern "C" {
    pub static mut Font16: sFONT;
}
extern "C" {
    pub static mut Font12: sFONT;
}
extern "C" {
    pub static mut Font8: sFONT;
}
extern "C" {
    #[doc = " end"]
    pub fn GUI_ReadBmp(path: *const ::std::os::raw::c_char, Xstart: u16, Ystart: u16) -> u8;
}
extern "C" {
    pub fn GUI_ReadBmp_4Gray(path: *const ::std::os::raw::c_char, Xstart: u16, Ystart: u16) -> u8;
}
extern "C" {
    pub fn GUI_ReadBmp_RGB_7Color(
        path: *const ::std::os::raw::c_char,
        Xstart: u16,
        Ystart: u16,
    ) -> u8;
}
#[repr(u32)]
#[doc = " The size of the point"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DOT_PIXEL {
    DOT_PIXEL_1X1 = 1,
    DOT_PIXEL_2X2 = 2,
    DOT_PIXEL_3X3 = 3,
    DOT_PIXEL_4X4 = 4,
    DOT_PIXEL_5X5 = 5,
    DOT_PIXEL_6X6 = 6,
    DOT_PIXEL_7X7 = 7,
    DOT_PIXEL_8X8 = 8,
}
#[repr(u32)]
#[doc = " Point size fill style"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DOT_STYLE {
    DOT_FILL_AROUND = 1,
    DOT_FILL_RIGHTUP = 2,
}
#[repr(u32)]
#[doc = " Line style, solid or dashed"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LINE_STYLE {
    LINE_STYLE_SOLID = 0,
    LINE_STYLE_DOTTED = 1,
}
#[repr(u32)]
#[doc = " Whether the graphic is filled"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DRAW_FILL {
    DRAW_FILL_EMPTY = 0,
    DRAW_FILL_FULL = 1,
}
#[doc = " Custom structure of a time attribute"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PAINT_TIME {
    pub Year: u16,
    pub Month: u8,
    pub Day: u8,
    pub Hour: u8,
    pub Min: u8,
    pub Sec: u8,
}
#[test]
fn bindgen_test_layout_PAINT_TIME() {
    assert_eq!(
        ::std::mem::size_of::<PAINT_TIME>(),
        8usize,
        concat!("Size of: ", stringify!(PAINT_TIME))
    );
    assert_eq!(
        ::std::mem::align_of::<PAINT_TIME>(),
        2usize,
        concat!("Alignment of ", stringify!(PAINT_TIME))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PAINT_TIME>())).Year as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PAINT_TIME),
            "::",
            stringify!(Year)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PAINT_TIME>())).Month as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(PAINT_TIME),
            "::",
            stringify!(Month)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PAINT_TIME>())).Day as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(PAINT_TIME),
            "::",
            stringify!(Day)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PAINT_TIME>())).Hour as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PAINT_TIME),
            "::",
            stringify!(Hour)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PAINT_TIME>())).Min as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(PAINT_TIME),
            "::",
            stringify!(Min)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PAINT_TIME>())).Sec as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(PAINT_TIME),
            "::",
            stringify!(Sec)
        )
    );
}
extern "C" {
    pub fn Paint_NewImage(image: *mut u8, Width: u16, Height: u16, Rotate: u16, Color: u16);
}
extern "C" {
    pub fn Paint_SelectImage(image: *mut u8);
}
extern "C" {
    pub fn Paint_SetRotate(Rotate: u16);
}
extern "C" {
    pub fn Paint_SetMirroring(mirror: u8);
}
extern "C" {
    pub fn Paint_SetPixel(Xpoint: u16, Ypoint: u16, Color: u16);
}
extern "C" {
    pub fn Paint_SetScale(scale: u8);
}
extern "C" {
    pub fn Paint_Clear(Color: u16);
}
extern "C" {
    pub fn Paint_ClearWindows(Xstart: u16, Ystart: u16, Xend: u16, Yend: u16, Color: u16);
}
extern "C" {
    pub fn Paint_DrawPoint(
        Xpoint: u16,
        Ypoint: u16,
        Color: u16,
        Dot_Pixel: DOT_PIXEL,
        Dot_FillWay: DOT_STYLE,
    );
}
extern "C" {
    pub fn Paint_DrawLine(
        Xstart: u16,
        Ystart: u16,
        Xend: u16,
        Yend: u16,
        Color: u16,
        Line_width: DOT_PIXEL,
        Line_Style: LINE_STYLE,
    );
}
extern "C" {
    pub fn Paint_DrawRectangle(
        Xstart: u16,
        Ystart: u16,
        Xend: u16,
        Yend: u16,
        Color: u16,
        Line_width: DOT_PIXEL,
        Draw_Fill: DRAW_FILL,
    );
}
extern "C" {
    pub fn Paint_DrawCircle(
        X_Center: u16,
        Y_Center: u16,
        Radius: u16,
        Color: u16,
        Line_width: DOT_PIXEL,
        Draw_Fill: DRAW_FILL,
    );
}
extern "C" {
    pub fn Paint_DrawChar(
        Xstart: u16,
        Ystart: u16,
        Acsii_Char: ::std::os::raw::c_char,
        Font: *mut sFONT,
        Color_Foreground: u16,
        Color_Background: u16,
    );
}
extern "C" {
    pub fn Paint_DrawString_EN(
        Xstart: u16,
        Ystart: u16,
        pString: *const ::std::os::raw::c_char,
        Font: *mut sFONT,
        Color_Foreground: u16,
        Color_Background: u16,
    );
}
extern "C" {
    pub fn Paint_DrawString_CN(
        Xstart: u16,
        Ystart: u16,
        pString: *const ::std::os::raw::c_char,
        font: *mut cFONT,
        Color_Foreground: u16,
        Color_Background: u16,
    );
}
extern "C" {
    pub fn Paint_DrawNum(
        Xpoint: u16,
        Ypoint: u16,
        Nummber: i32,
        Font: *mut sFONT,
        Color_Foreground: u16,
        Color_Background: u16,
    );
}
extern "C" {
    pub fn Paint_DrawTime(
        Xstart: u16,
        Ystart: u16,
        pTime: *mut PAINT_TIME,
        Font: *mut sFONT,
        Color_Foreground: u16,
        Color_Background: u16,
    );
}
extern "C" {
    pub fn Paint_DrawBitMap(image_buffer: *const ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn EPD_1in02d_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_1in54_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_1in54_V2_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_1in54b_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_1in54c_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2in7_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2in7b_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2in9_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2in9bc_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2in9b_V2_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2in9d_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2in13_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2in13_V2_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2in13bc_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2in13b_V2_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_2in13d_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_3in7_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_4in2_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_4in2bc_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_4in2b_V2_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_5in83_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_5in83bc_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_7in5_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_7in5_HD_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_7in5_V2_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_7in5bc_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn EPD_7in5bc_V2_test() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn LIBEPD_Init2in32Image();
}
extern "C" {
    pub fn LIBEPD_SelectImage();
}
extern "C" {
    pub fn LIBEPD_Display();
}
extern "C" {
    pub fn LIBEPD_DisplayPartialBase();
}
extern "C" {
    pub fn LIBEPD_DisplayPartial();
}
