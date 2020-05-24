#ifndef _LIBEPD_H_
#define _LIBEPD_H_

#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/Config/Debug.h"
#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/Config/DEV_Config.h"
#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/Config/dev_hardware_SPI.h"
#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/Config/RPI_sysfs_gpio.h"
#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/Config/sysfs_gpio.h"
#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/Config/sysfs_software_spi.h"

//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in02d.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in54b.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in54b_V2.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in54c.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in54.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_1in54_V2.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in13bc.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in13b_V2.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in13d.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in13.h"
#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in13_V2.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in7b.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in7.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in9bc.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in9b_V2.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in9d.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_2in9.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_3in7.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_4in2bc.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_4in2b_V2.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_4in2.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_5in65f.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_5in83bc.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_5in83.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5bc.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5b_HD.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5b_V2.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5b_V3.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5_HD.h"
//#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/e-Paper/EPD_7in5_V2.h"

#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/Fonts/fonts.h"
#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/GUI/GUI_BMPfile.h"
#include "../e-Paper/RaspberryPi&JetsonNano/c/lib/GUI/GUI_Paint.h"
#include "../e-Paper/RaspberryPi&JetsonNano/c/examples/ImageData.h"
#include "../e-Paper/RaspberryPi&JetsonNano/c/examples/EPD_Test.h"

void LIBEPD_Init2in32Image(void);
void LIBEPD_SelectImage(void);
void LIBEPD_Display(void);
void LIBEPD_DisplayPartialBase(void);
void LIBEPD_DisplayPartial(void);

#endif // _LIBEPD_H_