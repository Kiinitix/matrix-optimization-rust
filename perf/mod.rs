use perf_event::events::{CacheId, HardwareEventType};
use perf_event::{Builder, Counter};
use std::time::Instant;

pub struct BenchmarkResult {
    pub duration: f64,
    pub l1_misses: u64,
    pub l2_misses: u64,
}

pub fn measure<F>(mut f: F) -> BenchmarkResult
where
    F: FnMut(),
{
    let mut l1_counter = Builder::new()
        .group_leader()
        .kind(HardwareEventType::CacheMisses)
        .build()
        .unwrap();

    let mut l2_counter = Builder::new()
        .kind(HardwareEventType::CacheReferences)
        .build()
        .unwrap();

    l1_counter.enable().unwrap();
    l2_counter.enable().unwrap();

    let start = Instant::now();
    f();
    let duration = start.elapsed().as_secs_f64();

    l1_counter.disable().unwrap();
    l2_counter.disable().unwrap();

    BenchmarkResult {
        duration,
        l1_misses: l1_counter.read().unwrap(),
        l2_misses: l2_counter.read().unwrap(),
    }
}