#![feature(test)]

mod life_bench {
    extern crate test;
    use std::hint::black_box;

    use life_io::life::iter::LifeIter;
    use test::Bencher;

    #[bench]
    fn bench_16_16(b: &mut Bencher) {
        let mut life = LifeIter::new((16, 16));

        b.iter(|| {
            life = life.update();
        });

        black_box(life);
    }

    #[bench]
    fn bench_256_256(b: &mut Bencher) {
        let mut life = LifeIter::new((256, 256));

        b.iter(|| {
            life = life.update();
        });

        black_box(life);
    }
}
