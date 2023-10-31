//! Machine interrupt-level threshold register.
use core::arch::asm;

/// Machine interrupt-level threshold register.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Mintthresh {
    bits: usize,
}

impl Mintthresh {
    const TH: usize = 0xff << 0;

    /// Returns the contents of the register as raw bits.
    #[inline]
    pub const fn bits(self) -> usize {
        self.bits
    }

    /// Set threshold level.
    #[inline]
    pub const fn set_th(self, val: u8) -> Mintthresh {
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

/// Writes the CLIC `mintthresh` CSR.
#[inline]
pub unsafe fn write(val: Mintthresh) {
    asm!("csrw 0x347, {}", in(reg) val.bits)
}

/// Reads the `mintthresh` CSR.
#[inline]
pub fn read() -> Mintthresh {
    let bits: usize;
    unsafe { asm!("csrr {}, 0x347", out(reg) bits) };
    Mintthresh { bits }
}
