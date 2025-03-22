
use super::{Classification, Pattern};


const MAX_PERIOD: u32 = 40;


impl Pattern {

    fn run_to_stabilization(&mut self) -> Option<u32> {
        let hash = self.life.hash();

        for i in 0..MAX_PERIOD {
            self.life.update();
            if self.life.hash() == hash {
                return Some(i);
            }
        }
        
        None
    }

    pub fn classify(&mut self) {
        if let Some(period) = self.run_to_stabilization() {
            if period == 0 {
                self.classification = Some(Classification::StilLife);
                self.period_or_pop_or_lifespan = Some(self.life.get_pop(0) as u32);
            } else {
                self.classification = Some(Classification::Oscillator);
                self.period_or_pop_or_lifespan = Some(period);
            }
        }
    }
}

#[cfg(test)]
mod classify_tests {
    use crate::{life::from_plaintext, pattern::Classification};

    use super::*;

    #[test]
    fn test_block() {
        const BLOCK_TXT: &str = "OO\nOO";
        let mut pattern = Pattern::new_unclassified(from_plaintext(BLOCK_TXT, None, None));

        pattern.classify();
        
        assert_eq!(pattern.classification, Some(Classification::StilLife));
        assert_eq!(pattern.period_or_pop_or_lifespan, Some(4));
    }


    #[test]
    fn test_blinker() {
        const BLINKER: &str = ".O.\n.O.\n.O.";
        let mut pattern = Pattern::new_unclassified(from_plaintext(BLINKER, None, None));

        pattern.classify();
        
        assert_eq!(pattern.classification, Some(Classification::Oscillator));
        assert_eq!(pattern.period_or_pop_or_lifespan, Some(1));
    }

}
