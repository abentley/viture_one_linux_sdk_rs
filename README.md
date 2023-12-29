These are Rust bindings for the Viture One Linux C SDK.  They were developed
and tested on Ubuntu 22.04.

They require `viture_one_linux_sdk` to exist as a neighbour directory (used for
both header and .so).  This version is based on the 1.0.3 release.

They require libusb-1.0-0-dev and libudev-dev installed.  (libusb gets statically linked).

Two crates are provided:
viture-one-sdk-sys: Raw bindings via bindgen

viture_one_sdk: Safe bindings via viture-one-sdk-sys.

Included: safe bindings using RAII, unsafe bindings via bindgen in `viture_rs::sys`

Functionality: Set and query whether IMU is enabled.  Set and query IMU
frequency. Set and query whether SBS 3D is enabled.

To receive IMU data, implement CallbackImu.  If you want the raw data, you can
implement RawCallbackImu, but this requires writing unsafe code.

The Viture C SDK doesn't currently support event callbacks.

The sample program demonstrates all currently-supported functionality.  It must be run as root.
