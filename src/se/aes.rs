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
