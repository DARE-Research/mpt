use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use mpt::{branch::BranchNode, rlp::RlpNodes};



pub fn branch_node_benchmarks(c: &mut Criterion) {
    // Setup common test data
    let mut group = c.benchmark_group("branch_node");
    let sample_data = vec![1u8; 33];
    
    // Benchmark node creation
    group.bench_function("new_empty", |b| {
        b.iter(|| BranchNode::default())
    });

    // Single child operations
    let mut node = BranchNode::default();
    let test_node = RlpNodes::from_raw(&sample_data).unwrap();
    
    group.bench_function("set_single_child", |b| {
        b.iter(|| {
            node.set_child(vec![test_node.clone()], black_box(5))
        })
    });

    group.bench_function("get_single_child", |b| {
        b.iter(|| {
            black_box(node.get_child(black_box(5)))
        })
    });

    // Batch operations
    let indices: Vec<_> = (0..8).collect();
    let updates: Vec<_> = indices.iter()
        .map(|&i| (i, vec![test_node.clone()]))
        .collect();

    group.bench_function("batch_set_8_children", |b| {
        b.iter(|| {
            node.batch_set_children(updates.clone())
        })
    });

    group.bench_function("batch_get_8_children", |b| {
        b.iter(|| {
            black_box(node.batch_get_children(&indices))
        })
    });

    // Compare single vs batch for same number of operations
    let single_indices: Vec<_> = (0..8).collect();
    group.bench_function("eight_individual_sets", |b| {
        b.iter(|| {
            for i in single_indices.iter() {
                node.set_child(vec![test_node.clone()], *i);
            }
        })
    });

    // Benchmark with different batch sizes
    let batch_sizes = vec![2, 4, 8, 16];
    for size in batch_sizes {
        let updates: Vec<_> = (0..size)
            .map(|i| (i, vec![test_node.clone()]))
            .collect();
            
        group.bench_with_input(
            BenchmarkId::new("batch_set_variable", size), 
            &updates,
            |b, updates| {
                b.iter(|| {
                    node.batch_set_children(updates.clone())
                })
            }
        );
    }

    // Payload length calculation
    group.bench_function("rlp_payload_length", |b| {
        b.iter(|| {
            black_box(node.rlp_payload_length())
        })
    });

    group.finish();
}

criterion_group!(benches, branch_node_benchmarks);
criterion_main!(benches);

// Helper function to create test data
fn create_test_node(size: usize) -> BranchNode {
    let mut node = BranchNode::default();
    let test_data = vec![1u8; 33];
    let test_node = RlpNodes::from_raw(&test_data).unwrap();
    
    for i in 0..size {
        node.set_child(vec![test_node.clone()], i);
    }
    node
}