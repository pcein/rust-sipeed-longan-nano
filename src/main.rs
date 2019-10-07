#![feature(global_asm)]
#![no_main]
#![no_std]

// LED's on PC13, PA1 and PA2
// We will use PA1 (green only)

const rcu_apb2en: u32 = (0x4002_1000 + 0x18);

const gpioa_ctl0: u32 = (0x4001_0800 + 0x0);
const gpioa_data: u32 = (0x4001_0800 + 0xc);

use panic_abort;

// The reset handler
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    main()
}

fn init_ports() {
    unsafe {
        // Enable clock to Port A and Port C 
        let x = core::ptr::read_volatile(rcu_apb2en as *mut u32);
        core::ptr::write_volatile(rcu_apb2en as *mut u32, x | (1 << 2));
        // Enable push-pull o/p Port A, pins 1 and 2.
        let x = core::ptr::read_volatile(gpioa_ctl0 as *mut u32);
        core::ptr::write_volatile(gpioa_ctl0 as *mut u32, x | (1 << 4));
    }
}

// don't compile with optimization enabled!
fn delay(mut n: u32) {
    while n != 0 {
        n -= 1;
    }
}

// Blink Green LED (PA1).
fn blink_led() {
    let mut bits:u32 = !(1 << 1);    
    loop {
        unsafe {
            // LED on when PA1 bit is 0
            core::ptr::write_volatile(gpioa_data as *mut u32, bits);
        }
        delay(0x4ffff);
        bits = !bits;
    }
}

fn main() -> !  {
    init_ports();
    blink_led();
    loop { }
}

extern "C" {
    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section (stored in Flash)
    static _sidata: u32;
}

// Make sure there is an abort when linking
#[cfg(target_arch = "riscv32")]
global_asm!(r#"
lui sp, %hi(__stacktop)
call Reset
.globl abort
abort:
  jal zero, abort
"#);
