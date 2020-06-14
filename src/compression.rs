type BitPair = (bool, bool);

pub fn compress2(s: String) -> Vec<BitPair> {
    let mut v: Vec<BitPair>  = Vec::new();
        for c in s.chars() {
        match c {
            'A' => v.push((false, false)),
            'B' => v.push((false, true)),
            'C' => v.push((true, false)),
            'D' => v.push((true, true)),
            _ => panic!("invalid Nucleotide"),
        }
    }
    v
}


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
    fn test_bit() {
        let expected = vec![(false, false), (false, true), (true, false), (true, true)];
        assert_eq!(expected, compress2("ABCD".to_owned()));
    }

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
