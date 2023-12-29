These are Rust bindings for the Viture One Linux C SDK.  They were developed
and tested on Ubuntu 22.04.

They require `viture_one_linux_sdk` to exist as a neighbour directory (used for
both header and .so).  This version is based on the 1.0.3 release.

They require libusb-1.0-0-dev and libudev-dev installed.  (libusb gets statically linked).

The main crate is viture_one_sdk.

It provides safe bindings, using RAII and typestate to ensure that methods can only
be called when it is appropriate.

Functionality:
* Set and query whether IMU is enabled.
* Set and query IMU frequency.
* Set and query whether SBS 3D is enabled.
* Receive IMU data via a callback
* Receiving message events is not yet supported because the C SDK hasn't
  implemented it yet.

To receive IMU data, implement CallbackImu.  If you want the raw data, you can
implement RawCallbackImu, but this requires writing unsafe code.

The Viture C SDK provides an entrypoint for event callbacks, but it is not
functional yet.  Accordingly, it's not exposed in the Rust bindings.

The sample program demonstrates all currently-supported functionality.  It must be run as root.

The secondary crate is viture-one-sdk-sys.  This provides raw bindings via bindgen.
