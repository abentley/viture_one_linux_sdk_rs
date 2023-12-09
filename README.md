These are extremely early Rust bindings for the Viture One Linux C SDK.  They
were developed and tested on Ubuntu 22.04.

They require `viture_one_linux_sdk_1.0.2` to exist as a neighbour directory.

They require libusb-1.0-0-dev and libudev-dev installed.  (libusb gets statically linked).

Included: safe bindings using RAII, unsafe bindings via bindgen in `viture_rs::sys`

Functionality: Set and query whether IMU is enabled.  Set and query whether SBS 3D is enabled.
Not included (in safe bindings): callback support for IMU and MCU.
