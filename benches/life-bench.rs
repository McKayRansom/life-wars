#![feature(test)]

mod life_bench {
    extern crate test;

    use life_io::life::{Life, LifeAlgoSelect};
    use test::Bencher;

    const BENCH_SEED: u64 = 1234;

    #[bench]
    fn bench_life_basic(b: &mut Bencher) {
        let mut life = Life::new(life_io::life::LifeAlgoSelect::Basic, (256, 256));
        life.randomize(BENCH_SEED, false);

        b.iter(|| {
            life.update();
        });
    }

    #[bench]
    // #[ignore = "VERY SLOW"]
    fn bench_life_sparse(b: &mut Bencher) {
        let mut life = Life::new(life_io::life::LifeAlgoSelect::Sprase, (256, 256));
        life.randomize(BENCH_SEED, false);

        b.iter(|| {
            life.update();
        });
    }

    #[bench]
    fn bench_life_cached(b: &mut Bencher) {
        let mut life = Life::new(LifeAlgoSelect::Cached, (256, 256));
        life.randomize(BENCH_SEED, false);

        b.iter(|| {
            life.update();
        });
    }
}
