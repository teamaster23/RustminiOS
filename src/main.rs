#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ex::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ex::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    ex::init(); // new

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3(); // new

    #[cfg(test)]
    test_main();

    println!("NB");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ex::test_panic_handler(info)
}
