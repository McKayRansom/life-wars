use life_io::life::*;

const BLINKER_1: &str = "\
!Name: Blinker
.O.
.O.
.O.";

const BLINKER_2: &str = "\
!Name: Blinker
...
OOO
...";

#[test]
fn test_blinker() {
    for algo in WORKING_ALGOS {
        let mut life = from_plaintext(BLINKER_1, Some(*algo));
        assert_eq!(life_to_plaintext(&life), BLINKER_1, "algo: {algo:?}");

        life.update();
        assert_eq!(life_to_plaintext(&life), BLINKER_2, "algo: {algo:?}");
    }
}

#[test]
fn test_compare() {
    let mut life_basic = Life::new(LifeAlgoSelect::Basic, (64, 64));
    let mut life_cached = Life::new(LifeAlgoSelect::Cached, (64, 64));

    life_basic.randomize(1234, false);
    life_cached.randomize(1234, false);

    for i in 0..50 {

        life_basic.update();
        life_cached.update();

        assert_eq!(
            life_basic.get_pop(0),
            life_cached.get_pop(0),
            "Failed at i: {i}"
        );
    }
}

/*
 * Future Tests:
 * - Factions
 * - STAR WARS
 * - ETC...
 */
