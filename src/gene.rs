use std::fmt;
use std::ops::Mul;

#[derive(Debug, PartialEq, Eq)]
pub enum GeneBit {
    Zero = 0,
    One = 1,
    Three = 2,
}

impl fmt::Display for GeneBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.show_value())
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

    pub fn to_char(&self) -> char {
        match self {
            GeneBit::Zero => '0',
            GeneBit::One => '1',
            GeneBit::Three => '2',
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            GeneBit::Zero => 0,
            GeneBit::One => 1,
            GeneBit::Three => 3,
        }
    }

    pub fn show_value(&self) -> u32 {
        match self {
            GeneBit::Zero => 0,
            GeneBit::One => 1,
            GeneBit::Three => 2,
        }
    }

    pub fn hybridize(&self, another: &GeneBit) -> Possibilities {
        match (self.show_value(), another.show_value()) {
            (0, 0) => Possibilities::new(&[(1.0, "0")]),
            (0, 1) => Possibilities::new(&[(0.5, "0"), (0.5, "1")]),
            (0, 2) => Possibilities::new(&[(1.0, "1")]),
            (1, 0) => Possibilities::new(&[(0.5, "0"), (0.5, "1")]),
            (1, 1) => Possibilities::new(&[(0.25, "0"), (0.25, "2"), (0.5, "1")]),
            (1, 2) => Possibilities::new(&[(0.5, "1"), (0.5, "2")]),
            (2, 0) => Possibilities::new(&[(1.0, "1")]),
            (2, 1) => Possibilities::new(&[(0.5, "1"), (0.5, "2")]),
            (2, 2) => Possibilities::new(&[(1.0, "2")]),
            _ => panic!("impossible"),
        }
    }
}

impl Mul for GeneBit {
    type Output = Possibilities;
    fn mul(self, rhs: Self) -> Self::Output {
        self.hybridize(&rhs)
    }
}

// 低位在前，最低位是[0]
#[derive(Debug, PartialEq, Eq)]
pub struct Gene(Vec<GeneBit>);

impl Gene {
    pub fn value(&self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .fold(0, |acc, (index, bit)| acc | (bit.value() << (index * 2)))
    }

    pub fn from_string(str: &str) -> Self {
        Gene(str.chars().map(GeneBit::from_char).collect())
    }

    pub fn from_human_string(str: &str) -> Self {
        Gene(str.chars().rev().map(GeneBit::from_char).collect())
    }

    pub fn to_string(&self) -> String {
        self.0.iter().map(|bit| bit.to_char()).collect()
    }

    pub fn to_human_string(&self) -> String {
        self.0.iter().rev().map(|bit| bit.to_char()).collect()
    }

    pub fn hybridize(&self, another: &Self) -> Possibilities {
        self.0
            .iter()
            .zip(another.0.iter())
            .map(|(a, b)| a.hybridize(&b))
            .fold(Possibilities::zero(), |p1, p2| p1.merge(&p2))
    }

    pub fn build_all_gene_list(n: usize) -> Vec<Gene> {
        let gene_str_list: Vec<String> = Vec::new();
        build_gene_str_list(gene_str_list, n).iter().map(|s| Gene::from_string(s)).collect()
    }
}

fn build_gene_str_list(gene_str_list: Vec<String>, n: usize) -> Vec<String> {
    if n == 0 {
        vec![String::from("0"), String::from("1"), String::from("2")]
    } else {
        let gene_str_list_2 = build_gene_str_list(gene_str_list, n - 1);
        let result = ['0', '1', '2'].iter().flat_map(|c| {
            let mut pushed = Vec::new();
            for s in &gene_str_list_2 {
                let mut new_s = s.clone();
                new_s.push(*c);
                pushed.push(new_s);
            }
            pushed
        }).collect();
        result
    }
}

impl Mul for Gene {
    type Output = Possibilities;
    fn mul(self, rhs: Self) -> Self::Output {
        self.hybridize(&rhs)
    }
}

#[derive(Debug, PartialEq)]
pub struct Possibility {
    p: f32,
    v: Gene,
}

impl Possibility {
    pub fn new(p: f32, g: &str) -> Self {
        Possibility {
            p,
            v: Gene::from_string(g),
        }
    }

    pub fn human_new(p: f32, g: &str) -> Self {
        Possibility {
            p,
            v: Gene::from_human_string(g),
        }
    }

    pub fn from_tuple((p, g): (f32, &str)) -> Self {
        Possibility::new(p, g)
    }

    pub fn from_human_tuple((p, g): (f32, &str)) -> Self {
        Possibility::human_new(p, g)
    }

    pub fn merge(&self, another: &Self) -> Self {
        let merged_string = self.v.to_string() + &another.v.to_string();
        Possibility::from_tuple((self.p * another.p, &merged_string))
    }

    pub fn zero() -> Self {
        Self::from_tuple((1.0, ""))
    }
}

impl Mul for Possibility {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.merge(&rhs)
    }
}

#[derive(Debug, PartialEq)]
pub struct Possibilities {
    ps: Vec<Possibility>,
}

impl Possibilities {
    pub fn new(ps: &[(f32, &str)]) -> Self {
        Possibilities {
            ps: ps.iter().map(|p| Possibility::from_tuple(*p)).collect(),
        }
    }

    pub fn human_new(ps: &[(f32, &str)]) -> Self {
        Possibilities {
            ps: ps
                .iter()
                .map(|p| Possibility::from_human_tuple(*p))
                .collect(),
        }
    }

    pub fn merge(&self, another: &Possibilities) -> Possibilities {
        let mut new_ps = Vec::new();
        for my_p in &self.ps {
            for another_p in &another.ps {
                new_ps.push(my_p.merge(&another_p))
            }
        }
        Possibilities { ps: new_ps }
    }

    pub fn zero() -> Self {
        Self {
            ps: vec![Possibility::zero()],
        }
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
                GeneBit::Three,
                GeneBit::Zero,
                GeneBit::Zero
            ])
        )
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            GeneBit::from_number(0) * GeneBit::from_number(2),
            Possibilities::new(&[(1.0, "1")])
        );
    }

    #[test]
    fn test_merge_possibility() {
        let l = Possibility::from_tuple((0.5, "00"));
        let r = Possibility::from_tuple((0.5, "01"));
        let lr = l * r;
        assert_eq!(lr, Possibility::from_tuple((0.25, "0001")));
    }

    #[test]
    fn test_merge_possibilities() {
        let l = Possibilities::new(&[(0.5, "00"), (0.5, "01")]);
        let r = Possibilities::new(&[(0.5, "0"), (0.5, "1")]);
        let lr = l.merge(&r);
        assert_eq!(
            lr,
            Possibilities::new(&[(0.25, "000"), (0.25, "001"), (0.25, "010"), (0.25, "011"),])
        );
    }
}
