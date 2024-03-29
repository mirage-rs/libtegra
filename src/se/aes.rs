use byteorder::{ByteOrder, LE};
use tock_registers::{fields::FieldValue, interfaces::*};

use crate::arm;
use crate::se::constants::*;
use crate::se::core::*;
use crate::se::registers::*;
use crate::se::utils::trigger_single_block_operation;

macro_rules! init_aes {
    ($registers:ident, $encrypt:expr, $dest:ident) => {
        // Configure the hardware to perform an AES operation.
        $registers.SE_CONFIG_0.write(
            SE_CONFIG_0::ENC_MODE::Aes128
                + SE_CONFIG_0::DEC_MODE::Aes128
                + SE_CONFIG_0::ENC_ALG.val($encrypt as u32)
                + SE_CONFIG_0::DEC_ALG.val((!$encrypt) as u32)
                + SE_CONFIG_0::DESTINATION::$dest,
        );
    };
}

macro_rules! aes_config {
    ($registers:ident, $slot:expr, $encrypt:expr, $cntn:ident, $vctram:ident, $input:ident, $xor:ident, $hash:ident) => {
        $registers.SE_CRYPTO_CONFIG_0.write(
            SE_CRYPTO_CONFIG_0::MEMIF::Ahb
                + SE_CRYPTO_CONFIG_0::KEY_INDEX.val($slot)
                + SE_CRYPTO_CONFIG_0::CTR_CNTN::$cntn
                + SE_CRYPTO_CONFIG_0::KEYSCH_BYPASS::CLEAR
                + SE_CRYPTO_CONFIG_0::CORE_SEL.val($encrypt as u32)
                + SE_CRYPTO_CONFIG_0::IV_SELECT::Original
                + SE_CRYPTO_CONFIG_0::VCTRAM_SEL::$vctram
                + SE_CRYPTO_CONFIG_0::INPUT_SEL::$input
                + SE_CRYPTO_CONFIG_0::XOR_POS::$xor
                + SE_CRYPTO_CONFIG_0::HASH_ENB::$hash,
        );
    };
}

/// Representation of the different modes of operation for the AES algorithm
/// for use in Security Engine operations.
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    /// 128-bit AES operation.
    Aes128,
    /// 192-bit AES operation.
    Aes192,
    /// 256-bit AES operation.
    Aes256,
}

impl Mode {
    pub(crate) fn get_field_value(self) -> FieldValue<u32, SE_CONFIG_0::Register> {
        match self {
            Mode::Aes128 => SE_CONFIG_0::ENC_MODE::Aes128 + SE_CONFIG_0::DEC_MODE::Aes128,
            Mode::Aes192 => SE_CONFIG_0::ENC_MODE::Aes192 + SE_CONFIG_0::DEC_MODE::Aes192,
            Mode::Aes256 => SE_CONFIG_0::ENC_MODE::Aes256 + SE_CONFIG_0::DEC_MODE::Aes256,
        }
    }
}

#[inline(always)]
fn configure_aes_cmac(regs: &Registers, slot: u32, enc: bool) {
    aes_config!(regs, slot, enc, CLEAR, InitAesOut, Memory, Top, SET);
}

#[inline(always)]
fn configure_aes_ecb(regs: &Registers, slot: u32, enc: bool) {
    aes_config!(regs, slot, enc, CLEAR, Memory, Memory, Bypass, CLEAR);
}

#[inline(always)]
fn configure_aes_cbc_encrypt(regs: &Registers, slot: u32, enc: bool) {
    aes_config!(regs, slot, enc, CLEAR, InitAesOut, Memory, Top, CLEAR);
}

#[inline(always)]
fn configure_aes_cbc_decrypt(regs: &Registers, slot: u32, enc: bool) {
    aes_config!(regs, slot, enc, CLEAR, InitPrevMem, Memory, Bottom, CLEAR);
}

#[inline(always)]
fn configure_aes_ctr(regs: &Registers, slot: u32, enc: bool) {
    aes_config!(regs, slot, enc, SET, Memory, LinearCtr, Bottom, CLEAR);
}

