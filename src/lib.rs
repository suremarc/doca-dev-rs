#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/// A native Rust API for the DOCA SDK.
pub mod doca;

/// Auto-generated bindings to the DOCA SDK.
pub mod doca_sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
