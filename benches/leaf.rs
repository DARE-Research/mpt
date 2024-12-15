use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use mpt::leaf::LeafNode;

pub fn leaf_node_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("leaf_node");
    
    // Setup test data
    let test_value = [42u8; 33];
    let test_key = 0xAB;

    // Basic node creation
    group.bench_function("new", |b| {
        b.iter(|| {
            LeafNode::new(
                black_box(test_key),
                black_box(&test_value)
            )
        })
    });

    // Key extraction
    group.bench_function("extract_key", |b| {
        b.iter(|| {
            black_box(LeafNode::extract_key(black_box(test_key)))
        })
    });

    // Creation with different key values
    let test_keys = vec![0x00, 0x0F, 0xF0, 0xFF];
    for key in test_keys {
        group.bench_with_input(
            BenchmarkId::new("new_with_key", format!("0x{:02X}", key)),
            &key,
            |b, &key| {
                b.iter(|| {
                    LeafNode::new(black_box(key), black_box(&test_value))
                })
            }
        );
    }

    // Creation with different value patterns
    let value_patterns = vec![
        [0x00; 33],    // All zeros
        [0xFF; 33],    // All ones
        [0xAA; 33],    // Alternating bits
        {              // Sequential values
            let mut arr = [0u8; 33];
            for i in 0..33 {
                arr[i] = i as u8;
            }
            arr
        }
    ];

    for (i, value) in value_patterns.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("new_with_value_pattern", i),
            value,
            |b, value| {
                b.iter(|| {
                    LeafNode::new(black_box(test_key), black_box(value))
                })
            }
        );
    }

    // Batch creation
    let batch_sizes = vec![10, 100, 1000];
    for size in batch_sizes {
        group.bench_with_input(
            BenchmarkId::new("batch_create", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    (0..size).map(|i| {
                        LeafNode::new(
                            black_box(i as u8),
                            black_box(&test_value)
                        )
                    }).collect::<Vec<_>>()
                })
            }
        );
    }

    group.finish();
}

criterion_group!(benches, leaf_node_benchmarks);
criterion_main!(benches);