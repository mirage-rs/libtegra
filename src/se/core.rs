use core::convert::TryFrom;

use crate::{ahb::mem, timer};

use super::{constants::*, registers::*};

/// Address information of a DMA buffer.
///
/// Effectively used as a node of a Security Engine [`LinkedList`].
///
/// [`LinkedList`]: struct.LinkedList.html
#[derive(Debug, Default)]
#[repr(C)]
pub struct AddressInfo {
    /// The start address of the DMA buffer.
    pub address: u32,
    /// The length of data stored in the DMA buffer.
    pub data_len: u32,
}

impl<'a> From<&'a [u8]> for AddressInfo {
    fn from(buffer: &[u8]) -> Self {
        let address =
            u32::try_from(buffer.as_ptr() as usize).expect("Address does not fit an u32!");
        let data_len = buffer.len() as u32;

        AddressInfo { address, data_len }
    }
}

assert_eq_size!(AddressInfo, [u32; 2]);

/// Representation of a Security Engine Linked List.
///
/// Linked Lists are used for I/O within the Security Engine
/// and provide the buffers of data to read input from and
/// output the resulting data to.
#[derive(Debug, Default)]
#[repr(align(4), C)]
pub struct LinkedList {
    /// The total number of DMA entries within this list.
    ///
    /// NOTE: This number may not exceed 3.
    pub entries: u32,
    /// An array of DMA buffer information to be processed.
    pub address_info: [AddressInfo; 0x4],
}

impl LinkedList {
    /// Inserts a given DMA buffer into the Linked List.
    ///
    /// If the total capacity of the list is already exhausted,
    /// this function will return an error.
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

/// Enumeration of potential Security Engine errors that
/// may occur during internal operations on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperationError {
    /// A timeout has occurred during a SE operation.
    Timeout,
    /// A timeout has occurred during an AHB transfer.
    AhbTimeout,
    /// An exception was raised by the SE while processing DMA buffers.
    Exception,
}

/// Waits for the Security Engine to enter idle state before starting the next operation.
///
/// This function also clears pending interrupts from previous operations.
fn prepare_operation(engine: &Registers) -> Result<(), OperationError> {
    // Wait for the previous operation to finish.
    let timeout = timer::get_milliseconds() + 100;
    while engine.SE_STATUS_0.get() != 0 {
        if timer::get_milliseconds() > timeout {
            return Err(OperationError::Timeout);
        }
    }

    // Clear any pending interrupts from the previous operation.
    engine.SE_ERR_STATUS_0.set(engine.SE_ERR_STATUS_0.get());
    engine.SE_INT_STATUS_0.set(engine.SE_INT_STATUS_0.get());

    Ok(())
}

/// Checks whether a Security Engine operation has been fully completed.
///
/// This function ensures that no interrupts are pending, no exceptions
/// have occurred internally, the AHB transfer has terminated and that
/// the Security Engine has entered idle state.
fn complete_operation(engine: &Registers) -> Result<(), OperationError> {
    let ahb = unsafe { &*mem::REGISTERS };

    let mut timeout;

    // Wait until the operation has completed.
    timeout = timer::get_milliseconds() + 100;
    while !engine.SE_INT_STATUS_0.is_set(SE_INT_STATUS_0::SE_OP_DONE) {
        if timer::get_milliseconds() > timeout {
            return Err(OperationError::Timeout);
        }
    }

    // Ensure that no errors occurred.
    if engine.SE_INT_STATUS_0.is_set(SE_INT_STATUS_0::ERR_STAT) {
        return Err(OperationError::Exception);
    }

    // Ensure that the engine has gone back into idle state.
    timeout = timer::get_milliseconds() + 100;
    while !engine.SE_STATUS_0.matches_all(SE_STATUS_0::STATE::Idle) {
        if timer::get_milliseconds() > timeout {
            return Err(OperationError::Timeout);
        }
    }

    // Ensure the AHB bus transfer has completed.
    timeout = timer::get_milliseconds() + 100;
    while (ahb.AHB_ARBITRATION_AHB_MEM_WRQUE_MST_ID_0.get() & 0x6000) != 0 {
        if timer::get_milliseconds() > timeout {
            return Err(OperationError::AhbTimeout);
        }
    }

    // Ensure that no error status is set.
    if engine.SE_ERR_STATUS_0.get() != 0 {
        return Err(OperationError::Exception);
    }

    Ok(())
}

/// Launches a cryptographic Security Engine operation, given two [`LinkedList`]s for I/O.
///
/// [`LinkedList`]: struct.LinkedList.html
pub fn trigger_operation(
    engine: &Registers,
    opcode: u32,
    source: &LinkedList,
    destination: &mut LinkedList,
) -> Result<(), OperationError> {
    // Compute memory addresses of the LLs.
    let source_address =
        u32::try_from(source as *const _ as usize).expect("Address does not fit an u32!");
    let destination_address =
        u32::try_from(destination as *mut _ as usize).expect("Address does not fit an u32!");

    // Load in the LLs.
    engine.SE_IN_LL_ADDR_0.set(source_address);
    engine.SE_OUT_LL_ADDR_0.set(destination_address);

    // Ensure that the previous operation has fully completed.
    prepare_operation(engine)?;

    // Start the hardware operation.
    engine
        .SE_OPERATION_0
        .modify(SE_OPERATION_0::OPCODE.val(opcode));

    // Wait for the operation to complete.
    complete_operation(engine)?;

    Ok(())
}

/// Triggers a regular Security Engine operation.
///
/// See [`trigger_operation`] for further explanation.
///
/// [`trigger_operation`]: fn.trigger_operation.html
#[inline(always)]
pub fn start_normal_operation(
    engine: &Registers,
    source: &LinkedList,
    destination: &mut LinkedList,
) -> Result<(), OperationError> {
    trigger_operation(engine, opcodes::START, source, destination)
}

/// Triggers a Security Engine operation that saves the crypto context afterwards.
///
/// See [`trigger_operation`] for further explanation.
///
/// [`trigger_operation`]: fn.trigger_operation.html
#[inline(always)]
pub fn start_context_save_operation(
    engine: &Registers,
    source: &LinkedList,
    destination: &mut LinkedList,
) -> Result<(), OperationError> {
    trigger_operation(engine, opcodes::CTX_SAVE, source, destination)
}
