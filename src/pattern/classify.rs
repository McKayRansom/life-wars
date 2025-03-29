use crate::life::{Life, LifeOptions};

use super::{Classification, Pattern, PatternMetadata};

const MAX_PERIOD: u32 = 140;

impl Life {
    fn calc_live_bounds(&self) -> ((u16, u16), (u16, u16)) {
        // shouldn't usually get too far
        let mut min_pos = self.size();
        let mut max_pos: (u16, u16) = (0, 0);

        // TODO: THIS IS STUPID BUT MY HEAD HURTS TRYING TO THINK OF A BETTER METHOD!
        for (x, y, cell) in self.iter() {
            if cell.is_alive() {
                if x < min_pos.0 {
                    min_pos.0 = x;
                }
                if y < min_pos.1 {
                    min_pos.1 = y;
                }
                if x > max_pos.0 {
                    max_pos.0 = x;
                }
                if y > max_pos.1 {
                    max_pos.1 = y;
                }
            }
        }
        // go diagonally top-left to bot-right and look in strips
        // while start_pos.0 < self.size().0 && start_pos.1 < self.size().1 {
        //     // strip left to right
        //     for x in start_pos.0..self.size().0 {

        //     }
        // }
        (min_pos, max_pos)
    }
}

#[cfg(test)]
mod life_tests {
    use crate::life::{Cell, Life};

    #[test]
    fn test_calc_top_left_life() {
        let mut life = Life::new((4, 4));
        assert_eq!(life.calc_live_bounds(), ((4, 4), (0, 0)));
        life.insert((1, 2), Cell::new(1, 0));
        assert_eq!(life.calc_live_bounds(), ((1, 2), (1, 2)));
        life.insert((2, 1), Cell::new(1, 0));
        assert_eq!(life.calc_live_bounds(), ((1, 1), (2, 2)));
    }
}

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
        let mut new_life = Life::new_ex(
            (self.life.size().0 + 2, self.life.size().1 + 2),
            LifeOptions {
                algo: crate::life::LifeAlgoSelect::Cached,
                rule: *self.life.get_rule(),
            },
        );

        new_life.paste(&self.life, (1, 1), None);

        // let starting_hash = self.life.hash();
        let initial_pop = self.life.get_pop(0);
        let mut min_form: String = self.life.to_apgcode();
        let mut min_top_left_pos: (u16, u16) = (1, 1);
        let mut min_form_period: u32 = 0;
        // let starting_form = min_form.clone();

        for period in 0..MAX_PERIOD {
            new_life.update();
            // TODO: This doesn't work well because of obvious reasons
            if new_life.get_pop(0) == 0 {
                self.metadata = PatternMetadata {
                    classification: Some(Classification::Messless),
                    period_or_pop_or_lifespan: Some(initial_pop as u32),
                    ..Default::default()
                };
                return;
            }

            // TODO: this is not completely what we need
            // - Rotations

            // calculate the top-left pos of live cells
            let (top_left_pos, _bot_right_pos) = new_life.calc_live_bounds();

            // dbg!(top_left_pos);

            let new_form: String = if top_left_pos == (0, 0) {
                new_life.to_apgcode()
            } else {
                new_life
                    .copy(
                        top_left_pos,
                        (
                            new_life.size().0 - top_left_pos.0 + 1,
                            new_life.size().1 - top_left_pos.1 + 1,
                        ),
                    )
                    .to_apgcode()
            };

            if new_form == min_form {
                new_life = Life::from_apgcode(min_form.as_str(), LifeOptions::default());
                if period == 0 {
                    self.metadata = PatternMetadata {
                        classification: Some(Classification::StilLife),
                        period_or_pop_or_lifespan: Some(new_life.get_pop(0) as u32),
                        ..Default::default()
                    }
                } else {
                    self.metadata = PatternMetadata {
                        classification: Some(if min_top_left_pos == top_left_pos {
                            Classification::Oscillator
                        } else {
                            Classification::Spaceship
                        }),
                        period_or_pop_or_lifespan: Some((period - min_form_period) + 1),
                        ..Default::default()
                    };
                }

                // OOOOFFFFFF
                self.life = Life::from_apgcode(min_form.as_str(), LifeOptions::default());
                // self.life = new_life.copy(
                //     top_left_pos.into(),
                //     (
                //         _bot_right_pos.0 - top_left_pos.0,
                //         _bot_right_pos.1 - top_left_pos.1,
                //     ),
                // );
                return;
            }

            // dbg!(&new_form);
            if match new_form.len().cmp(&min_form.len()) {
                std::cmp::Ordering::Less => true,
                std::cmp::Ordering::Equal => new_form < min_form,
                std::cmp::Ordering::Greater => false,
            } {
                min_form = new_form;
                min_top_left_pos = top_left_pos;
                min_form_period = period + 1;
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
            period_or_pop_or_lifespan: Some(2),
            ..Default::default()
        });
    }
}
