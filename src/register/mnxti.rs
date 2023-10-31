//! Machine interrupt handler address and enable modifier register.
use core::arch::asm;

/// Get next interrupt handler address and enable MIE.
#[inline]
pub fn next_handler_set_mie() -> usize {
    let mut val: usize;
    unsafe { asm!("csrrsi {}, 0x345, 8", out(reg) val) };
    val
}

/// Get next interrupt handler address and enable SIE.
#[inline]
pub fn next_handler_set_sie() -> usize {
    let mut val: usize;
    unsafe { asm!("csrrsi {}, 0x345, 2", out(reg) val) };
    val
}

/// Get next interrupt handler address and disable MIE.
#[inline]
pub fn next_handler_clear_mie() -> usize {
    let mut val: usize;
    unsafe { asm!("csrrci {}, 0x345, 8", out(reg) val) };
    val
}

/// Get next interrupt handler address and disable SIE.
#[inline]
pub fn next_handler_clear_sie() -> usize {
    let mut val: usize;
    unsafe { asm!("csrrci {}, 0x345, 2", out(reg) val) };
    val
}

/// Get next interrupt handler address.
#[inline]
pub fn next_handler() -> usize {
    let mut val: usize;
    unsafe { asm!("csrr {}, 0x345", out(reg) val) };
    val
}
