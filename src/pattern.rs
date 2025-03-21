use crate::life::Life;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Classification {
    StilLife, // population
    Oscillator,
    Spaceship, // Period
    LinearGrowth, // Period
    Methuselah, // lifespan
    Messless, // lifespan (A.K.A. diehard)
    Megasized, // ??? (Large final population)

    Explosive,
    Linear,
    Quadratic,
    Replicator,
    Pathological,
}

#[derive(Default)]
pub struct Pattern {
    pub life: Life,
    pub name: Option<String>,
    pub description: Option<String>,
    pub classification: Option<Classification>,
    pub period: Option<u32>,
}
