use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::cpu::gdt;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Breakpoint handler (int3)
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        // Double fault handler (must use IST stack)
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init() {
    IDT.load();
}

// ========================
//  INTERRUPT HANDLERS
// ========================

extern "x86-interrupt" fn breakpoint_handler(_stack: InterruptStackFrame) {
    crate::vga_print(">>> BREAKPOINT INTERRUPT FIRED <<<\n");
}

extern "x86-interrupt" fn double_fault_handler(
    _stack: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    crate::vga_print("!! DOUBLE FAULT — HALTING\n");
    loop {}
}

