use cargo_snippet::snippet;

#[snippet("linear_sieve")]
#[allow(dead_code)]
/// Sieve of eratosthenes having linear time complexity
/// for primarity test and prime factorization.
///
/// https://cp-algorithms.com/algebra/prime-sieve-linear.html
///
/// * `primes`: Vector of found primes.
/// * `lpf`: `lpf[i]` is the least prime factor of `i`.
/// e.g.) `lpf[7] == 7`, `lpf[20] == 5`, `lpf[30] == 2`.
pub struct LinearSieve {
    primes: Vec<usize>,
    lpf: Vec<usize>,
}

#[snippet("linear_sieve")]
impl LinearSieve {
    /// Constructs a new LinearSieve struct.
    ///
    /// # Arguments
    ///
    /// * `n_max`: Max number to check primarity (inclusive).
    pub fn new(n_max: usize) -> Self {
        let mut primes: Vec<usize> = vec![];
        let mut lpf: Vec<usize> = vec![0; n_max + 1];
        for d in 2..=n_max {
            if lpf[d] == 0 {
                lpf[d] = d;
                primes.push(d);
            }
            for &p in &primes {
                if p * d > n_max || p > lpf[d] {
                    break;
                }
                lpf[p * d] = p;
            }
        }
        Self { primes, lpf }
    }

    /// Tests if `n` is a prime number.
    pub fn is_prime(&self, n: usize) -> bool {
        n > 1 && self.lpf[n] == n
    }

    /// Returns vector of prime factors of `n` in increasing order
    /// with time-complexity `O(log n)`
    pub fn factorize(&self, n: usize) -> Vec<usize> {
        if n < 2 {
            return vec![];
        }
        let mut res = vec![];
        let mut i = n;
        while !self.is_prime(i) {
            res.push(self.lpf[i]);
            i /= self.lpf[i];
        }
        res.push(self.lpf[i]);
        res
    }
}

#[test]
fn test_prime() {
    let l = LinearSieve::new(1_000_000);
    assert!(!l.is_prime(0));
    assert!(!l.is_prime(1));
    assert!(l.is_prime(2));
    assert!(l.is_prime(278809));
    assert!(!l.is_prime(836427));
}

#[test]
#[should_panic]
fn test_prime_out_of_bounds() {
    let l = LinearSieve::new(10);
    l.is_prime(11);
}

#[test]
fn test_factorize() {
    let l = LinearSieve::new(1_000_000);
    assert_eq!(l.factorize(0), vec![]);
    assert_eq!(l.factorize(1), vec![]);
    assert_eq!(l.factorize(2), vec![2]);
    assert_eq!(l.factorize(120), vec![2, 2, 2, 3, 5]);
    assert_eq!(l.factorize(836427), vec![3, 278809]);
}

#[test]
fn test_list_primes() {
    let l = LinearSieve::new(29);
    assert_eq!(l.primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}
