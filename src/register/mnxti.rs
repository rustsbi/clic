//! Machine interrupt handler address and enable modifier register.

// TODO: this CSR is different than a regular one; it's only designed to use with
// CSRR(CSRRS rd, csr, x0), CSRRSI and CSRRCI instructions.
