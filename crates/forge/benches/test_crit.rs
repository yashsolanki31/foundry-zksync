use criterion::{criterion_group, criterion_main, Criterion};
use foundry_test_utils::{util::check_out_forge_remote, TestCommand, TestProject};
// use foundry_zksync_compiler::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    let prj = check_out_project();
    println!("ENTER TO BENCH {:?}", prj);
   // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

   let mut group = c.benchmark_group("forge build");
   group.sample_size(10);
   group.bench_function("build_aave", |b| {
       let mut cmd: TestCommand = prj.forge_command();
       cmd.arg("build").arg("--zksync");
       b.iter(|| {
           cmd.ensure_execute_success().unwrap();
       });
   });
}

pub fn check_out_project() -> TestProject {
    check_out_forge_remote("Moonsong-Labs/aave-delivery-infrastructure")
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);