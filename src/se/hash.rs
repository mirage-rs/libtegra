use byteorder::{ByteOrder, BE, LE};

use crate::se::registers::*;

use tock_registers::interfaces::*;

macro_rules! init_sha {
    ($registers:ident, $mode:ident) => {
        // Configure the hardware to perform a SHA hashing operation.
        $registers.SE_CONFIG_0.write(
            SE_CONFIG_0::HASH_MODE::$mode
                + SE_CONFIG_0::DEC_MODE::Aes128
                + SE_CONFIG_0::ENC_ALG::Sha
                + SE_CONFIG_0::DEC_ALG::Nop
                + SE_CONFIG_0::DESTINATION::HashReg,
        );
        $registers
            .SE_SHA_CONFIG_0
            .write(SE_SHA_CONFIG_0::HW_INIT_HASH::SET);
    };
}

macro_rules! gen_sha_impl {
    ($size:tt, $out_len:expr) => {
        ::paste::paste! {
            #[doc = "Calculates a SHA" $size " hash over a given buffer of data."]
            pub fn [<calculate_sha $size>](
                &self,
                source: &[u8],
                output: &mut [u8; $out_len],
            ) -> Result<(), OperationError> {
                let engine = unsafe { &*self.registers };

                // Configure the hardware for SHA256 hashing.
                init_sha!(engine, [<Sha $size>]);
                hash::set_source_size(engine, source.len() as u32);

                // Prepare the linked lists and kick off the operation.
                let source_ll = LinkedList::from(source);
                let mut destination_ll = LinkedList::default();
                start_normal_operation(engine, &source_ll, &mut destination_ll)?;

                // Read and copy back the resulting hash.
                hash::read_result(engine, output, true);

                Ok(())
            }
        }
    };
    ($size:tt) => {
        gen_sha_impl!($size, $size >> 3);
    };
}

pub fn set_source_size(registers: &Registers, size: u32) {
    // Set the message size.
    registers.SE_SHA_MSG_LENGTH_0[0].set(size << 3);
    registers.SE_SHA_MSG_LENGTH_0[1].set(0);
    registers.SE_SHA_MSG_LENGTH_0[2].set(0);
    registers.SE_SHA_MSG_LENGTH_0[3].set(0);

    // Set the message remaining size.
    registers.SE_SHA_MSG_LEFT_0[0].set(size << 3);
    registers.SE_SHA_MSG_LEFT_0[1].set(0);
    registers.SE_SHA_MSG_LEFT_0[2].set(0);
    registers.SE_SHA_MSG_LEFT_0[3].set(0);
}

pub fn read_result(registers: &Registers, output: &mut [u8], byteswap: bool) {
    for i in 0..output.len() >> 2 {
        let word = registers.SE_HASH_RESULT_0[i].get();

        if byteswap {
            BE::write_u32(&mut output[i << 2..], word);
        } else {
            LE::write_u32(&mut output[i << 2..], word);
        }
    }
}
