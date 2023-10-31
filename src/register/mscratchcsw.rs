//! Machine conditional scratch swap on privilege mode change.
use core::arch::asm;

/// Writes the CLIC `mscratchcsw` CSR.
#[inline]
pub unsafe fn write(val: usize) {
    asm!("csrw 0x348, {}", in(reg) val)
}

/// Reads the `mscratchcsw` CSR.
#[inline]
pub fn read() -> usize {
    let bits: usize;
    unsafe { asm!("csrr {}, 0x348", out(reg) bits) };
    bits
}
