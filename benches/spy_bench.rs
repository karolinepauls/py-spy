#[path = "../tests/integration_test.rs"]
mod integration_test;

use integration_test::TestRunner;
use py_spy::Config;

use criterion::{criterion_group, criterion_main, Criterion};

fn subprocess_benchmark(c: &mut Criterion) {
    let mut runner = TestRunner::new(Config::default(), "./tests/scripts/deep_stack.py");
    std::thread::sleep(std::time::Duration::from_millis(500));

    c.bench_function("get_stack_traces - deep stack", |b| {
        b.iter(|| runner.spy.get_stack_traces().unwrap())
    });
}

criterion_group!(benches, subprocess_benchmark);
criterion_main!(benches);
