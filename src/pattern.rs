use crate::life::Life;

// pub mod classify;
// pub mod identify;


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
    pub period_or_pop_or_lifespan: Option<u32>,
}

impl Pattern {
    pub fn new_unclassified(life: Life) -> Self {
        Self {
            life,
            ..Default::default()
        }
    }
}