pub fn clear_keyslot(registers: &Registers, slot: u32) {
    for i in 0..aes::BLOCK_SIZE {
        // Select the next word in the keyslot.
        registers.SE_CRYPTO_KEYTABLE_ADDR_0.write(
            SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEY_SLOT.val(slot)
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_WORD.val(i as u32),
        );

        // Zero out the word in the keyslot.
        registers.SE_CRYPTO_KEYTABLE_DATA_0.set(0);
    }
}

pub fn set_key(registers: &Registers, slot: u32, key: &[u8]) {
    for (i, c) in key.chunks(aes::BLOCK_SIZE >> 2).enumerate() {
        // Select the next word in the keyslot.
        registers.SE_CRYPTO_KEYTABLE_ADDR_0.write(
            SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEY_SLOT.val(slot)
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEYIV_SEL::Key
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_WORD.val(i as u32),
        );

        // Fill the word in the keyslot.
        registers.SE_CRYPTO_KEYTABLE_DATA_0.set(LE::read_u32(c));
    }
}

pub fn get_key(registers: &Registers, slot: u32, key: &mut [u8]) {
    for (i, c) in key.chunks_mut(aes::BLOCK_SIZE >> 2).enumerate() {
        // Select the next word in the keyslot.
        registers.SE_CRYPTO_KEYTABLE_ADDR_0.write(
            SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEY_SLOT.val(slot)
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEYIV_SEL::Key
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_WORD.val(i as u32),
        );

        // Read the word from the keyslot.
        LE::write_u32(c, registers.SE_CRYPTO_KEYTABLE_DATA_0.get());
    }
}

pub fn clear_key_iv(registers: &Registers, slot: u32) {
    for i in 0..aes::BLOCK_SIZE >> 2 {
        // Select the next original IV word in the keyslot.
        registers.SE_CRYPTO_KEYTABLE_ADDR_0.write(
            SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEY_SLOT.val(slot)
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEYIV_SEL::Iv
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_IV_SEL::OriginalIv
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEY_WORD.val(i as u32),
        );

        // Zero out the original IV word in the keyslot.
        registers.SE_CRYPTO_KEYTABLE_DATA_0.set(0);

        // Select the next updated IV word in the keyslot.
        registers.SE_CRYPTO_KEYTABLE_ADDR_0.write(
            SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEY_SLOT.val(slot)
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEYIV_SEL::Iv
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_IV_SEL::UpdatedIv
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEY_WORD.val(i as u32),
        );

        // Zero out the updated IV word in the keyslot.
        registers.SE_CRYPTO_KEYTABLE_DATA_0.set(0);
    }
}

fn set_iv(registers: &Registers, slot: u32, iv: &[u8]) {
    assert_eq!((iv.len() % aes::BLOCK_SIZE) >> 2, 0);

    for (i, c) in iv.chunks(aes::BLOCK_SIZE >> 2).enumerate() {
        // Select the next original IV word in the keyslot.
        registers.SE_CRYPTO_KEYTABLE_ADDR_0.write(
            SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEY_SLOT.val(slot)
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEYIV_SEL::Iv
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_IV_SEL::OriginalIv
                + SE_CRYPTO_KEYTABLE_ADDR_0::KEYIV_KEY_WORD.val(i as u32),
        );

        // Fill the original IV word in the keyslot.
        registers.SE_CRYPTO_KEYTABLE_DATA_0.set(LE::read_u32(c));
    }
}

fn set_counter(registers: &Registers, counter: &[u8; aes::BLOCK_SIZE]) {
    // Copy the given counter words to the linear CTR registers.
    for (i, c) in counter.chunks(aes::BLOCK_SIZE >> 2).enumerate() {
        registers.SE_CRYPTO_LINEAR_CTR_0[i].set(LE::read_u32(c));
    }
}

fn expand_key(subkey: &mut [u8]) {
    // Shift everything left one bit.
    let mut previous = 0;
    for i in (0..aes::BLOCK_SIZE).rev() {
        let lsb = subkey[i] >> 7;
        subkey[i] = (subkey[i] << 1) | previous;
        previous = lsb;
    }

    // XOR with Rb, if necessary.
    if previous != 0 {
        subkey[aes::BLOCK_SIZE - 1] ^= aes::RB;
    }
}

