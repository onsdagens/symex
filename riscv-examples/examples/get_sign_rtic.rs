#![no_std]
#![no_main]
use core::panic::PanicInfo;
use hippomenes_rt as _;

#[rtic::app(device = hippomenes_core)]
mod app {
    #[shared]
    struct Shared {
        resource: bool,
    }

    #[local]
    struct Local {
        v: u32,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        (Shared { resource: true }, Local { v: 500 })
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(binds = Interrupt0, priority=2, shared=[resource], local=[v])]
    fn timer_task(cx: timer_task::Context) {
        let v = *cx.local.v;
        for _ in 0..v {
            unsafe {
                core::ptr::read_volatile(&v);
            }
        }
    }
}

#[panic_handler]
fn p(_: &PanicInfo) -> ! {
    loop {}
}
