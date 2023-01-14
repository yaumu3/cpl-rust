use cargo_snippet::snippet;

#[snippet("rolling_hash")]
/// rolling hash with fixed mod (1 << 61 - 1)
pub struct RollingHash {
    hash_acc: Vec<u64>,
    base_pow: Vec<u64>,
}

#[snippet("rolling_hash")]
impl RollingHash {
    const MOD: u64 = (1 << 61) - 1;
    const MASK30: u64 = (1 << 30) - 1;
    const MASK31: u64 = (1 << 31) - 1;
    const MASK61: u64 = (1 << 61) - 1;

    fn mul(a: u64, b: u64) -> u64 {
        let (au, ad) = (a >> 31, a & Self::MASK31);
        let (bu, bd) = (b >> 31, b & Self::MASK31);
        let mid = ad * bu + au * bd;
        au * bu * 2 + (mid >> 30) + ((mid & Self::MASK30) << 31) + ad * bd
    }

    fn modulo(x: u64) -> u64 {
        let mut res = (x >> 61) + (x & Self::MASK61);
        if res >= Self::MOD {
            res -= Self::MOD;
        }
        res
    }

    pub fn new(target: &[u8], base: u64) -> Self {
        let n = target.len();

        let mut hash_acc = vec![0; n + 1];
        let mut base_pow = vec![0; n + 1];
        base_pow[0] = 1;

        for i in 0..n {
            hash_acc[i + 1] = Self::modulo(Self::mul(hash_acc[i], base) + target[i] as u64);
            base_pow[i + 1] = Self::modulo(Self::mul(base_pow[i], base));
        }
        Self { hash_acc, base_pow }
    }

    /// Get hash within range [`left`, `right`)
    pub fn query(&self, left: Option<usize>, right: Option<usize>) -> Option<u64> {
        let n = self.hash_acc.len() - 1;
        let l = left.unwrap_or(0);
        let r = right.unwrap_or(self.hash_acc.len() - 1);
        if l > r || r > n {
            return None;
        }
        Some(Self::modulo(
            self.hash_acc[r] + 4 * Self::MOD - Self::mul(self.hash_acc[l], self.base_pow[r - l]),
        ))
    }

    /// find all start indices that match `other`
    pub fn find_all(&self, pattern: &Self) -> Option<Vec<usize>> {
        let n = self.hash_acc.len() - 1;
        let m = pattern.hash_acc.len() - 1;
        if n < m {
            return None;
        }
        let pattern = pattern.query(None, None).unwrap();
        Some(
            (0..=n - m)
                .filter(|&i| self.query(Some(i), Some(i + m)).unwrap() == pattern)
                .collect::<Vec<_>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        assert_eq!(RollingHash::mul(0, 5), 0);
        assert_eq!(RollingHash::mul(5, 0), 0);
        assert_eq!(
            RollingHash::mul(12_345_678_901, 10_987_654_321),
            1_911_157_587_856_932_063
        );
    }

    #[test]
    fn test_modulo() {
        assert_eq!(RollingHash::modulo(RollingHash::MOD * 2), 0);
        assert_eq!(
            RollingHash::modulo(RollingHash::MOD * 2 - 1),
            RollingHash::MOD - 1
        );
    }

    #[test]
    fn test_find_all_matched() {
        let base = 3;

        let txt = "ABABBABABABBABA";
        let ptn = "ABA";
        let txt_hash = RollingHash::new(txt.as_bytes(), base);
        let ptn_hash = RollingHash::new(ptn.as_bytes(), base);

        assert_eq!(txt_hash.find_all(&ptn_hash), Some(vec![0, 5, 7, 12]));
    }

    #[test]
    fn test_find_all_error() {
        let base = 3;

        let txt = "ABA";
        let ptn = "ABABBABABABBABA";
        let txt_hash = RollingHash::new(txt.as_bytes(), base);
        let ptn_hash = RollingHash::new(ptn.as_bytes(), base);

        assert_eq!(txt_hash.find_all(&ptn_hash), None);
    }
}
