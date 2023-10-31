//! CLIC extended machine trap cause register.
use core::arch::asm;
use core::mem;

/// Machine trap cause register with CLIC extension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Mcause {
    bits: usize,
}

impl Mcause {
    const INTERRUPT: usize = 1 << (mem::size_of::<usize>() - 1);
    const MPP: usize = 0b11 << 28;
    const MPIE: usize = 0b1 << 27;
    const MPIL: usize = 0xff << 16;
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

    /// Get machine previous privilege mode; same as `mstatus.mpp`.
    #[inline]
    pub const fn mpp(self) -> MPP {
        match (self.bits & Self::MPP) >> 28 {
            0 => MPP::User,
            1 => MPP::Supervisor,
            3 => MPP::Machine,
            _ => unreachable!(),
        }
    }

    /// Get machine previous interrupt enable; same as `mstatus.mpie`.
    #[inline]
    pub const fn mpie(self) -> bool {
        self.bits & Self::MPIE != 0
    }

    /// Get CLIC machine previous interrupt level.
    #[inline]
    pub const fn mpil(self) -> u8 {
        ((self.bits & Self::MPIL) >> 16) as u8
    }

    /// Get CLIC machine exception or interrupt code.
    #[inline]
    pub const fn trap_code(self) -> u16 {
        ((self.bits & Self::EXC_CODE) >> 0) as u16
    }
}

/// Machine previous privilege mode.
pub enum MPP {
    /// Machine mode.
    Machine = 3,
    /// Supervisor mode.
    Supervisor = 1,
    /// User mode.
    User = 0,
}

/// Reads the CLIC extended `mcause` CSR.
#[inline]
pub fn read() -> Mcause {
    let bits: usize;
    unsafe { asm!("csrr {}, mcause", out(reg) bits) };
    Mcause { bits }
}
