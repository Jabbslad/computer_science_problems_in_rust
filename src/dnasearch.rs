#![allow(dead_code)]
#[derive(PartialEq, PartialOrd, Debug)]
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
        assert_eq!(Nucleotide::A, Nucleotide::A);
        assert_ne!(Nucleotide::A, Nucleotide::T);
    }
}
