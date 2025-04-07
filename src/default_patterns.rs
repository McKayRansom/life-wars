const FIGHTER: &str = "\
.OO.
OBBO
BCCB
C..C";

const BOMBER: &str = "\
.OO.
OOO.
.OBO";

const FRIGATE: &str = "\
...OO.......
...OO.......
..OBBOO.....
...OO.OO....
..C..OO.O...
..BCB.OOB...
...OOOO.CO..
....C.OO.B..
..C.BOO..CO.
...OO.OOB.B.
....OOO.O.C.
..C...OO.O..
.OOB.OO.OO..
BCO.O.OO.OOO
OO.O.OO.OO.O
.OO...OO.OBO
..OC..O.OO.O
.OBO..OO.O.O
..C.BOO.OO.B
..C.C.OO.O.C";

const DREADNAUGHT: &str = "\
..........
....OO....
....OO....
..OOBBOO..
.OO.OO.OO.
..OO..OO..
OOO.OO.OOO
B.OO..OO.B
OOO.OO.OOO";

const BATTLESTATION: &str = "\
.............
......OO.....
......OO.....
....OOBBOO...
....O.OO.O...
..OOBO..OBOO.
..OOBO..OBOO.
....O.OO.O...
....OOBBOO...
......OO.....
......OO.....
.............";

pub const EASY_PATTERNS: &[&str] = &[
    FIGHTER, BOMBER,
    // FRIGATE,
    // DREADNAUGHT,
    // BATTLESTATION,
];

pub const MEDIUM_PATTERNS: &[&str] = &[
    FIGHTER, BOMBER,
    // FRIGATE,
    // DREADNAUGHT,
    // BATTLESTATION,
];

pub const HARD_PATTERNS: &[&str] = &[
    FIGHTER,
    BOMBER,
    FRIGATE,
    DREADNAUGHT,
    BATTLESTATION,
    // future
];

pub const PLAYER_PATTERNS: &[&str] = &[
    FIGHTER,
    BOMBER,
    FRIGATE,
    DREADNAUGHT,
    BATTLESTATION,
    // future
];

pub const PATTERN_TIMES: &[usize] = &[
    10,
    15,
    50,
    150,
    250,
];

pub const PATTERN_MAX_COUNT: usize = PLAYER_PATTERNS.len();
