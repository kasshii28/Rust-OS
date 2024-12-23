
//標準ライブラリ無効
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// エントリポイント

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}