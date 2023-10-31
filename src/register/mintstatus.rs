//! Machine current interrupt level register.
use core::arch::asm;

/// Machine current interrupt level register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Mintstatus {
    bits: usize,
}

impl Mintstatus {
    const MIL: usize = 0xff << 24;
    const SIL: usize = 0xff << 8;
    const UIL: usize = 0xff << 0;

    /// Returns the contents of the register as raw bits.
    #[inline]
    pub const fn bits(self) -> usize {
        self.bits
    }

    /// Machine-mode interrupt level.
    #[inline]
    pub const fn mil(self) -> u8 {
        ((self.bits & Self::MIL) >> 24) as u8
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

/// Reads the CLIC 'mintstatus' CSR.
#[inline]
pub fn read() -> Mintstatus {
    let bits: usize;
    unsafe { asm!("csrr {}, 0xFB1", out(reg) bits) };
    Mintstatus { bits }
}
