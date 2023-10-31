//! Supervisor conditional scratch swap on privilege mode change.
use core::arch::asm;

/// Writes the CLIC `sscratchcsw` CSR.
#[inline]
pub unsafe fn write(val: usize) {
    asm!("csrw 0x148, {}", in(reg) val)
}

/// Reads the `sscratchcsw` CSR.
#[inline]
pub fn read() -> usize {
    let bits: usize;
    unsafe { asm!("csrr {}, 0x148", out(reg) bits) };
    bits
}
