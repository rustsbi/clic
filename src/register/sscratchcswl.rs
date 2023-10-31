//! Supervisor conditional scratch swap on level change.
use core::arch::asm;

/// Writes the CLIC `sscratchcswl` CSR.
#[inline]
pub unsafe fn write(val: usize) {
    asm!("csrw 0x149, {}", in(reg) val)
}

/// Reads the `sscratchcswl` CSR.
#[inline]
pub fn read() -> usize {
    let bits: usize;
    unsafe { asm!("csrr {}, 0x149", out(reg) bits) };
    bits
}
