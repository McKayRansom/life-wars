#![feature(test)]

mod life_bench {
    extern crate test;
    use std::hint::black_box;

    use life_io::life::{basic::LifeBasic, sparse::LifeSparse, Life};
    use test::Bencher;

    const BENCH_SEED: u64 = 1234;

    #[bench]
    fn bench_life_iter(b: &mut Bencher) {
        let mut life = LifeBasic::new((256, 256));
        life.randomize(BENCH_SEED);

        b.iter(|| {
            life = life.update();
        });

        black_box(life);
    }

    #[bench]
    fn bench_life_sparse(b: &mut Bencher) {
        let mut life = LifeSparse::new((256, 256));
        life.randomize(BENCH_SEED);

        b.iter(|| {
            life = life.update();
        });

        black_box(life);
    }
}
