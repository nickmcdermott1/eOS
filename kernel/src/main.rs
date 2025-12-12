#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod mem;

mod cpu {
    pub mod gdt;
    pub mod idt;
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Load GDT and IDT
    cpu::gdt::init();
    cpu::idt::init();

    // Enable interrupts globally (REQUIRED so int3 is processed)
    x86_64::instructions::interrupts::enable();

    vga_print("eOS ProtoCore booting...\n");
    vga_print("GDT initialised.\n");
    vga_print("IDT initialised.\n");

    // Trigger breakpoint interrupt
    x86_64::instructions::interrupts::int3();

    // This will print AFTER the handler fires
    vga_print("Kernel running successfully!\n");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// ===== VGA TEXT OUTPUT =====
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
static mut VGA_CURSOR: usize = 0;

pub fn vga_print(s: &str) {
    unsafe {
        for byte in s.bytes() {
            if byte == b'\n' {
                VGA_CURSOR = (VGA_CURSOR / (80 * 2) + 1) * (80 * 2);
                continue;
            }

            *VGA_BUFFER.add(VGA_CURSOR) = byte;
            *VGA_BUFFER.add(VGA_CURSOR + 1) = 0x0f;
            VGA_CURSOR += 2;
        }
    }
}

