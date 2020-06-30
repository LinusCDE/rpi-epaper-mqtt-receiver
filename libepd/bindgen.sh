#!/bin/sh

cd `dirname $0`

if [ `ls llvmlinks | wc -l` -eq 1 ]; then
  echo "$0: WARNING:"
  echo "    If you're missing commands like llvm-config but have them with a"
  echo "    suffix (e.g. \"-7\"), you'll probably want to go into the llvmlinks"
  echo "    directory and run genlinks.py (and change the suffix to yours there)"
  echo
fi

  #e-Paper/RaspberryPi\&JetsonNano/c/lib/e-Paper/EPD_2in13b_V2.h \

PATH="$PATH:$PWD/llvmlinks" bindgen \
  src/libepd.h \
  --whitelist-function '^(DEV|EPD|Paint|GUI|LIBEPD)_.*' \
  --whitelist-var '(^(DEV|EPD)_.*)|^(sFONT|cFONT|Font[0-9]+)$' \
  --rustified-enum '^(LINE_STYLE|DRAW_FILL|PAINT_TIME|BMP_FILE_HEADER|BMP_INFO|RGB_QUAD|PAINT|MIRROR_IMAGE|DOT_PIXEL|DOT_STYLE)$' \
  -o src/bindings.rs \
  -- \
 -Ie-Paper/RaspberryPi\&JetsonNano/c/lib/Config \
 -Ie-Paper/RaspberryPi\&JetsonNano/c/lib/e-Paper \
 -Ie-Paper/RaspberryPi\&JetsonNano/c/lib/Fonts \
 -Ie-Paper/RaspberryPi\&JetsonNano/c/lib/GUI \
 -Ie-Paper/RaspberryPi\&JetsonNano/c/examples

sed -i 's/EPD_2IN13_WIDTH: u32/EPD_2IN13_WIDTH: u16/g' src/bindings.rs
sed -i 's/EPD_2IN13_HEIGHT: u32/EPD_2IN13_HEIGHT: u16/g' src/bindings.rs
sed -i 's/EPD_2IN13_FULL: u32/EPD_2IN13_FULL: u8/g' src/bindings.rs
sed -i 's/EPD_2IN13_PART: u32/EPD_2IN13_PART: u8/g' src/bindings.rs
