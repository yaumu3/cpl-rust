use cargo_snippet::snippet;

#[snippet("eratosthenes")]
/// Eratosthenes sieve for primarity test and prime factorization.
///
/// * `lpf`: `lpf[i]` is the least prime factor of `i`.
/// e.g.) `lpf[7] == 7`, `lpf[20] == 5`, `lpf[30] == 2`.
pub struct Eratosthenes {
    lpf: Vec<usize>,
}

#[snippet("eratosthenes")]
impl Eratosthenes {
    /// Constructs a new Eratosthenes struct.
    ///
    /// # Arguments
    ///
    /// * `n_max`: Max number to check primarity (inclusive).
    pub fn new(n_max: usize) -> Self {
        let mut lpf: Vec<usize> = (0..=n_max).collect();
        let mut i = 2;
        while i * i <= n_max {
            if lpf[i] == i {
                let mut j = i * i;
                while j <= n_max {
                    lpf[j] = lpf[j].min(i);
                    j += i;
                }
            }
            i += 1;
        }
        Self { lpf }
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
    let e = Eratosthenes::new(1_000_000);
    assert!(!e.is_prime(0));
    assert!(!e.is_prime(1));
    assert!(e.is_prime(2));
    assert!(e.is_prime(278809));
    assert!(!e.is_prime(836427));
}

#[test]
#[should_panic]
fn test_prime_out_of_bounds() {
    let e = Eratosthenes::new(10);
    e.is_prime(11);
}

#[test]
fn test_factorize() {
    let e = Eratosthenes::new(1_000_000);
    assert_eq!(e.factorize(0), vec![]);
    assert_eq!(e.factorize(1), vec![]);
    assert_eq!(e.factorize(2), vec![2]);
    assert_eq!(e.factorize(120), vec![2, 2, 2, 3, 5]);
    assert_eq!(e.factorize(836427), vec![3, 278809]);
}
