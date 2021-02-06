use byteorder::{ByteOrder, BE};
use register::FieldValue;

use crate::se::constants::*;
use crate::se::core::*;
use crate::se::registers::*;

#[derive(Clone, Copy)]
pub struct KeyInfo {
    modulus_size: u32,
    exponent_size: u32,
}

impl KeyInfo {
    pub const fn new() -> Self {
        KeyInfo {
            modulus_size: 0,
            exponent_size: 0,
        }
    }

    pub fn update(&mut self, modulus_size: usize, exponent_size: usize) {
        self.modulus_size = (modulus_size / 64 - 1) as u32;
        self.exponent_size = (exponent_size / 4) as u32;
    }

    pub fn reset(&mut self) {
        self.modulus_size = 0;
        self.exponent_size = 0;
    }
}

fn clear_keyslot_impl(
    registers: &Registers,
    slot: u32,
    expmode: FieldValue<u32, SE_RSA_KEYTABLE_ADDR_0::Register>,
) {
    for i in 0..rsa::SIZE >> 2 {
        // Select the key slot word.
        registers.SE_RSA_KEYTABLE_ADDR_0.write(
            SE_RSA_KEYTABLE_ADDR_0::INPUT_MODE::FromRegister
                + SE_RSA_KEYTABLE_ADDR_0::KEY_SLOT.val(slot)
                + expmode
                + SE_RSA_KEYTABLE_ADDR_0::WORD_ADDR.val(i as u32),
        );

        // Clear the keyslot word.
        registers.SE_RSA_KEYTABLE_DATA_0.set(0);
    }
}

pub fn clear_keyslot(regs: &Registers, slot: u32) {
    // Clear the modulus.
    clear_keyslot_impl(regs, slot, SE_RSA_KEYTABLE_ADDR_0::EXPMOD_SEL::Modulus);

    // Clear the exponent.
    clear_keyslot_impl(regs, slot, SE_RSA_KEYTABLE_ADDR_0::EXPMOD_SEL::Exponent);
}

fn fill_keyslot_impl(
    registers: &Registers,
    slot: u32,
    expmod: FieldValue<u32, SE_RSA_KEYTABLE_ADDR_0::Register>,
    key: &[u8],
) {
    let nwords = key.len() >> 2;
    for i in 0..nwords {
        // Select the keyslot word.
        registers.SE_RSA_KEYTABLE_ADDR_0.write(
            SE_RSA_KEYTABLE_ADDR_0::INPUT_MODE::FromRegister
                + SE_RSA_KEYTABLE_ADDR_0::KEY_SLOT.val(slot)
                + expmod
                + SE_RSA_KEYTABLE_ADDR_0::WORD_ADDR.val(i as u32),
        );

        // Read the next word and write it to the key slot.
        let word = BE::read_u32(&key[(nwords - 1 - i) << 2..]);
        registers.SE_RSA_KEYTABLE_DATA_0.set(word);
    }
}

pub fn fill_keyslot(registers: &Registers, slot: u32, modulus: &[u8], exponent: &[u8]) {
    // Set the modulus.
    fill_keyslot_impl(
        registers,
        slot,
        SE_RSA_KEYTABLE_ADDR_0::EXPMOD_SEL::Modulus,
        modulus,
    );

    // Set the exponent.
    fill_keyslot_impl(
        registers,
        slot,
        SE_RSA_KEYTABLE_ADDR_0::EXPMOD_SEL::Exponent,
        exponent,
    );
}

pub fn encrypt(
    registers: &Registers,
    key_info: &KeyInfo,
    slot: u32,
    source: &[u8],
    destination: &mut [u8],
) -> Result<(), OperationError> {
    // Load in the correct modulus and exponent sizes for the operation.
    registers.SE_RSA_KEY_SIZE_0.set(key_info.modulus_size);
    registers.SE_RSA_EXP_SIZE_0.set(key_info.exponent_size);

    // Configure the hardware to perform RSA encryption.
    registers.SE_CONFIG_0.write(
        SE_CONFIG_0::ENC_MODE::Aes128
            + SE_CONFIG_0::DEC_MODE::Aes128
            + SE_CONFIG_0::ENC_ALG::Rsa
            + SE_CONFIG_0::DEC_ALG::Nop
            + SE_CONFIG_0::DESTINATION::RsaReg,
    );

    // Load in the keyslot to use.
    registers
        .SE_RSA_CONFIG_0
        .write(SE_RSA_CONFIG_0::KEY_SLOT.val(slot));

    // Prepare the linked lists and kick off the operation.
    let source_ll = LinkedList::from(source);
    let mut destination_ll = LinkedList::default();
    start_normal_operation(registers, &source_ll, &mut destination_ll)?;

    // Read back the result of the operation from RSA output.
    let nwords = destination.len() >> 2;
    for i in 0..nwords {
        // Read out the next word and write it to the output buffer.
        let word = registers.SE_RSA_OUTPUT_0[i].get();
        BE::write_u32(&mut destination[(nwords - 1 - i) << 2..], word);
    }

    Ok(())
}
