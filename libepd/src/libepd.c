#include "libepd.h"
#include <stdlib.h>
#include <stdint.h>

static uint8_t* canvas;

void LIBEPD_Init2in32Image(void) {
    /* you have to edit the startup_stm32fxxx.s file and set a big enough heap size */
    uint16_t Imagesize = ((EPD_2IN13_V2_WIDTH % 8 == 0)? (EPD_2IN13_V2_WIDTH / 8 ): (EPD_2IN13_V2_WIDTH / 8 + 1)) * EPD_2IN13_V2_HEIGHT;
    if((canvas = (uint8_t *)malloc(Imagesize)) == NULL) {
        printf("Failed to apply for image memory...\r\n");
        exit(-1);
    }
    Paint_NewImage(canvas, EPD_2IN13_V2_WIDTH, EPD_2IN13_V2_HEIGHT, 270, WHITE);
    LIBEPD_SelectImage();
    Paint_SetMirroring(MIRROR_HORIZONTAL); //
}

void LIBEPD_SelectImage(void) {
    Paint_SelectImage(canvas);
}

void LIBEPD_Display(void) {
    EPD_2IN13_V2_Display(canvas);
}

void LIBEPD_DisplayPartialBase(void) {
    EPD_2IN13_V2_DisplayPartBaseImage(canvas);
}

void LIBEPD_DisplayPartial(void) {
    EPD_2IN13_V2_DisplayPart(canvas);
}