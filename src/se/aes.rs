use byteorder::{ByteOrder, LE};
use register::FieldValue;

use crate::se::constants::*;
use crate::se::core::*;
use crate::se::registers::*;

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
    assert_eq!(iv.len() % aes::BLOCK_SIZE >> 2, 0);

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

pub fn do_ecb_operation(
    registers: &Registers,
    encrypt: bool,
    slot: u32,
    source: &[u8; aes::BLOCK_SIZE],
    destination: &mut [u8; aes::BLOCK_SIZE],
    mode: Mode,
) -> Result<(), OperationError> {
    #[cfg(target_arch = "aarch64")]
    use crate::se::utils::*;

    // Configure an AES-ECB operation to memory.
    init_aes!(registers, encrypt, Memory);
    configure_aes_ecb(registers, slot, encrypt);
    if encrypt {
        registers.SE_CONFIG_0.modify(mode.get_field_value());
    }

    // Configure an AES operation on a single block.
    registers.SE_CRYPTO_LAST_BLOCK_0.set(0);

    // On AArch64, we need to cache-align the source buffer to ensure data cache coherency.
    #[allow(unreachable_patterns)]
    let source = match () {
        #[cfg(target_arch = "aarch64")]
        () => CachePad::from(&source[..]).into_inner(),
        () => *source,
    };

    // On AArch64, we need to cache-align the output buffer to ensure data cache coherency.
    #[allow(unreachable_patterns)]
    let output = match () {
        #[cfg(target_arch = "aarch64")]
        () => CachePad::new([0u8; aes::BLOCK_SIZE]).into_inner(),
        () => [0; aes::BLOCK_SIZE],
    };

    // Prepare the linked lists and kick off the operation.
    let source_ll = LinkedList::from(&source[..]);
    let mut destination_ll = LinkedList::from(&output[..]);
    start_normal_operation(registers, &source_ll, &mut destination_ll)?;

    // On AArch64, ensure data cache coherency to get the correct output data.
    #[cfg(target_arch = "aarch64")]
    unsafe {
        use cortex_a::barrier;

        barrier::dsb(barrier::ISH);
        flush_data_cache_line(source.as_ptr() as usize);
        barrier::dsb(barrier::ISH);
    }

    // Copy the output back into the destination buffer.
    destination[..].copy_from_slice(&output[..]);

    Ok(())
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
    let mut destination_ll = LinkedList::from(&destination[..]);
    start_normal_operation(registers, &source_ll, &mut destination_ll)
}
