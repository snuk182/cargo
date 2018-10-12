//#![deny(warnings)]
#![cfg_attr(feature = "cargo-clippy", allow(blacklisted_name))]
#![cfg_attr(feature = "cargo-clippy", allow(explicit_iter_loop))]

extern crate bufstream;
extern crate cargo;
extern crate filetime;
extern crate flate2;
extern crate git2;
extern crate glob;
extern crate hex;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate proptest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tar;
extern crate toml;
extern crate url;
#[cfg(windows)]
extern crate winapi;

#[macro_use]
mod support;


mod cfg_features;


#[test]
fn aaa_trigger_cross_compile_disabled_check() {
    // This triggers the cross compile disabled check to run ASAP, see #5141
    support::cross_compile::disabled();
}
