# rpi-epaper-mqtt-receiver
Control the Waveshare 2in13 E-Ink display with drawcalls over mqtt using rust and the demo c library. Mirror of https://git.cosmos-ink.net/linus/rpi-epaper-mqtt-receiver

Disclaimer: This code was written on a RPi Zero W (with those speeds) while still being really new to Rust.

The code is riddled with ignorance, frustrations, bad practice, inefficiencies and more.
It may probably by some representation of what a beginner to rust might first write.

It worked (the ffi version), but only after about 30 seconds. After that the c driver somehow locked up and stopped updating the eink screen.
