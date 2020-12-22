#![no_std]

// extern {
//     fn test(a:i32);
// }

extern "C" {
    fn add(a:i32, b:i32) -> i32;
}

#[no_mangle]
pub fn invoke() {
    unsafe { 
        add(1,2);
        // test(1);
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}