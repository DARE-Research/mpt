# Zero-Copy Merkle Patricia Trie Nodes

An efficient, zero-copy implementation of Ethereum's Modified Merkle Patricia Trie nodes.

⚠️ WARNING: This is an experimental implementation, don't use in production

## Overview

This crate provides memory-efficient implementations of MPT nodes using fixed-size arrays and zero-copy operations. The focus is on minimizing allocations and memory copies while maintaining high performance.

## Node Types

### RlpNodes


`pub  struct  RlpNodes([u8;  33]);`

The base type representing RLP-encoded node data.

-   Fixed-size array (33 bytes) for optimal memory layout
-   Zero-copy slice access 

### BranchNode



`pub  struct  BranchNode  {   children:  [Vec<RlpNodes>;  16], value:  Option<Vec<u8>>, }`

Handles branch paths with up to 16 children. Features:

-   Aligned array storage for performance


### ExtensionNode
`pub  struct  ExtensionNode  {   key:  u8, child:  RlpNodes, }`

Compresses shared path segments. Features:

-   Single-byte key storage using nibbles

| **Operation**                  | **Zero-Copy MPT (ns)** | **Alloy MPT (ns)** | **Improvement**       |
|--------------------------------|------------------------|--------------------|-----------------------|
| **Node Creation**              |                        |                    |                       |
| - New                          | 9.17                   | 15.95              | ~42% faster           |
| **RLP Operations**             |                        |                    |                       |
| - Encode                       | N/A**                  | 47.11              |                       |
| - Decode                       | N/A**                  | 64.01              |                       |
| **Batch Creation**             |                        |                    |                       |
| - 10 nodes                     | 105.96                 | 149.30             | ~29% faster           |
| - 100 nodes                    | 713.44                 | 1360.80            | ~48% faster           |
| - 1000 nodes                   | 7232.60                | 13452.00           | ~46% faster           |

* N/A: Not applicable or not measured.


### LeafNode
`pub  struct  LeafNode  {   key:  u8, value:  [u8;  33], }`

| **Operation**       | **Zero-Copy MPT (ns)** | **Alloy MPT (ns)** | **Improvement**       |
|----------------------|------------------------|--------------------|-----------------------|
| **Node Creation**    |                        |                    |                       |
| - New                | 4.47                   | 21.85              | ~80% faster           |
| **Basic Operations** |                        |                    |                       |
| - Extract Key        | 0.492                  | N/A*               |                       |
| **RLP Operations**   |                        |                    |                       |
| - Encode             | N/A**                  | 54.23              |                       |
| - Decode             | N/A**                  | 93.54              |                       |
| **Batch Creation**   |                        |                    |                       |
| - 10 nodes           | 39.11                  | 462.91             | ~92% faster           |
| - 100 nodes          | 114.17                 | 4258.70            | ~97% faster           |
| - 1000 nodes         | 1777.40                | 39479.00           | ~95% faster           |

* N/A: Not applicable or not measured.


## Zero-Copy Design

The implementation achieves zero-copy through:

-   Fixed-size arrays instead of vectors where possible, this would optimize cache hits

## Performance Notes

-   fixed array to get more cache hits
-   Avoids mcopy
-   maximize cache hits