fn read_cmac_result(registers: &Registers, output: &mut [u8]) {
    for i in 0..output.len() >> 2 {
        let word = registers.SE_HASH_RESULT_0[i].get();
        LE::write_u32(&mut output[i << 2..], word);
    }
}

pub fn set_encrypted_key(
    registers: &Registers,
    dest_slot: u32,
    kek_slot: u32,
    key: &[u8],
    mode: Mode,
) -> Result<(), OperationError> {
    // Configure an AES-ECB operation to keytable.
    init_aes!(registers, false, KeyTable);
    configure_aes_ecb(registers, kek_slot, false);
    registers.SE_CONFIG_0.modify(mode.get_field_value());

    // Configure an AES operation on a single block.
    registers.SE_CRYPTO_LAST_BLOCK_0.set(0);

    // Select the destination keyslot.
    registers.SE_CRYPTO_KEYTABLE_DST_0.write(
        SE_CRYPTO_KEYTABLE_DST_0::KEY_INDEX.val(dest_slot)
            + SE_CRYPTO_KEYTABLE_DST_0::DST_WORD_QUAD::Keys03,
    );

    // Ensure cache coherency so the SE sees the correct data.
    unsafe {
        arm::cache::flush_data_cache(key, key.len());
        #[cfg(target_arch = "aarch64")]
        cortex_a::asm::barrier::dsb(cortex_a::asm::barrier::ISH);
    }

    // Prepare the linked lists and kick off the operation.
    let source_ll = LinkedList::from(key);
    let mut destination_ll = LinkedList::default();
    start_normal_operation(registers, &source_ll, &mut destination_ll)
}

pub fn do_cmac_operation(
    registers: &Registers,
    slot: u32,
    source: &[u8],
    destination: &mut [u8],
    mode: Mode,
) -> Result<(), OperationError> {
    // Determine the amount of blocks to generate.
    let nblocks = source.len() / aes::BLOCK_SIZE;
    let aligned_size = nblocks * aes::BLOCK_SIZE;
    let unaligned_size = source.len() - aligned_size;

    // Create the AES subkey for CMAC.
    let subkey = {
        // Encrypt a block of zeroes using AES-ECB.
        let mut key = [0; aes::BLOCK_SIZE];
        do_ecb_operation(registers, true, slot, &[0; aes::BLOCK_SIZE], &mut key, mode)?;

        // Expand the key into a subkey and prepare for last block, if necessary.
        expand_key(&mut key);
        if unaligned_size != aes::BLOCK_SIZE {
            expand_key(&mut key);
        }

        key
    };

    // Configure an AES-CMAC operation to hashreg.
    init_aes!(registers, true, HashReg);
    configure_aes_cmac(registers, slot, true);
    registers.SE_CONFIG_0.modify(mode.get_field_value());

    // Set the IV to zeroes.
    set_iv(registers, slot, &[0; aes::BLOCK_SIZE]);

    // Process all aligned blocks first.
    if aligned_size > 0 {
        // Load in the number of blocks to generate.
        registers.SE_CRYPTO_LAST_BLOCK_0.set((nblocks - 1) as u32);

        // Prepare the linked lists and kick off the operation.
        let source_ll = LinkedList::from(&source[..aligned_size]);
        let mut destination_ll = LinkedList::default();
        start_normal_operation(registers, &source_ll, &mut destination_ll)?;

        // Select the updated IV value after that.
        registers
            .SE_CRYPTO_CONFIG_0
            .modify(SE_CRYPTO_CONFIG_0::IV_SELECT::Updated);
    }

    // Process the last block.
    {
        // Configure an operation on a single block.
        registers.SE_CRYPTO_LAST_BLOCK_0.set(0);

        // Prepare the last block.
        let mut last_block = {
            let mut block = [0; aes::BLOCK_SIZE];
            if unaligned_size < aes::BLOCK_SIZE {
                block[unaligned_size] = 0x80;
            }
            block.copy_from_slice(&source[aligned_size..]);

            block
        };

        // XOR the block with the subkey.
        for (x, y) in last_block.iter_mut().zip(subkey.iter().cycle()) {
            *x ^= *y;
        }

        // Ensure data cache coherency to get the correct output data.
        unsafe {
            arm::cache::flush_data_cache(&last_block[..], last_block.len());
            #[cfg(target_arch = "aarch64")]
            cortex_a::asm::barrier::dsb(cortex_a::asm::barrier::ISH);
        }

        // Prepare the linked lists and kick off the operation.
        let source_ll = LinkedList::from(&last_block[..]);
        let mut destination_ll = LinkedList::default();
        start_normal_operation(registers, &source_ll, &mut destination_ll)?;
    }

    // Copy back the output into the destination buffer.
    read_cmac_result(registers, destination);

    Ok(())
}

