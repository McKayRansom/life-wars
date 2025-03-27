use crate::life::{Life, LifeOptions};

use super::{Classification, Pattern, PatternMetadata};

const MAX_PERIOD: u32 = 140;

impl Pattern {
    /*
     * Preconditions:
     * - The pattern is of minimum required bounding box
     *   - Top left will contain a live cell, etc..
     *   - The life is right-sized to hold the pattern for it's entire life (for Messless)
     * 
     * Issues:
     *   - need more space for Messless/Spaceships
     */
    pub fn classify(&mut self) {
        let hash = self.life.hash();
        let initial_pop = self.life.get_pop(0);
        let mut min_form: String = self.life.to_apgcode();

        for period in 0..MAX_PERIOD {
            self.life.update();
            // TODO: This doesn't work well because of obvious reasons
            if self.life.get_pop(0) == 0 {
                self.metadata = PatternMetadata {
                    classification: Some(Classification::Messless),
                    period_or_pop_or_lifespan: Some(initial_pop as u32),
                    ..Default::default()
                };
                return;
            }
            let new_form: String = self.life.to_apgcode();
            if new_form < min_form {
                min_form = new_form;
            }
            if self.life.hash() == hash {
                self.life = Life::from_apgcode(min_form.as_str(), LifeOptions::default());
                if period == 0 {
                    self.metadata = PatternMetadata {
                        classification: Some(Classification::StilLife),
                        period_or_pop_or_lifespan: Some(self.life.get_pop(0) as u32),
                        ..Default::default()
                    }
                } else {
                    self.metadata = PatternMetadata {
                        classification: Some(Classification::Oscillator),
                        period_or_pop_or_lifespan: Some(period),
                        ..Default::default()
                    };
                }
                return;
            }
        }
    }
}

#[cfg(test)]
mod classify_tests {
    use crate::{
        life::{Life, LifeOptions},
        pattern::{Classification, PatternMetadata},
    };

    use super::*;

    #[test]
    fn still_life_block() {
        const BLOCK_TXT: &str = "OO\nOO";
        let mut pattern =
            Pattern::new_unclassified(Life::from_plaintext(BLOCK_TXT, LifeOptions::default()));

        pattern.classify();

        assert_eq!(
            pattern.metadata.classification,
            Some(Classification::StilLife)
        );
        assert_eq!(pattern.metadata.period_or_pop_or_lifespan, Some(4));
    }

    #[test]
    fn oscillator_blinker() {
        const BLINKER: &str = ".O.\n.O.\n.O.";
        let mut pattern =
            Pattern::new_unclassified(Life::from_plaintext(BLINKER, LifeOptions::default()));

        pattern.classify();

        assert_eq!(pattern.metadata, PatternMetadata {
            classification: Some(Classification::Oscillator),
            period_or_pop_or_lifespan: Some(1),
            ..Default::default()
        });
    }

}
