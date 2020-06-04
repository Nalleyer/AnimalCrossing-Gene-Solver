use crate::gene::{Gene};

lazy_static! {
    static ref RED_SEED_4: Gene = Gene::from_human_string("2001");
    static ref WHITE_SEED_4: Gene = Gene::from_human_string("0010");
    static ref YELLOW_SEED_4: Gene = Gene::from_human_string("0200");
}

mod test {
    use super::*;

    #[test]
    fn test_white_seed() {
        use crate::gene::Possibilities;
        assert_eq!(
            WHITE_SEED_4.hybridize(&WHITE_SEED_4),
            Possibilities::human_new(&[(0.25, "0000"), (0.25, "0020"), (0.5, "0010"),])
        )
    }
}
