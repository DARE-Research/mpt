use alloy_primitives::hex;
use core::fmt;

#[derive(PartialEq, Eq, Clone)]
pub struct LeafNode {
    /// The key for this leaf node.
    /// To avoid unecessary padding a byte was used instead of a nibble
    pub key: u8,
    /// The node value.
    pub value: [u8; 33],
}

impl fmt::Debug for LeafNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LeafNode")
            .field("key", &self.key)
            .field("value", &hex::encode(&self.value))
            .finish()
    }
}

impl LeafNode {
    /// Creates a new leaf node with the given key and value.
    pub const fn new(key: u8, value: &[u8; 33]) -> Self {
        let value = *value;
        let key = Self::extract_key(key);
        Self { key, value }
    }

    /// Extracts a nibble (4 bits) from the key based on the flag.
    /// This assumes your computer is little endian
    pub const fn extract_key(key: u8) -> u8 {
        // Extract lower nibble
        key & 0x0F
    }
}
