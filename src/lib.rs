#![no_std]
#![allow(unused)]

#[macro_use]
extern crate cfg_if as _;

#[macro_use]
extern crate core as _;

#[cfg(feature = "std")]
#[macro_use]
extern crate std as _;

cfg_if! {
    if #[cfg(feature = "std")] {
        pub use ::std::sync::atomic::*;
    } else {
        pub use ::core::sync::atomic::*;
    }
}
