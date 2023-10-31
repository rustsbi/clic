//! Machine conditional scratch swap on level change.
use core::arch::asm;

/// Writes the CLIC `mscratchcswl` CSR.
#[inline]
pub unsafe fn write(val: usize) {
    asm!("csrw 0x349, {}", in(reg) val)
}

/// Reads the `mscratchcswl` CSR.
#[inline]
pub fn read() -> usize {
    let bits: usize;
    unsafe { asm!("csrr {}, 0x349", out(reg) bits) };
    bits
}
