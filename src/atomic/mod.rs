//! Abstractions for the Atomics interface provided by the Tegra X1.
//!
//! See Chapter 7 in the Tegra X1 Technical Reference Manual for details.

mod registers;
pub use registers::*;

use register::FieldValue;

/// Abstraction for a `u32` that can be used atomically using
/// the Tegra X1 atomic operations.
pub struct AtomicU32 {
    /// The register index that is used for this atomic value.
    target_register: u32,
}

impl AtomicU32 {
    /// Creates a new `AtomicU32` that will operate in the given target register.
    ///
    /// The initial value is the given `value`.
    ///
    /// # Safety
    ///
    /// The given target register must not be used by any other
    /// `Atomic` type, otherwise the data will get corrupted.
    pub unsafe fn new(target_register: u32, value: u32) -> Self {
        assert!(
            target_register < 128,
            "there are only 128 target registers available"
        );

        Self { target_register }
    }

    /// Atomically swap this value with `val`, and return the old value that
    /// was stored in this atomic.
    pub fn exchange(&self, val: u32) -> u32 {
        let registers = unsafe { &*REGISTERS };

        // setup phase: store the new value inside the setup register
        registers.ATOMICS_AP0_SETUP_V_0[self.target_register as usize].set(val);

        // trigger the operation
        self.trigger_operation(TRIGGER::CMD::EXCHANGE);

        // read the old value out of the result register
        registers.ATOMICS_AP0_RESULT_0[self.target_register as usize].get()
    }

    /// Replaces the value of this atomic with `new`, if it matches `current`.
    pub fn compare_exchange(&self, current: u32, new: u32) {
        let registers = unsafe { &*REGISTERS };

        // setup phase: store `current` and `new` in the setup register
        registers.ATOMICS_AP0_SETUP_V_0[self.target_register as usize].set(new);
        registers.ATOMICS_AP0_SETUP_C_0[self.target_register as usize].set(current);

        // trigger the operation
        self.trigger_operation(TRIGGER::CMD::COMPARE_EXCHANGE);

        // if the comparison succeeded, the previous value will be in the result register,
        // but we don't return it because it may be undefined
        registers.ATOMICS_AP0_RESULT_0[self.target_register as usize].get();
    }

    /// Increment the value of this atomic by `x`.
    ///
    /// Returns the previous value.
    pub fn increment(&self, x: u32) -> u32 {
        let registers = unsafe { &*REGISTERS };

        // setup phase: store the value to add in the setup register
        registers.ATOMICS_AP0_SETUP_V_0[self.target_register as usize].set(x);

        // trigger the operation
        self.trigger_operation(TRIGGER::CMD::INCREMENT);

        // read the old value from the result register
        registers.ATOMICS_AP0_RESULT_0[self.target_register as usize].get()
    }

    /// Decrement the value of this atomic by `x`.
    ///
    /// If the addition goes under zero, a saturation will be added
    /// and the value becomes `0`, similair to Rusts [`wrapping_sub`](usize::wrapping_sub).
    ///
    /// Returns the previous value.
    pub fn decrement(&self, x: u32) -> u32 {
        let registers = unsafe { &*REGISTERS };

        // setup phase: store the value to subtract in the setup register
        registers.ATOMICS_AP0_SETUP_V_0[self.target_register as usize].set(x);

        // trigger the operation
        self.trigger_operation(TRIGGER::CMD::DECREMENT);

        // read the old value from the result register
        registers.ATOMICS_AP0_RESULT_0[self.target_register as usize].get()
    }

    /// Loads the value for this `Atomic`.
    pub fn get(&self) -> u32 {
        let registers = unsafe { &*REGISTERS };

        // for the get operation, no setup is required, so trigger the operation instantly
        self.trigger_operation(TRIGGER::CMD::GET);

        // read the value out of the result register
        registers.ATOMICS_AP0_RESULT_0[self.target_register as usize].get()
    }

    /// Puts `x` into this atomic value.
    pub fn put(&self, x: u32) {
        let registers = unsafe { &*REGISTERS };

        // setup phase: store the value to put inside the setup register
        registers.ATOMICS_AP0_SETUP_V_0[self.target_register as usize].set(x);

        // trigger the operation
        self.trigger_operation(TRIGGER::CMD::PUT);

        // `PUT` will also store the previous value in the result register,
        // like `exchange`, but we don't return it here, but we read it anyway
        registers.ATOMICS_AP0_RESULT_0[self.target_register as usize].get();
    }

    /// Performs a bit set with `x` and the value of this atomic.
    ///
    /// Returns the previous value.
    pub fn set(&self, x: u32) -> u32 {
        let registers = unsafe { &*REGISTERS };

        // setup phase: store the value to perfom a bit set with in the setup register
        registers.ATOMICS_AP0_SETUP_V_0[self.target_register as usize].set(x);

        // trigger the operation
        self.trigger_operation(TRIGGER::CMD::TEST_AND_SET);

        // read the old value from the result register
        registers.ATOMICS_AP0_RESULT_0[self.target_register as usize].get()
    }

    /// Performs a bit clear with `x` and the value of this atomic.
    ///
    /// Returns the previous value.
    pub fn clear(&self, x: u32) -> u32 {
        let registers = unsafe { &*REGISTERS };

        // setup phase: store the value to perfom a bit set with in the setup register
        registers.ATOMICS_AP0_SETUP_V_0[self.target_register as usize].set(x);

        // trigger the operation
        self.trigger_operation(TRIGGER::CMD::TEST_AND_CLEAR);

        // read the old value from the result register
        registers.ATOMICS_AP0_RESULT_0[self.target_register as usize].get()
    }

    /// Performs a bit invert with `x` and the value of this atomic.
    ///
    /// Returns the previous value.
    pub fn invert(&self, x: u32) -> u32 {
        let registers = unsafe { &*REGISTERS };

        // setup phase: store the value to perfom a bit set with in the setup register
        registers.ATOMICS_AP0_SETUP_V_0[self.target_register as usize].set(x);

        // trigger the operation
        self.trigger_operation(TRIGGER::CMD::TEST_AND_INVERT);

        // read the old value from the result register
        registers.ATOMICS_AP0_RESULT_0[self.target_register as usize].get()
    }

    /// Triggers the `cmd` operation by writing into the trigger register.
    fn trigger_operation(&self, cmd: FieldValue<u32, TRIGGER::Register>) {
        let registers = unsafe { &*REGISTERS };

        registers
            .ATOMICS_AP0_TRIGGER_0
            .write(cmd + TRIGGER::WIDTH64::CLEAR + TRIGGER::ID.val(self.target_register));
    }
}
