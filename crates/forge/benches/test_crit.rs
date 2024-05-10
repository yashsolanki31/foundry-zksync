use criterion::{black_box, criterion_group, criterion_main, Criterion};
use foundry_test_utils::{util::setup_forge_remote, TestCommand, TestProject};
use foundry_zksync_compiler::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    let (prj, _) = built_solmate();
    println!("{:?}", prj);
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

pub fn built_solmate() -> (TestProject, TestCommand) {
    setup_forge_remote("Moonsong-Labs/aave-delivery-infrastructure")
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);