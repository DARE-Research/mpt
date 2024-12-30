use crate::rlp::RlpNodes;
use core::fmt;

/// A branch node in an Ethereum Merkle Patricia Trie.
///
/// Contains up to 16 children (one for each hex digit) and an optional value.
/// Uses byte-sized indices for alignment and performance, even though only
/// nibble (4-bit) values are valid indices.
pub struct BranchNode {
    // 16 potential children, one for each hex digi, this should be nibble but for allignment
    // issues and performance 1 byte
    children: [Vec<RlpNodes>; 16],

    value: Option<Vec<u8>>,
}

impl fmt::Debug for BranchNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BranchNode")
            .field("children", &self.children.iter())
            .field("value", &self.value.iter())
            .field("first_child_index", &self.children.get(1))
            .finish()
    }
}

impl BranchNode {
    /// Creates a new branch node with the given children
    pub const fn new(children: [Vec<RlpNodes>; 16], value: Option<Vec<u8>>) -> Self {
        Self { children, value }
    }

    pub fn rlp_payload_length(&self) -> usize {
        let mut payload_length = 0;
        for i in 0..16 {
            payload_length += if self.children[i].is_empty() {
                1
            } else {
                self.children[i].len()
            };
        }
        payload_length += match &self.value {
            Some(value) => value.len(),
            None => 1,
        };
        payload_length
    }

    pub fn get_child(&self, index: usize) -> Option<&Vec<RlpNodes>> {
        if index >= 16 {
            return None;
        }
        let child = &self.children[index];
        if !child.is_empty() {
            Some(child)
        } else {
            None
        }
    }

    pub fn batch_get_children(&self, indices: &[usize]) -> Vec<Option<&Vec<RlpNodes>>> {
        indices
            .iter()
            .map(|&index| {
                if index < 16 && !self.children[index].is_empty() {
                    Some(&self.children[index])
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn batch_set_children(&mut self, updates: Vec<(usize, Vec<RlpNodes>)>) {
        //why vec and not a slice: this was done to avoid a clone on every insertion
        for (index, node) in updates {
            debug_assert!(index < 16, "Branch node index out of bounds");
            self.children[index] = node;
        }
    }

    pub fn set_child(&mut self, node: Vec<RlpNodes>, index: usize) {
        debug_assert!(index < 16, "Branch node index out of bounds");
        self.children[index] = node;
    }
}

impl Default for BranchNode {
    fn default() -> Self {
        Self {
            children: Default::default(),
            value: None,
        }
    }
}
