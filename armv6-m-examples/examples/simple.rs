//! Simple example.
//!
//! ```shell
//! cargo symex --example simple
//! ```
#![no_std]
#![no_main]
use symex_lib::{assume, symbolic, Any};
use core::panic;
use panic_halt as _;
use riscv_rt as _;
use riscv_rt::entry;
#[no_mangle]
#[inline(never)]
fn rust_simple_test(mut t: u32) -> u32 {
    assume(t > 1);
    if t == 2 {
        // This path should never happen.
       panic!("does not work");
    }
    symbolic(&mut t);
    if t == 3 {
        // But this should.
        0
    }
    else if t==4 {
        4
    }
    else{
        13
    }
}
#[entry]
unsafe fn main() -> !{
    let t = u32::any();
    rust_simple_test(t);
    let c = t + 1;
    loop{}
    panic!();
}
