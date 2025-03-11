
pub mod life;



#[cfg(test)]
pub mod life_test {

    use crate::life::{iter::LifeIter, Life};
    

    #[test]
    fn life_test_basic() {
        let life: LifeIter = " * 
 * 
 * "
        .into();

        assert_eq!(life.get((0, 0)).unwrap(), &0);
        assert_eq!(life.get((1, 0)).unwrap(), &1);
        assert_eq!(life.get((0, 1)).unwrap(), &0);

        assert_eq!(life.neighbors((0, 0)), 2);
        assert_eq!(life.neighbors((1, 0)), 1);
        assert_eq!(life.neighbors((0, 1)), 3);

        assert_eq!(life.update(), "   \n***\n   ".into());

        // as
    }
}
