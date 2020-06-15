use rand::{thread_rng, Rng};

pub fn random_key(n: u32) -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(0, n)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_random_key_between_range() {
        for i in 1..=1000 {
            let r = random_key(i);
            assert_eq!(true, r <= i);
        }
    }
}
