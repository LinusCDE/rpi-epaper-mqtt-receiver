use std::env;

fn main() {
    //let math = pkg_config::Config::new().probe("m").unwrap();
    //pkg_config::Config::new().probe("bcm2835").unwrap();
    //println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=bcm2835");
    let mut cflags = String::new();
    cflags.push_str("-Ie-Paper/RaspberryPi&JetsonNano/c/lib/Config ");
    cflags.push_str("-Ie-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper ");
    cflags.push_str("-Ie-Paper/RaspberryPi&JetsonNano/c/lib/Fonts ");
    cflags.push_str("-Ie-Paper/RaspberryPi&JetsonNano/c/lib/GUI ");
    cflags.push_str("-Ie-Paper/RaspberryPi&JetsonNano/c/examples ");
    env::set_var("CFLAGS", cflags.as_str());

    let src = [
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Fonts/font12.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Fonts/font8.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Fonts/font12CN.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Fonts/font24.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Fonts/font24CN.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Fonts/font20.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Fonts/font16.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in7.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in9.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5bc.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in54c.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_5in83.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_4in2bc.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in13b_V2.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_5in65f.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in13_V2.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5b_HD.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in54b_V2.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in13bc.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_3in7.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5_HD.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5b_V3.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in9b_V2.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_4in2.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5_V2.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in54b.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in54_V2.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5b_V2.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in54.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in13d.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_5in83bc.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in02d.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_4in2b_V2.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in13.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in9bc.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in7b.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in9d.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/GUI/GUI_Paint.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/GUI/GUI_BMPfile.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Config/dev_hardware_SPI.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Config/sysfs_software_spi.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Config/DEV_Config.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Config/sysfs_gpio.c",
        "e-Paper/RaspberryPi&JetsonNano/c/lib/Config/RPI_sysfs_gpio.c",
        "e-Paper/RaspberryPi&JetsonNano/c/examples/ImageData.c",
        "e-Paper/RaspberryPi&JetsonNano/c/examples/EPD_2in13_V2_test.c",
        "src/libepd.c"
    ];
    let mut builder = cc::Build::new();
    // clang_arg and include dont seem to work! But bindgen ... -- -I... does!
    let build = builder
        .files(src.iter())
        .define("RPI", None)
        .define("USE_BCM2835_LIB", None)
        .flag("-Wno-unused-parameter");

    build.compile("libepd.a");
}