pub fn do_ecb_operation(
    registers: &Registers,
    encrypt: bool,
    slot: u32,
    source: &[u8; aes::BLOCK_SIZE],
    destination: &mut [u8; aes::BLOCK_SIZE],
    mode: Mode,
) -> Result<(), OperationError> {
    // Configure an AES-ECB operation to memory.
    init_aes!(registers, encrypt, Memory);
    configure_aes_ecb(registers, slot, encrypt);
    if encrypt {
        registers.SE_CONFIG_0.modify(mode.get_field_value());
    }

    // Execute the operation.
    trigger_single_block_operation(registers, &source[..], &mut destination[..])
}

pub fn do_cbc_operation(
    registers: &Registers,
    encrypt: bool,
    slot: u32,
    source: &[u8],
    destination: &mut [u8],
    iv: &[u8; aes::BLOCK_SIZE],
    mode: Mode,
) -> Result<(), OperationError> {
    // Determine the amount of blocks to generate.
    let nblocks = source.len() / aes::BLOCK_SIZE;
    let aligned_size = nblocks * aes::BLOCK_SIZE;

    // Configure an AES-CBC operation to memory.
    init_aes!(registers, encrypt, Memory);
    if encrypt {
        configure_aes_cbc_encrypt(registers, slot, true);
    } else {
        configure_aes_cbc_decrypt(registers, slot, false);
    }
    registers.SE_CONFIG_0.modify(mode.get_field_value());

    // Set the IV.
    set_iv(registers, slot, &iv[..]);

    // Load in the number of blocks to process.
    registers.SE_CRYPTO_LAST_BLOCK_0.set((nblocks - 1) as u32);

    // Prepare the linked lists and kick off the operation.
    let source_ll = LinkedList::from(&source[..aligned_size]);
    let mut destination_ll = LinkedList::from(destination as &[u8]);
    start_normal_operation(registers, &source_ll, &mut destination_ll)
}

pub fn do_ctr_operation(
    registers: &Registers,
    encrypt: bool,
    slot: u32,
    source: &[u8],
    destination: &mut [u8],
    iv: &[u8; aes::BLOCK_SIZE],
    mode: Mode,
) -> Result<(), OperationError> {
    // Determine the amount of blocks to generate.
    let nblocks = source.len() / aes::BLOCK_SIZE;
    let aligned_size = nblocks * aes::BLOCK_SIZE;

    // XXX: Nintendo does it. I have no idea why this needs to happen.
    registers.SE_SPARE_0.set(1);

    // Configure an AES-CTR operation to memory.
    init_aes!(registers, encrypt, Memory);
    configure_aes_ctr(registers, slot, encrypt);
    registers.SE_CONFIG_0.modify(mode.get_field_value());

    // Initialize the counter.
    set_counter(registers, iv);

    // Process all aligned blocks first.
    if aligned_size > 0 {
        // Load in the number of blocks to process.
        registers.SE_CRYPTO_LAST_BLOCK_0.set((nblocks - 1) as u32);

        // Prepare the linked lists and kick off the operation.
        let source_ll = LinkedList::from(&source[..aligned_size]);
        let mut destination_ll = LinkedList::from(&destination[..aligned_size]);
        start_normal_operation(registers, &source_ll, &mut destination_ll)?;

        // On AArch64, synchronize the data between the SE and the CPU.
        #[cfg(target_arch = "aarch64")]
        unsafe {
            cortex_a::asm::barrier::dsb(cortex_a::asm::barrier::ISH);
        }
    }

    // Process the last unaligned block, if necessary.
    trigger_single_block_operation(
        registers,
        &source[aligned_size..],
        &mut destination[aligned_size..],
    )
}
