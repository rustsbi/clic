//! CLIC extended supervisor trap-handler base address register.
use core::arch::asm;

/// Supervisor trap-handler base address register with CLIC extension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Stvec {
    bits: usize,
}

impl Stvec {
    /// Returns the contents of the register as raw bits.
    #[inline]
    pub const fn bits(self) -> usize {
        self.bits
    }

    /// Returns the trap handler base-address.
    ///
    /// This function returns `OBASE` in specification under CLINT trap modes,
    /// `NBASE` if under CLIC trap mode, `None` if current trap mode is reserved.
    #[inline]
    pub const fn base_address(self) -> Option<usize> {
        match self.trap_mode() {
            Some(TrapMode::Direct | TrapMode::Vectored) => Some(self.bits & !0b11),
            Some(TrapMode::Clic) => Some(self.bits & !0b111111),
            None => None,
        }
    }

    /// Returns the trap-vector mode, `None` if reserved under CLIC.
    #[inline]
    pub const fn trap_mode(self) -> Option<TrapMode> {
        let mode = self.bits & 0b11;
        let submode = (self.bits & 0b111100) >> 2;
        match (mode, submode) {
            (0, _) => Some(TrapMode::Direct),
            (1, _) => Some(TrapMode::Vectored),
            (3, 0b0000) => Some(TrapMode::Vectored),
            _ => None,
        }
    }
}

/// Trap mode with CLIC extension.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrapMode {
    /// CLINT non-vectored basic mode.
    Direct = 0,
    /// CLINT vectored basic mode.
    Vectored = 1,
    /// CLIC mode.
    Clic = 3,
}

/// Writes the CLIC extended `mtvec` CSR.
#[inline]
pub unsafe fn write(addr: usize, mode: TrapMode) {
    match mode {
        TrapMode::Direct | TrapMode::Vectored => {
            assert!(addr & 0b11 == 0, "base address must align to 4 bytes")
        }
        TrapMode::Clic => assert!(
            addr & 0b111111 == 0,
            "CLIC base address must align to 64 bytes"
        ),
    }
    asm!("csrw mtvec, {}", in(reg) addr | mode as usize)
}

/// Reads the CLIC extended `stvec` CSR.
#[inline]
pub fn read() -> Stvec {
    let bits: usize;
    unsafe { asm!("csrr {}, stvec", out(reg) bits) };
    Stvec { bits }
}
