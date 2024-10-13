#![no_std]
#![allow(unused)]

#[macro_use]
extern crate cfg_if as _;

#[macro_use]
extern crate core as _;

#[cfg(feature = "std")]
#[macro_use]
extern crate std as _;

#[cfg(not(target_has_atomic))]
compile_error!("the current device does not have atomics");

cfg_if! {
    if #[cfg(feature = "std")] {
        pub use ::std::sync::atomic::*;
    } else {
        pub use ::core::sync::atomic::*;
    }
}
