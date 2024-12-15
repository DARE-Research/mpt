use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use mpt::{extension::ExtensionNode, rlp::RlpNodes};

pub fn extension_node_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("extension_node");
    
    // Setup test data
    let test_data = [1u8; 33];
    let test_key = 0xAB;
    let test_node = RlpNodes::from_raw(&test_data).unwrap();

    // Basic node creation
    group.bench_function("new", |b| {
        b.iter(|| {
            ExtensionNode::new(
                black_box(test_key),
                black_box(test_node.clone())
            )
        })
    });

    // Nibble extraction
    let node = ExtensionNode::new(test_key, test_node.clone());
    
    group.bench_function("extract_upper_nibble", |b| {
        b.iter(|| {
            black_box(ExtensionNode::extract_key(test_key, true))
        })
    });

    group.bench_function("extract_lower_nibble", |b| {
        b.iter(|| {
            black_box(ExtensionNode::extract_key(test_key, false))
        })
    });

    group.bench_function("get_upper_nibble", |b| {
        b.iter(|| {
            black_box(node.get_upper_nibble())
        })
    });

    group.bench_function("get_lower_nibble", |b| {
        b.iter(|| {
            black_box(node.get_lower_nibble())
        })
    });

    // Hash operations
    group.bench_function("has_hashed_child", |b| {
        b.iter(|| {
            black_box(node.has_hashed_child())
        })
    });

    group.bench_function("child_hash", |b| {
        b.iter(|| {
            black_box(node.child_hash())
        })
    });

    // Test with different key patterns
    let test_keys = vec![0x00, 0x0F, 0xF0, 0xFF];
    for key in test_keys {
        group.bench_with_input(
            BenchmarkId::new("new_with_key", format!("0x{:02X}", key)),
            &key,
            |b, &key| {
                b.iter(|| {
                    ExtensionNode::new(black_box(key), black_box(test_node.clone()))
                })
            }
        );
    }

    // Batch node creation
    let batch_sizes = vec![10, 100, 1000];
    for size in batch_sizes {
        group.bench_with_input(
            BenchmarkId::new("batch_create", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    (0..size).map(|i| {
                        ExtensionNode::new(
                            black_box(i as u8),
                            black_box(test_node.clone())
                        )
                    }).collect::<Vec<_>>()
                })
            }
        );
    }

    group.finish();
}

criterion_group!(benches, extension_node_benchmarks);
criterion_main!(benches);