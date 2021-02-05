use crate::se::constants::*;
use crate::se::core::*;
use crate::se::registers::*;
use crate::se::utils::trigger_single_block_operation;

macro_rules! init_rng {
    ($registers:ident, $dest:ident, $mode:ident) => {
        // Configure the hardware to do RNG encryption.
        $registers.SE_CONFIG_0.write(
            SE_CONFIG_0::ENC_MODE::Aes128
                + SE_CONFIG_0::DEC_MODE::Aes128
                + SE_CONFIG_0::ENC_ALG::Rng
                + SE_CONFIG_0::DEC_ALG::Nop
                + SE_CONFIG_0::DESTINATION::$dest,
        );

        // Configure the cryptographic operation.
        $registers.SE_CRYPTO_CONFIG_0.write(
            SE_CRYPTO_CONFIG_0::MEMIF::Ahb
                + SE_CRYPTO_CONFIG_0::CTR_CNTN::CLEAR
                + SE_CRYPTO_CONFIG_0::KEYSCH_BYPASS::CLEAR
                + SE_CRYPTO_CONFIG_0::CORE_SEL::Encrypt
                + SE_CRYPTO_CONFIG_0::IV_SELECT::Original
                + SE_CRYPTO_CONFIG_0::VCTRAM_SEL::Memory
                + SE_CRYPTO_CONFIG_0::INPUT_SEL::Random
                + SE_CRYPTO_CONFIG_0::XOR_POS::Bypass
                + SE_CRYPTO_CONFIG_0::HASH_ENB::CLEAR,
        );

        // Configure the RNG to use Entropy as source.
        $registers
            .SE_RNG_CONFIG_0
            .write(SE_RNG_CONFIG_0::SOURCE::Entropy + SE_RNG_CONFIG_0::MODE::$mode);
    };
}

pub fn initialize(registers: &Registers) -> Result<(), OperationError> {
    // Lock the entropy source.
    registers.SE_RNG_SRC_CONFIG_0.write(
        SE_RNG_SRC_CONFIG_0::RO_ENTROPY_SOURCE::SET
            + SE_RNG_SRC_CONFIG_0::RO_ENTROPY_SOURCE_LOCK::SET,
    );

    // Set the reseed interval to force a reseed every 70.000 blocks.
    registers.SE_RNG_RESEED_INTERVAL_0.set(rng::RESEED_INTERVAL);

    // Configure the RNG.
    init_rng!(registers, Memory, ForceInstantiation);

    // Only process a single RNG block to trigger DRBG initialization.
    registers.SE_CRYPTO_LAST_BLOCK_0.set(0);

    let buffer = [0; aes::BLOCK_SIZE];

    // Prepare the linked lists and kick off the operation.
    let source_ll = LinkedList::from(&buffer[..]);
    let mut destination_ll = LinkedList::default();
    start_normal_operation(registers, &source_ll, &mut destination_ll)
}

pub fn set_random_key(registers: &Registers, slot: u32) -> Result<(), OperationError> {
    // Configure the RNG.
    init_rng!(registers, KeyTable, Normal);

    // Configure the keytable to be the low words of the key.
    registers.SE_CRYPTO_KEYTABLE_DST_0.write(
        SE_CRYPTO_KEYTABLE_DST_0::DST_WORD_QUAD::Keys03
            + SE_CRYPTO_KEYTABLE_DST_0::KEY_INDEX.val(slot),
    );

    // Configure an RNG operation on a single block.
    registers.SE_CRYPTO_LAST_BLOCK_0.set(0);

    // Generate the first part of the key.
    start_normal_operation(
        registers,
        &LinkedList::default(),
        &mut LinkedList::default(),
    )?;

    // Configure the keytable to be the high words of the key.
    registers.SE_CRYPTO_KEYTABLE_DST_0.write(
        SE_CRYPTO_KEYTABLE_DST_0::DST_WORD_QUAD::Keys47
            + SE_CRYPTO_KEYTABLE_DST_0::KEY_INDEX.val(slot),
    );

    // Generate the second part of the key.
    start_normal_operation(
        registers,
        &LinkedList::default(),
        &mut LinkedList::default(),
    )
}

pub fn generate_srk(registers: &Registers) -> Result<(), OperationError> {
    // Configure the RNG.
    init_rng!(registers, Srk, ForceReseed);

    // Only process a single block to trigger DRBG initialization.
    registers.SE_CRYPTO_LAST_BLOCK_0.set(0);

    // Kick off the hardware operation.
    start_normal_operation(
        registers,
        &LinkedList::default(),
        &mut LinkedList::default(),
    )
}

pub fn generate_random(registers: &Registers, output: &mut [u8]) -> Result<(), OperationError> {
    // Determine the amount of blocks to generate.
    let nblocks = output.len() / aes::BLOCK_SIZE;
    let aligned_size = nblocks * aes::BLOCK_SIZE;

    // Configure the RNG.
    init_rng!(registers, Memory, Normal);

    // Generate all the aligned blocks first.
    if aligned_size > 0 {
        // Load in the number of blocks to generate.
        registers.SE_CRYPTO_LAST_BLOCK_0.set((nblocks - 1) as u32);

        // Prepare the linked lists and kick off the operation.
        let source_ll = LinkedList::default();
        let mut destination_ll = LinkedList::from(&output[..aligned_size]);
        start_normal_operation(registers, &source_ll, &mut destination_ll)?;
    }

    // Generate a single unaligned block, if necessary.
    trigger_single_block_operation(registers, &[], &mut output[aligned_size..])
}
