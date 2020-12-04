use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_loop(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 1;
    let mut c = 0;
    for _i in 2..n {
        c = a + b;
        a = b;
        b = c;
    }
    c
}

fn fibonacci_tail(prev: u64, cur: u64, n: u64) -> u64 {
    if n == 0 {
        return cur;
    }
    return fibonacci_tail(cur, prev + cur, n - 1);
}
fn fib_tr(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    return fibonacci_tail(0, 1, n - 1);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}
fn criterion_benchmark_loop(c: &mut Criterion) {
    c.bench_function("fib loop 20", |b| b.iter(|| fibonacci_loop(black_box(20))));
}
fn criterion_benchmark_tail(c: &mut Criterion) {
    c.bench_function("fib tail 20", |b| b.iter(|| fib_tr(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_group!(benches_loop, criterion_benchmark_loop);
criterion_group!(benches_tail, criterion_benchmark_tail);
criterion_main!(benches, benches_loop, benches_tail);
