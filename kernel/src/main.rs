// ~/eos/kernel/src/main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod cpu {
    pub mod gdt;
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_print("eOS: reached _start()\n");

    // Step: GDT only (no IDT, no interrupts yet)
    cpu::gdt::init();

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_print("\nPANIC!\n");

    if let Some(loc) = info.location() {
        vga_print("at ");
        vga_print(loc.file());
        vga_print(":");
        vga_print_u64(loc.line() as u64);
        vga_print("\n");
    } else {
        vga_print("at <unknown>\n");
    }

    loop {
        x86_64::instructions::hlt();
    }
}

// ======================
// VGA TEXT OUTPUT
// ======================
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_ATTR: u8 = 0x0f;

static mut VGA_CURSOR: usize = 0; // byte offset into VGA buffer (2 bytes per cell)

pub fn vga_print(s: &str) {
    unsafe {
        for byte in s.bytes() {
            match byte {
                b'\n' => new_line(),
                b => write_byte(b),
            }
        }
    }
}

unsafe fn write_byte(byte: u8) {
    if VGA_CURSOR >= VGA_WIDTH * VGA_HEIGHT * 2 {
        scroll_up();
        VGA_CURSOR = (VGA_HEIGHT - 1) * VGA_WIDTH * 2;
    }

    *VGA_BUFFER.add(VGA_CURSOR) = byte;
    *VGA_BUFFER.add(VGA_CURSOR + 1) = VGA_ATTR;
    VGA_CURSOR += 2;

    // wrap end-of-line
    if (VGA_CURSOR / 2) % VGA_WIDTH == 0 {
        new_line();
    }
}

unsafe fn new_line() {
    let row = (VGA_CURSOR / 2) / VGA_WIDTH;
    if row + 1 >= VGA_HEIGHT {
        scroll_up();
        VGA_CURSOR = (VGA_HEIGHT - 1) * VGA_WIDTH * 2;
    } else {
        VGA_CURSOR = (row + 1) * VGA_WIDTH * 2;
    }
}

unsafe fn scroll_up() {
    // Move rows 1..end up to 0..end-1
    for row in 1..VGA_HEIGHT {
        for col in 0..VGA_WIDTH {
            let from = (row * VGA_WIDTH + col) * 2;
            let to = ((row - 1) * VGA_WIDTH + col) * 2;

            *VGA_BUFFER.add(to) = *VGA_BUFFER.add(from);
            *VGA_BUFFER.add(to + 1) = *VGA_BUFFER.add(from + 1);
        }
    }

    // Clear last row
    let last_row = VGA_HEIGHT - 1;
    for col in 0..VGA_WIDTH {
        let i = (last_row * VGA_WIDTH + col) * 2;
        *VGA_BUFFER.add(i) = b' ';
        *VGA_BUFFER.add(i + 1) = VGA_ATTR;
    }
}

// Minimal decimal printer (no heap)
fn vga_print_u64(mut n: u64) {
    if n == 0 {
        vga_print("0");
        return;
    }

    let mut buf = [0u8; 20];
    let mut i = 0usize;

    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }

    while i > 0 {
        i -= 1;
        let b = buf[i];
        unsafe {
            write_byte(b);
        }
    }
}

