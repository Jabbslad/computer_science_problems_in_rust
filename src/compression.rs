pub fn compress(s: String) -> String {
    let mut res: i16 = 1;
    for c in s.chars() {
        res = res << 2;
        match c {
            'A' => res = res + 0,
            'B' => res = res + 1,
            'C' => res = res + 2,
            'D' => res = res + 3,
            _ => panic!("invalid Nucleotide")
        }
    }
    format!("{:b}", res)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_empty() {
        assert_eq!("1", compress("".to_string()));
    }

    #[test]
    fn has_a() {
        assert_eq!("100", compress("A".to_string()));
    }

    #[test]
    fn has_b() {
        assert_eq!("101", compress("B".to_string()));
    }

    #[test]
    fn has_c() {
        assert_eq!("110", compress("C".to_string()));
    }

    #[test]
    fn has_d() {
        assert_eq!("111", compress("D".to_string()));
    }

    #[test]
    fn has_abcd() {
        assert_eq!("100011011", compress("ABCD".to_string()));
    }
}
