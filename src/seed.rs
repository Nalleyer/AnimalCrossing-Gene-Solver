use crate::gene::{Gene, Possibilities};

lazy_static! {
    static ref RED_SEED: Gene = Gene::from_human_string("2001");
    static ref WHITE_SEED: Gene = Gene::from_human_string("0010");
    static ref YELLOW_SEED: Gene = Gene::from_human_string("0200");
}

mod test {
    use super::*;

    #[test]
    fn test_white_seed() {
        assert_eq!(
            WHITE_SEED.hybridize(&WHITE_SEED),
            Possibilities::human_new(&[
                (0.25, "0000"),
                (0.25, "0020"),
                (0.5, "0010"),
            ])
        )
    }
}