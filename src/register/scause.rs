//! CLIC extended supervisor trap cause register.
use core::arch::asm;
use core::mem;

/// Supervisor trap cause register with CLIC extension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Scause {
    bits: usize,
}

impl Scause {
    const INTERRUPT: usize = 1 << (mem::size_of::<usize>() - 1);
    const SPP: usize = 0b1 << 28;
    const SPIE: usize = 0b1 << 27;
    const SPIL: usize = 0xff << 16;
    const EXC_CODE: usize = 0xfff << 0;

    /// Returns the contents of the register as raw bits.
    #[inline]
    pub const fn bits(self) -> usize {
        self.bits
    }

    /// Check if the trap is caused by an interrupt.
    #[inline]
    pub const fn is_interrupt(self) -> bool {
        self.bits & Self::INTERRUPT != 0
    }

    /// Check if the trap is caused by an exception.
    #[inline]
    pub const fn is_exception(self) -> bool {
        self.bits & Self::INTERRUPT == 0
    }

    /// Get supervisor previous privilege mode; same as `sstatus.mpp`.
    #[inline]
    pub const fn spp(self) -> SPP {
        match (self.bits & Self::SPP) >> 28 {
            0 => SPP::User,
            1 => SPP::Supervisor,
            _ => unreachable!(),
        }
    }

    /// Get supervisor previous interrupt enable; same as `sstatus.mpie`.
    #[inline]
    pub const fn spie(self) -> bool {
        self.bits & Self::SPIE != 0
    }

    /// Get CLIC supervisor previous interrupt level.
    #[inline]
    pub const fn spil(self) -> u8 {
        ((self.bits & Self::SPIL) >> 16) as u8
    }

    /// Get CLIC supervisor exception or interrupt code.
    #[inline]
    pub const fn trap_code(self) -> u16 {
        ((self.bits & Self::EXC_CODE) >> 0) as u16
    }
}

/// Supervisor previous privilege mode.
pub enum SPP {
    /// Supervisor mode.
    Supervisor = 1,
    /// User mode.
    User = 0,
}

/// Reads the CLIC extended `scause` CSR.
#[inline]
pub fn read() -> Scause {
    let bits: usize;
    unsafe { asm!("csrr {}, scause", out(reg) bits) };
    Scause { bits }
}
