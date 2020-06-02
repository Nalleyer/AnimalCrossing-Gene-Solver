use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum GeneBit {
    Zero = 0,
    One = 1,
    Three = 2,
}

impl fmt::Display for GeneBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let show_value = match *self {
            GeneBit::Zero => 0,
            GeneBit::One => 1,
            GeneBit::Three => 2,
        };
        write!(f, "{}", show_value)
    }
}

impl GeneBit {
    pub fn from_number(value: u32) -> Self {
        match value {
            0 => GeneBit::Zero,
            1 => GeneBit::One,
            _ => GeneBit::Three,
        }
    }

    pub fn from_char(ch: char) -> Self {
        Self::from_number(ch.to_digit(10).unwrap())
    }

    pub fn value(&self) -> u32 {
        match self {
            GeneBit::Zero => 0,
            GeneBit::One => 1,
            GeneBit::Three => 3,
        }
    }

    pub fn hybridize(&self, another: &GeneBit) -> Possibilities {
        match (self.value(), another.value()) {
            (0, 0) => Possibilities::new(&[(1.0, 0)]),
            (0, 1) => Possibilities::new(&[(0.5, 0), (0.5, 1)]),
            (0, 2) => Possibilities::new(&[(1.0, 1)]),
            (1, 0) => Possibilities::new(&[(0.5, 0), (0.5, 1)]),
            (1, 1) => Possibilities::new(&[(0.25, 0), (0.25, 2), (0.5, 1)]),
            (1, 2) => Possibilities::new(&[(0.5, 1), (0.5, 2)]),
            (2, 0) => Possibilities::new(&[(1.0, 1)]),
            (2, 1) => Possibilities::new(&[(0.5, 1), (0.5, 2)]),
            (2, 2) => Possibilities::new(&[(1.0, 2)]),
            _ => panic!("impossible"),
        }
    }
}

pub struct Possibility {
    p: f32,
    v: GeneBit,
}

impl Possibility {
    pub fn new(p: f32, v: u32) -> Self {
        Possibility {
            p,
            v: GeneBit::from_number(v),
        }
    }

    pub fn from_tuple((p, v): (f32, u32)) -> Self {
        Possibility::new(p, v)
    }
}

pub struct Possibilities {
    ps: Vec<Possibility>,
}

impl Possibilities {
    pub fn new(ps: &[(f32, u32)]) -> Self {
        Possibilities {
            ps: ps.iter().map(|p| Possibility::from_tuple(*p)).collect(),
        }
    }
}

// 低位在前，最低位是[0]
#[derive(Debug, PartialEq, Eq)]
pub struct Gene(Vec<GeneBit>);

impl Gene {
    pub fn value(&self) -> u32 {
        let mut result = 0;
        self.0
            .iter()
            .enumerate()
            .fold(0, |acc, (index, bit)| acc | (bit.value() << (index * 2)))
    }

    pub fn from_string(str: &str) -> Self {
        Gene(str.chars().rev().map(|c| GeneBit::from_char(c)).collect())
    }
}

mod test {
    use super::*;

    #[test]
    fn test_gene4_value() {
        assert_eq!(
            Gene(vec![
                GeneBit::Zero,
                GeneBit::Zero,
                GeneBit::Three,
                GeneBit::Zero
            ])
            .value(),
            48u32
        );
        assert_eq!(Gene(vec![GeneBit::One]).value(), 1u32);
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            Gene::from_string("0200"),
            Gene(vec![
                GeneBit::Zero,
                GeneBit::Zero,
                GeneBit::Three,
                GeneBit::Zero
            ])
        )
    }
}
