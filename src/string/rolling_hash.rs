use cargo_snippet::snippet;

#[allow(clippy::clippy::module_inception)]
#[snippet]
/// Rolling hash with fixed mod (1 << 61 - 1)
pub mod rolling_hash {
    const MOD: u64 = (1 << 61) - 1;
    const MASK30: u64 = (1 << 30) - 1;
    const MASK31: u64 = (1 << 31) - 1;
    const MASK61: u64 = (1 << 61) - 1;

    pub struct RollingHash {
        hash_acc: Vec<u64>,
        base_pow: Vec<u64>,
    }

    impl RollingHash {
        pub fn new(string: &[u8], base: u64) -> Self {
            let n = string.len();

            let mut hash_acc = vec![0; n + 1];
            let mut base_pow = vec![0; n + 1];
            base_pow[0] = 1;

            for i in 0..n {
                hash_acc[i + 1] = modulo(mul(hash_acc[i], base) + string[i] as u64);
                base_pow[i + 1] = modulo(mul(base_pow[i], base));
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
            Some(modulo(
                self.hash_acc[r] + 4 * MOD - mul(self.hash_acc[l], self.base_pow[r - l]),
            ))
        }

        /// Find all start indices that match `other`
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

    fn mul(a: u64, b: u64) -> u64 {
        let (au, ad) = (a >> 31, a & MASK31);
        let (bu, bd) = (b >> 31, b & MASK31);
        let mid = ad * bu + au * bd;
        au * bu * 2 + (mid >> 30) + ((mid & MASK30) << 31) + ad * bd
    }

    fn modulo(x: u64) -> u64 {
        let mut res = (x >> 61) + (x & MASK61);
        if res >= MOD {
            res -= MOD;
        }
        res
    }
}

#[test]
fn test_find_all_matched() {
    let base = 3;

    let txt = "ABABBABABABBABA";
    let ptn = "ABA";
    let txt_hash = rolling_hash::RollingHash::new(txt.as_bytes(), base);
    let ptn_hash = rolling_hash::RollingHash::new(ptn.as_bytes(), base);

    assert_eq!(txt_hash.find_all(&ptn_hash), Some(vec![0, 5, 7, 12]));
}

#[test]
fn test_find_all_error() {
    let base = 3;

    let txt = "ABA";
    let ptn = "ABABBABABABBABA";
    let txt_hash = rolling_hash::RollingHash::new(txt.as_bytes(), base);
    let ptn_hash = rolling_hash::RollingHash::new(ptn.as_bytes(), base);

    assert_eq!(txt_hash.find_all(&ptn_hash), None);
}
