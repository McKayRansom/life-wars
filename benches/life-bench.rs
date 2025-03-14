#![feature(test)]

mod life_bench {
    extern crate test;

    use life_io::life::{basic::LifeBasic, cached::LifeCached, sparse::LifeSparse, LifeAlgo};
    use test::Bencher;

    const BENCH_SEED: u64 = 1234;

    #[bench]
    fn bench_life_iter(b: &mut Bencher) {
        let mut life = LifeBasic::new((256, 256));
        life.randomize(BENCH_SEED, false);

        b.iter(|| {
            life = life.update();
        });
    }

    #[bench]
    fn bench_life_sparse(b: &mut Bencher) {
        let mut life = LifeSparse::new((256, 256));
        life.randomize(BENCH_SEED, false);

        b.iter(|| {
            life = life.update();
        });
    }

    #[bench]
    fn bench_life_cached(b: &mut Bencher) {
        let mut life = LifeCached::new((256, 256));
        life.randomize(BENCH_SEED, false);

        b.iter(|| {
            life.update();
        });
    }
}
