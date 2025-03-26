
/*
Final tally from https://ericlippert.com/2020/09/14/life-part-35/

Algorithm           time(ms) size  Mcells/s 
Naïve (Optimized):   4000     8      82     
Abrash (Original)     550     8     596     
Stafford              180     8    1820     
QuickLife              65    20      ?      
Gosper, sp 0 * 5000  3700    60      ?
Gosper, sp 13 * 1     820    60      ?

*/

use std::time::Instant;

use life_io::life::{Life, LifeOptions, WORKING_ALGOS};

const ACORN: &str = "\
!Name: Acorn
!Author: Charles Corderman
!A methuselah that stabilizes after 5206 generations.
!www.conwaylife.com/wiki/index.php?title=Acorn
.O
...O
OO..OOO";

fn main() {
    println!("Life performance comparison: 5000 gens of 'Acorn'");

    let acorn_life = Life::from_plaintext(ACORN, LifeOptions::default());
    println!("\n{:<16}  time(ms)", "Algorithm");
    println!("-------------------------");

    for algo in WORKING_ALGOS {
        let mut life = Life::new_ex((256, 256), LifeOptions {
            algo: *algo,
            ..Default::default()
        });
        life.paste(&acorn_life, (128, 128), None);

        let now = Instant::now();

        for _ in 0..5000 {
            life.update();
        }

        let elapsed = now.elapsed();
        assert_eq!(789, life.get_pop(0));
        println!("{:<16} {:>4}", format!("{:?}", algo), elapsed.as_millis());
    }
}

