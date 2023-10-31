//! Supervisor current interrupt level register.
use core::arch::asm;

/// Supervisor current interrupt level register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Sintstatus {
    bits: usize,
}

impl Sintstatus {
    const SIL: usize = 0xff << 8;
    const UIL: usize = 0xff << 0;

    /// Returns the contents of the register as raw bits.
    #[inline]
    pub const fn bits(self) -> usize {
        self.bits
    }
    
    /// Supervisor-mode interrupt level.
    #[inline]
    pub const fn sil(self) -> u8 {
        ((self.bits & Self::SIL) >> 8) as u8
    }

    /// User-mode interrupt level.
    #[inline]
    pub const fn uil(self) -> u8 {
        ((self.bits & Self::UIL) >> 0) as u8
    }
}

/// Reads the CLIC 'sintstatus' CSR.
#[inline]
pub fn read() -> Sintstatus {
    let bits: usize;
    unsafe { asm!("csrr {}, 0xDB1", out(reg) bits) };
    Sintstatus { bits }
}