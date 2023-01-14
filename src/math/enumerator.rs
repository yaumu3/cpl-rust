use cargo_snippet::snippet;

#[snippet("enumerator")]
pub struct Enumerator {
    fact: Vec<usize>,
    finv: Vec<usize>,
    n: usize,
    p: usize,
}

#[snippet("enumerator")]
impl Enumerator {
    fn init(&mut self) {
        self.fact[0] = 1;
        self.finv[0] = 1;
        if self.n == 1 {
            return;
        }
        let mut invs = vec![0_usize; self.n];
        self.fact[1] = 1;
        self.finv[1] = 1;
        invs[1] = 1;
        for i in 2..self.n {
            self.fact[i] = self.fact[i - 1] * i % self.p;
            invs[i] = self.p - invs[self.p % i] * (self.p / i) % self.p;
            self.finv[i] = self.finv[i - 1] * invs[i] % self.p;
        }
    }

    pub fn new(n_max: usize, p: usize) -> Enumerator {
        let mut enr = Enumerator {
            fact: vec![0; n_max + 1],
            finv: vec![0; n_max + 1],
            n: n_max + 1,
            p,
        };
        Enumerator::init(&mut enr);
        enr
    }

    pub fn factorial(&self, n: usize) -> usize {
        self.fact[n]
    }

    pub fn choose(&self, n: usize, k: usize) -> usize {
        let perm = self.permutate(n, k);
        if perm != 0 {
            perm * self.finv[k] % self.p
        } else {
            0
        }
    }

    pub fn permutate(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        assert!(n <= self.n && k <= self.n);
        self.fact[n] * self.finv[n - k] % self.p
    }

    pub fn choose_with_duplicates(&self, n: usize, k: usize) -> usize {
        self.choose(n + k - 1, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let e = Enumerator::new(30, 1_000_000_007);
        e.choose(31, 2);
    }

    #[test]
    fn test_factorial() {
        let e = Enumerator::new(100, 1_000_000_007);
        assert_eq!(e.factorial(0), 1);
        assert_eq!(e.factorial(6), 720);
    }

    #[test]
    fn test_choose() {
        let e = Enumerator::new(100, 1_000_000_007);
        assert_eq!(e.choose(6, 0), 1);
        assert_eq!(e.choose(6, 1), 6);
        assert_eq!(e.choose(6, 2), 15);
        assert_eq!(e.choose(6, 4), e.choose(6, 2));
        assert_eq!(e.choose(6, 7), 0);
    }

    #[test]
    fn test_permutate() {
        let e = Enumerator::new(100, 1_000_000_007);
        assert_eq!(e.permutate(7, 0), 1);
        assert_eq!(e.permutate(7, 1), 7);
        assert_eq!(e.permutate(7, 7), 5040);
        assert_eq!(e.permutate(7, 8), 0);
    }

    #[test]
    fn test_choose_with_duplicates() {
        let e = Enumerator::new(100, 1_000_000_007);
        assert_eq!(e.choose_with_duplicates(3, 0), 1);
        assert_eq!(e.choose_with_duplicates(3, 1), 3);
        assert_eq!(e.choose_with_duplicates(3, 4), 15);
    }
}
