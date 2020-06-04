#[macro_use]
extern crate lazy_static;
mod character;
mod color;
mod gene;
mod seed4;

use crate::gene::{Gene};

fn main() {
    println!("{:?}", Gene::build_all_gene_list(3));
}
