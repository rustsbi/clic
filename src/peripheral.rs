//! CLIC memory-mapped peripherals.
use volatile_register::RW;

/// M-mode CLIC memory map.
#[repr(C)]
pub struct Clic {
    _reserved0: [u8; 64],
    trigger: [RW<u32>; 32],
    _reserved1: [u8; 1856],
    interrupt: [Interrupt; 4096],
}

/// Memory-mapped control registers for each interrupt.
#[repr(C)]
pub struct Interrupt {
    pending: RW<Pending>,
    enable: RW<Enable>,
    attribute: RW<Attribute>,
    control: RW<Control>,
}

/// Interrupt pending register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Pending(u8);

/// Interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Enable(u8);

/// Interrupt attribute register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Attribute(u8);

/// Interrupt level and priority register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control(u8);
