use core::convert::TryFrom;

use crate::{ahb::mem, timer};

pub use registers::*;

mod registers;

const AES_BLOCK_SIZE: u32 = 16;

/// Security Engine operation control opcodes.
pub mod opcodes {
    pub const ABORT: u32 = 0;
    pub const START: u32 = 1;
    pub const RESTART: u32 = 2;
    pub const CTX_SAVE: u32 = 3;
    pub const RESTART_IN: u32 = 4;
}

/// Address information of a DMA buffer, representing a node of the Security Engine Linked List.
#[derive(Debug, Default)]
#[repr(C)]
struct AddressInfo {
    /// The DMA buffer address.
    pub address: u32,
    /// The data length stored in DMA buffer.
    pub data_len: u32,
}

impl<'a> From<&'a [u8]> for AddressInfo {
    fn from(buffer: &[u8]) -> Self {
        let address = u32::try_from(buffer.as_ptr() as usize)
            .expect("Address does not fit an u32!");
        let data_len = buffer.len() as u32;

        AddressInfo {
            address,
            data_len,
        }
    }
}

assert_eq_size!(AddressInfo, [u32; 2]);

/// Representation of the Security Engine Linked List.
#[derive(Debug)]
#[repr(align(4), C)]
struct LinkedList {
    /// The total number of entries.
    pub entries: u32,
    /// An array of DMA buffer information to be processed.
    pub address_info: [AddressInfo; 0x4],
}

impl LinkedList {
    /// Inserts another DMA buffer into the linked list so it can be processed.
    ///
    /// If the size of the list has already reached its limits, this function
    /// will return an error.
    pub fn append(&mut self, entry: &[u8]) -> Result<(), ()> {
        if self.entries >= 3 {
            return Err(());
        }

        self.entries += 1;
        self.address_info[self.entries as usize] = AddressInfo::from(entry);

        Ok(())
    }
}

impl<'a> From<&'a [u8]> for LinkedList {
    fn from(buffer: &[u8]) -> Self {
        LinkedList {
            entries: 0,
            address_info: [
                AddressInfo::from(buffer),
                AddressInfo::default(),
                AddressInfo::default(),
                AddressInfo::default(),
            ],
        }
    }
}

assert_eq_size!(LinkedList, [u32; 9]);

/// Enumeration of potential Security Engine errors
/// that may occur during internal operations on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperationError {
    /// A timeout has occurred during an operation.
    Timeout,
    /// A timeout has occurred during an AHB transfer.
    AhbTimeout,
    /// An exception was raised during an operation.
    Exception,
}

/// Waits for the Security Engine to enter idle state before starting the next operation.
///
/// This function also clears pending interrupts from previous operations.
/// In case of a timeout, an error will be returned.
fn prepare_operation() -> Result<(), OperationError> {
    let engine = unsafe { &*REGISTERS };

    // Disable interrupts to be issued by a Security Engine operation.
    engine.SE_INT_ENABLE_0.set(0);

    // Wait for the previous operation to finish.
    let timeout = timer::get_milliseconds() + 100;
    while engine.SE_STATUS_0.get() != 0 {
        if timer::get_milliseconds() > timeout {
            return Err(OperationError::Timeout);
        }
    }

    // Clear any pending interrupts from the previous operation.
    engine.SE_INT_STATUS_0.set(engine.SE_INT_STATUS_0.get());

    Ok(())
}

/// Checks that a Security Engine operation has been fully completed after kickoff.
///
/// This function ensures that no interrupts are pending, no exceptions have occurred,
/// the AHB transfer has terminated and that the Security Engine has entered idle state.
fn complete_operation() -> Result<(), OperationError> {
    let engine = unsafe { &*REGISTERS };
    let ahb = unsafe { &*ahb::REGISTERS };
    let mut timeout;

    // Poll the interrupt register to ensure the operation has completed.
    timeout = timer::get_milliseconds() + 100;
    while (engine.SE_INT_STATUS_0.get() & 0x10) == 0 {
        if timer::get_milliseconds() > timeout {
            return Err(OperationError::Timeout);
        }
    }

    // Poll the status register to ensure the operation has completed.
    timeout = timer::get_milliseconds() + 100;
    while engine.SE_STATUS_0.get() != 0 {
        if timer::get_milliseconds() > timeout {
            return Err(OperationError::Timeout);
        }
    }

    // Ensure the AHB Bus transfer has completed.
    timeout = timer::get_milliseconds() + 100;
    while (ahb.AHB_ARBITRATION_AHB_MEM_WRQUE_MST_ID_0.get() & 0x6000) != 0 {
        if timer::get_milliseconds() > timeout {
            return Err(OperationError::AhbTimeout);
        }
    }

    // Ensure that no errors occurred during the operation.
    if engine.SE_ERR_STATUS_0.get() != 0 {
        return Err(OperationError::Exception);
    }

    Ok(())
}

/// Starts an internal Security Engine operation.
///
/// Given an opcode, two [`LinkedList`]s holding data buffers and the
/// actual amount of bytes to be processed, this function triggers
/// a cryptographic operation within the Security Engine.
///
/// [`LinkedList`]: struct.LinkedList.html
fn trigger_operation(
    opcode: u32,
    source: &LinkedList,
    destination: &LinkedList,
    nbytes: u32,
) -> Result<(), OperationError> {
    let engine = unsafe { &*REGISTERS };

    // Compute memory addresses of the linked lists.
    let source_address = u32::try_from(source as *const LinkedList as usize)
        .expect("Address does not fit an u32!");
    let destination_address = u32::try_from(destination as *const LinkedList as usize)
        .expect("Address does not fit an u32!");

    // Calculate the amount of blocks to be processed.
    let nblocks = nbytes / AES_BLOCK_SIZE;

    // Load in the linked lists for input and output.
    engine.SE_IN_LL_ADDR_0.set(source_address);
    engine.SE_OUT_LL_ADDR_0.set(destination_address);

    // Check that the previous operation has completed.
    prepare_operation()?;

    // Program SE operation size.
    if nblocks > 0 {
        engine.SE_CRYPTO_LAST_BLOCK_0.set(nblocks - 1);
    }

    // Start hardware operation.
    engine.SE_OPERATION_0.set(opcode);

    // Wait for operation to complete.
    complete_operation()?;

    Ok(())
}

/// Triggers a normal Security Engine operation.
///
/// See [`trigger_operation`] for further explanation.
///
/// [`trigger_operation`]: fn.trigger_operation.html
fn start_normal_operation(
    source: &LinkedList,
    destination: &LinkedList,
    nbytes: u32
) -> Result<(), OperationError> {
    trigger_operation(opcodes::START, source, destination, nbytes)
}

/// Triggers a Security Engine operation where the crypto context will be saved afterwards.
///
/// See [`trigger_operation`] for further explanation.
///
/// [`trigger_operation`]: fn.trigger_operation.html
fn start_context_save_operation(
    source: &LinkedList,
    destination: &LinkedList,
    nbytes: u32
) -> Result<(), OperationError> {
    trigger_operation(opcode::CTX_SAVE, source, destination, nbytes)
}
