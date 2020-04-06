use core::convert::TryFrom;

pub use registers::*;

mod registers;

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
