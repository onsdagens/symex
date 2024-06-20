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

    #[init]
    fn init() -> Shared {
        Shared { v: false }
    }

    #[task(binds = Interrupt0, priority=1, shared=[v])]
    struct Task1;

    impl RticTask for Task1 {
        fn init() -> Self {
            Self
        }

        fn exec(&mut self) {
            self.shared().v.lock(|v| {
                if *v {
                    for _ in 0..20 {
                        unsafe { core::arch::asm!("nop") };
                    }
                } else {
                }
            });
        }
    }

    #[task(binds = Interrupt1, priority=3, shared=[v])]
    struct Task2;

    impl RticTask for Task2 {
        fn init() -> Self {
            Self
        }

        fn exec(&mut self) {
            self.shared().v.lock(|v| *v = true);
        }
    }
}

#[panic_handler]
fn p(_: &PanicInfo) -> ! {
    loop {}
}
