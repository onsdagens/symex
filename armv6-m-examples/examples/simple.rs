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
//use cortex_m_rt::entry;
use riscv_rt as _;
use riscv_rt::entry;
//use rp2040_hal as _;
#[no_mangle]
#[inline(never)]
fn rust_simple_test(mut t: u32) -> u32 {
    assume(t > 1);
    if t == 2 {
        // This path should never happen.
        //return 6
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
        //panic!("something");
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
