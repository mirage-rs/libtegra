use register::{mmio::ReadWrite, register_structs};

use crate::memory_map::FUSE;

/// A pointer to the FUSE register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = FUSE as *const Registers;

// TODO: Bitfields.

// TODO: Are really all of them ReadWrite?

register_structs! {
    /// Representation of the FUSE registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x000 => pub FUSE_CTRL: ReadWrite<u32>),
        (0x004 => pub FUSE_ADDR: ReadWrite<u32>),
        (0x008 => pub FUSE_RDATA: ReadWrite<u32>),
        (0x00C => pub FUSE_WDATA: ReadWrite<u32>),
        (0x010 => pub FUSE_TIME_RD1: ReadWrite<u32>),
        (0x014 => pub FUSE_TIME_RD2: ReadWrite<u32>),
        (0x018 => pub FUSE_TIME_PGM1: ReadWrite<u32>),
        (0x01C => pub FUSE_TIME_PGM2: ReadWrite<u32>),
        (0x020 => pub FUSE_PRIV2INTFC: ReadWrite<u32>),
        (0x024 => pub FUSE_FUSEBYPASS: ReadWrite<u32>),
        (0x028 => pub FUSE_PRIVATEKEYDISABLE: ReadWrite<u32>),
        (0x02C => pub FUSE_DISABLEREGPROGRAM: ReadWrite<u32>),
        (0x030 => pub FUSE_WRITE_ACCESS_SW: ReadWrite<u32>),
        (0x034 => pub FUSE_PWR_GOOD_SW: ReadWrite<u32>),
        (0x038 => _reserved_to_end),
        (0x100 => @END),
    }
}
