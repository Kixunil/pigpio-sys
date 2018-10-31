//! # Raw, unsafe bindings to pigpio library
//!
//! Please see the [pigpio documentation](http://abyz.me.uk/rpi/pigpio/cif.html).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
