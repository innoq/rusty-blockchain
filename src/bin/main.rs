extern crate rustyblockchain;
extern crate histogram;

use rustyblockchain::*;
use std::time::SystemTime;
use histogram::Histogram;

fn main() {
    let prefix = get_prefix();
    let previous_block = Block::genesis();

    measure("test_mine_single_threaded_mutably", || {
        let mut block = Block::new(1524480511, Vec::new(), &previous_block);
        Block::mine_single_threaded_mutably(&mut block, &prefix);
        format!("{:?}", block.proof)
    });

    measure("test_mine_with_iterator", || {
        let block = Block::new(1524480511, Vec::new(), &previous_block);
        format!("{:?}", Block::mine_with_iterator(&block, &prefix).proof)
    });

    measure("test_mine_with_parallel_iterator_find_first", || {
        let block = Block::new(1524480511, Vec::new(), &previous_block);
        format!("{:?}", Block::mine_with_parallel_iterator_find_first(&block, &prefix).proof)
    });

    measure("test_mine_with_parallel_iterator_find_any", || {
        let block = Block::new(1524480511, Vec::new(), &previous_block);
        format!("{:?}", Block::mine_with_parallel_iterator_find_any(&block, &prefix).proof)
    });

    measure("test_mine_with_channels", move || {
        let block = Block::new(1524480511, Vec::new(), &previous_block);
        format!("{:?}", Block::mine_with_channels(&block, &prefix).proof)
    });
}

fn get_prefix() -> String {
    std::env::var("HASH_PREFIX").unwrap_or(String::from("0000"))
}

fn measure<F>(label: &str, closure: F) where F: Fn() -> String {
    let iters = 10;
    let mut histogram = Histogram::new();
    println!("{}:", label);
    for _ in 0..iters {
        let start = SystemTime::now();
        let mut s = closure();
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        let millis: u64 = duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000000;
        s.clear();
        println!("{} ms", millis);
        histogram.increment(millis).unwrap();
    }
    let mean = histogram.mean().unwrap();
    let median = histogram.percentile(50 as f64).unwrap();
    let max = histogram.maximum().unwrap();
    let min = histogram.minimum().unwrap();
    let std_dev = histogram.stddev().unwrap();
    println!("mean:\t{} ms/iter", mean);
    println!("median:\t{} ms/iter", median);
    println!("min:\t{} ms/iter", min);
    println!("max:\t{} ms/iter", max);
    println!("stddev:\t{}\n", std_dev);
}
