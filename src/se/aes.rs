use crate::se::constants::*;
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
