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
        let mut life = from_plaintext(BLINKER_1, Some(*algo), None);
        assert_eq!(life_to_plaintext(&life), BLINKER_1, "algo: {algo:?}");

        life.update();
        assert_eq!(life_to_plaintext(&life), BLINKER_2, "algo: {algo:?}");
    }
}

#[test]
fn test_compare() {
    let mut life_basic = Life::new(LifeAlgoSelect::Basic, (64, 64));
    let mut life_cached = Life::new(LifeAlgoSelect::Cached, (64, 64));

    // TODO: THIS IS NOT THREAD SAFE!
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

const FACTION_1: &str = "\
!Name: Faction
.O..1.
.O..1.
.O..1.";

const FACTION_2: &str = "\
!Name: Faction
......
OOO111
......";

// Coexistence... Interesting
// without factions this makes a solid 4x4 block
const FACTION_3: &str = "\
!Name: Faction
.O..1.
.O..1.
.O..1.";


#[test]
fn test_faction() {
    for algo in FACTION_ALGOS {
        let mut life = from_plaintext(FACTION_1, Some(*algo), None);
        assert_eq!(life_to_plaintext(&life), FACTION_1, "algo: {algo:?} life: {life}");

        life.update();
        assert_eq!(life_to_plaintext(&life), FACTION_2, "algo: {algo:?} life: {life}");

        life.update();
        assert_eq!(life_to_plaintext(&life), FACTION_3, "algo: {algo:?} life: {life}");

        // life.update();
        // assert_eq!(life_to_plaintext(&life), FACTION_3, "algo: {algo:?} life: {life}");
    }
}

// #[test]
// fn test_faction_compare() {
//     let mut life_basic = Life::new(LifeAlgoSelect::Basic, (64, 64));
//     let mut life_cached = Life::new(LifeAlgoSelect::Cached, (64, 64));

//     life_basic.randomize(1234, false);
//     life_cached.randomize(1234, false);

//     for i in 0..50 {

//         life_basic.update();
//         life_cached.update();

//         assert_eq!(
//             life_basic.get_pop(0),
//             life_cached.get_pop(0),
//             "Failed at i: {i}"
//         );
//     }
// }

const STAR_WARS_SHIP_STATES: &[&str] = &[
"!Name: SW_SHIP
.O....
..O...
..O...
.O....",
"!Name: SW_SHIP
.BO...
..BO..
..BO..
.BO...",
"!Name: SW_SHIP
.CBO..
..CBO.
..CBO.
.CBO..",
"!Name: SW_SHIP
..CBO.
...CBO
...CBO
..CBO."
];

#[test]
fn test_star_wars() {
    for algo in WORKING_ALGOS {
        let mut life = from_plaintext(STAR_WARS_SHIP_STATES[0], Some(*algo), Some(LifeRule::STAR_WARS));
        assert_eq!(life_to_plaintext(&life), STAR_WARS_SHIP_STATES[0], "algo: {algo:?}");

        life.update();
        assert_eq!(life_to_plaintext(&life), STAR_WARS_SHIP_STATES[1], "algo: {algo:?}");

        life.update();
        assert_eq!(life_to_plaintext(&life), STAR_WARS_SHIP_STATES[2], "algo: {algo:?}");

        life.update();
        assert_eq!(life_to_plaintext(&life), STAR_WARS_SHIP_STATES[3], "algo: {algo:?}");

        // life.update();
        // assert_eq!(life_to_plaintext(&life), STAR_WARS_SHIP_STATES[4], "algo: {algo:?}");
    }
}
