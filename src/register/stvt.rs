//! Supervisor trap-handler vector table base address register.
use core::arch::asm;

/// Supervisor trap-handler vector table base address register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Stvt {
    bits: usize,
}

impl Stvt {
    /// Returns the trap-vector base-address.
    #[inline]
    pub const fn base_address(self) -> usize {
        self.bits & !0b111111
    }
}

/// Writes the `stvt` CSR.
#[inline]
pub unsafe fn write(addr: usize) {
    assert!(
        addr & 0b111111 == 0,
        "CLIC vector base address must align to 64 bytes"
    );
    asm!("csrw 0x307, {}", in(reg) addr)
}

/// Reads the `stvt` CSR.
#[inline]
pub fn read() -> Stvt {
    let bits: usize;
    unsafe { asm!("csrr {}, 0x107", out(reg) bits) };
    Stvt { bits }
}
