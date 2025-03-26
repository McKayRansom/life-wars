use crate::life::Life;

pub mod classify;
pub mod identify;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Classification {
    StilLife,     // population
    Oscillator,   // Period
    Spaceship,    // Period
    LinearGrowth, // Period
    Methuselah,   // lifespan
    Messless,     // lifespan (A.K.A. diehard)
    Megasized,    // ??? (Large final population)

    Explosive,
    Linear,
    Quadratic,
    Replicator,
    Pathological,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct PatternMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub classification: Option<Classification>,
    pub period_or_pop_or_lifespan: Option<u32>,
}

#[derive(Default, Clone)]
pub struct Pattern {
    pub life: Life,
    pub metadata: PatternMetadata,
}

impl Pattern {
    pub fn new_unclassified(life: Life) -> Self {
        Self {
            life,
            ..Default::default()
        }
    }
}
