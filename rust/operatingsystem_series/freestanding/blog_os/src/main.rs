#![no_std]
#![no_main]
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}
//In a typical Rust binary that links the standard library, 
// execution starts in a C runtime library called crt0 (“C runtime zero”), 
// which sets up the environment for a C application. 
// This includes creating a stack and placing the arguments in the right registers. 
// 
// The C runtime then invokes the entry point of the Rust runtime, 
// which is marked by the start language item

//By using the #[no_mangle] attribute we disable the name mangling to ensure 
// that the Rust compiler really outputs a function with the name _start

//The ! return type means that the function is diverging, i.e. not allowed to ever return

//target triple
// rustc --version --verbose
//for my machine
//x86-64-unknown-linux-gnu
//architecture-vendor-os-ABI

//LINKER ERROR - 
//By compiling for our host triple, the Rust compiler and 
// the linker assume that there is an underlying operating system such as Linux or
//  Windows that use the C runtime by default, which causes the linker errors

//added target for compilation thumbv7em-none-eabihf
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
