use core::fmt;

use alloy_primitives::{hex, B256};
use crate::rlp::RlpNodes;


/// An extension node in an Ethereum Merkle Patricia Trie.
///
/// An intermediate node that exists solely to compress the trie's paths. It contains a path segment
/// (a shared prefix of keys) and a single child pointer. Essentially, an extension node can be
/// thought of as a shortcut within the trie to reduce its overall depth.
///
/// The purpose of an extension node is to optimize the trie structure by collapsing multiple nodes
/// with a single child into one node. This simplification reduces the space and computational
/// complexity when performing operations on the trie.
#[derive(PartialEq, Eq, Clone)]
pub struct ExtensionNode {
    /// The key for this extension node.
    pub key: u8,
    /// A pointer to the child node.
    pub child: RlpNodes,
}

impl fmt::Debug for ExtensionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("ExtensionNode");
        
        debug_struct
            .field("key", &format!("0x{:02X}", self.key))
            .field("upper_nibble", &self.get_upper_nibble())
            .field("lower_nibble", &self.get_lower_nibble());

        if let Some(hash) = self.child_hash() {
            debug_struct.field("child_hash", &hex::encode(hash));
        } else {
            debug_struct.field("child_data", &hex::encode(&self.child));
        }

        debug_struct.finish()
    }
}




impl ExtensionNode {
    /// Creates a new extension node with the given key and a pointer to the child.
    pub const fn new(key: u8, child: RlpNodes) -> Self {
        Self { key, child }
    }

    /// Extracts a nibble (4 bits) from the key based on the flag.
    /// If the flag is true, it extracts the upper nibble.
    /// If the flag is false, it extracts the lower nibble.
    pub const fn extract_key(key: u8, flag: bool) -> (u8, bool) {
        if flag {
            // Extract upper nibble
            (key >> 4, false)
        } else {
            // Extract lower nibble
            (key & 0x0F, true)
        }
    }


    /// Gets the upper nibble (first 4 bits) of the key.
    pub fn get_upper_nibble(&self) -> u8 {
        self.key >> 4
    }

    /// Gets the lower nibble (last 4 bits) of the key.
    pub fn get_lower_nibble(&self) -> u8 {
        self.key & 0x0F
    }

    /// Checks if the child node is stored as a hash.
    ///
    /// In the Merkle Patricia Trie, nodes larger than 32 bytes are stored
    /// as their keccak256 hash in the trie structure.
    pub fn has_hashed_child(&self) -> bool {
        self.child.as_hash().is_some()
    }

    /// Gets the hash of the child node if it's stored as a hash.
    pub fn child_hash(&self) -> Option<B256> {
       self.child.as_hash()
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_node_creation() {
        let child = RlpNodes::from_raw(&[0x01, 0x02, 0x03]).unwrap();
        let node = ExtensionNode::new(0xAB, child.clone());
        
        assert_eq!(node.key, 0xAB);
        assert_eq!(node.child, child);
        assert_eq!(node.get_upper_nibble(), 0xA);
        assert_eq!(node.get_lower_nibble(), 0xB);
    }

    #[test]
    fn test_nibble_extraction() {
        let (upper, more_after_upper) = ExtensionNode::extract_key(0xAB, true);
        assert_eq!(upper, 0xA);
        assert!(!more_after_upper);
        
        let (lower, more_after_lower) = ExtensionNode::extract_key(0xAB, false);
        assert_eq!(lower, 0xB);
        assert!(more_after_lower);
    }

    #[test]
    fn test_child_hash_detection() {
        let small_child = RlpNodes::from_raw(&[0x01, 0x02, 0x03]).unwrap();
        let small_node = ExtensionNode::new(0xAB, small_child);
        assert!(!small_node.has_hashed_child());
    
        // Hash node (proper RLP format)
        let mut hash_data = [0u8; 33];
        // RLP prefix for hash
        hash_data[0] = 0x80; 
        hash_data[1..33].copy_from_slice(&[0x42; 32]); 
        
        let hash_child = RlpNodes::from_raw(&hash_data).unwrap();
        let hash_node = ExtensionNode::new(0xCD, hash_child);
        assert!(hash_node.has_hashed_child());
    }
}