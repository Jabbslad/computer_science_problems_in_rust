#![allow(dead_code)]
#[derive(PartialEq, PartialOrd)]
enum Nucleotide {
    A = 0,
    C,
    G,
    T
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_compare() {
        assert_eq!(true, Nucleotide::A < Nucleotide::C);
    }
}
