//! Supervisor interrupt-level threshold register.
use core::arch::asm;

/// Supervisor interrupt-level threshold register.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Sintthresh {
    bits: usize,
}

impl Sintthresh {
    const TH: usize = 0xff << 0;

    /// Returns the contents of the register as raw bits.
    #[inline]
    pub const fn bits(self) -> usize {
        self.bits
    }

    /// Set threshold level.
    #[inline]
    pub const fn set_th(self, val: u8) -> Sintthresh {
        Self {
            bits: (self.bits & !Self::TH) | ((val as usize) << 0),
        }
    }

    /// Get threshold level.
    #[inline]
    pub const fn th(self) -> u8 {
        ((self.bits & Self::TH) >> 0) as u8
    }
}

/// Writes the CLIC `sintthresh` CSR.
#[inline]
pub unsafe fn write(val: Sintthresh) {
    asm!("csrw 0x147, {}", in(reg) val.bits)
}

/// Reads the `sintthresh` CSR.
#[inline]
pub fn read() -> Sintthresh {
    let bits: usize;
    unsafe { asm!("csrr {}, 0x147", out(reg) bits) };
    Sintthresh { bits }
}
