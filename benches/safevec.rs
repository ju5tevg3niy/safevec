use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use safevec::SafeVec;

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }
}

fn gen_safevec_u32(size: usize) -> SafeVec<u32> {
    let mut result: SafeVec<u32> = SafeVec::new();
    for x in 0..size {
        result.push(x as u32);
    }
    result
}

fn gen_safevec_vec3(size: usize) -> SafeVec<Vec3> {
    let mut result: SafeVec<Vec3> = SafeVec::new();
    for i in 0..size {
        result.push(Vec3 {
            x: i as f32 * 2.0,
            y: i as f32 * 3.0,
            z: i as f32 * 5.0,
        });
    }
    result
}

fn iteration(c: &mut Criterion) {
    let sizes = [10_000, 20_000, 50_000];
    {
        let mut group = c.benchmark_group("u32");

        for size in sizes {
            group.throughput(Throughput::Elements(size as u64));

            group.bench_with_input(BenchmarkId::new("iter", size), &size, |b, &input| {
                let sv = gen_safevec_u32(input);

                b.iter(|| sv.iter().sum::<u32>());
            });

            group.bench_with_input(BenchmarkId::new("iter_mut", size), &size, |b, &input| {
                let mut sv = gen_safevec_u32(input);

                b.iter(|| {
                    sv.iter_mut()
                        .map(|x| {
                            *x += 42;
                            *x
                        })
                        .sum::<u32>()
                });
            });
        }

        group.finish();
    }
    {
        let mut group = c.benchmark_group("vec3");

        for size in sizes {
            group.throughput(Throughput::Elements(size as u64));

            group.bench_with_input(BenchmarkId::new("iter", size), &size, |b, &input| {
                let sv = gen_safevec_vec3(input);

                b.iter(|| sv.iter().map(|v| v.len()).sum::<f32>());
            });

            group.bench_with_input(BenchmarkId::new("iter_mut", size), &size, |b, &input| {
                let mut sv = gen_safevec_vec3(input);

                b.iter(|| {
                    sv.iter_mut()
                        .map(|v| {
                            v.normalize();
                            v.len()
                        })
                        .sum::<f32>()
                });
            });
        }

        group.finish();
    }
}

criterion_group!(benches, iteration);
criterion_main!(benches);
