#![no_std]
#![no_main]

#[cfg_attr(target_os = "linux", no_main)]
use noli::prelude::*;

#[no_mangle]
fn main() -> u64 {
    0
}

entry_point!(main);
