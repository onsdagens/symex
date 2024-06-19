#![no_std]
#![no_main]
use core::panic::PanicInfo;
use hippomenes_rt as _;

#[rtic::app(device = hippomenes_core)]
mod app {
    #[shared]
    struct Shared {
        v: bool,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        (Shared { v: true }, Local {})
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(binds = Interrupt1, priority = 2, shared = [v])]
    fn some_task(mut cx: some_task::Context) {
        cx.shared.v.lock(|v| {
            if *v {
                for _ in 0..20 {
                    unsafe { core::arch::asm!("nop") };
                }
            } else {
            }
        })
    }
    #[task(binds = Interrupt0, priority=2, shared = [v])]
    fn timer_task(mut cx: timer_task::Context) {
        cx.shared.v.lock(|v| *v = false);
    }
}

#[panic_handler]
fn p(_: &PanicInfo) -> ! {
    loop {}
}
