use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum GeneBit {
    Zero = 0,
    One = 1,
    Three = 2,
}

pub struct Possibility {
    p: f32,
    v: GeneBit,
}

pub struct Children(Vec<Possibility>);

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
    pub fn value(&self) -> u32 {
        match self {
            GeneBit::Zero => 0,
            GeneBit::One => 1,
            GeneBit::Three => 3,
        }
    }

    pub fn hybridize(&self, another: &GeneBit) -> Children {
        match (&self, another) {
            (GeneBit::Zero, GeneBit::Zero) => Children(vec![Possibility {
                p: 1.0,
                v: GeneBit::Zero,
            }]),
            (GeneBit::Zero, GeneBit::One) => Children(vec![
                Possibility {
                    p: 0.5,
                    v: GeneBit::Zero,
                },
                Possibility {
                    p: 0.5,
                    v: GeneBit::One,
                },
            ]),
            (GeneBit::Zero, GeneBit::Three) => Children(vec![Possibility {
                p: 1.0,
                v: GeneBit::One,
            }]),
            (GeneBit::One, GeneBit::Zero) => Children(vec![
                Possibility {
                    p: 0.5,
                    v: GeneBit::Zero,
                },
                Possibility {
                    p: 0.5,
                    v: GeneBit::One,
                },
            ]),
            (GeneBit::One, GeneBit::One) => Children(vec![
                Possibility {
                    p: 0.25,
                    v: GeneBit::Zero,
                },
                Possibility {
                    p: 0.25,
                    v: GeneBit::Three,
                },
                Possibility {
                    p: 0.5,
                    v: GeneBit::One,
                },
            ]),
            (GeneBit::One, GeneBit::Three) => Children(vec![
                Possibility {
                    p: 0.5,
                    v: GeneBit::One,
                },
                Possibility {
                    p: 0.5,
                    v: GeneBit::Three,
                },
            ]),
            (GeneBit::Three, GeneBit::Zero) => Children(vec![Possibility {
                p: 1.0,
                v: GeneBit::One,
            }]),
            (GeneBit::Three, GeneBit::One) => Children(vec![
                Possibility {
                    p: 0.5,
                    v: GeneBit::One,
                },
                Possibility {
                    p: 0.5,
                    v: GeneBit::Three,
                },
            ]),
            (GeneBit::Three, GeneBit::Three) => Children(vec![Possibility {
                p: 1.0,
                v: GeneBit::Three,
            }]),
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
}
