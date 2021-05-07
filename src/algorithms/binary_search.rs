use cargo_snippet::snippet;

#[snippet("binary_search")]
/// Binary search trait.
pub trait BinarySearch<T> {
    fn binary_search(&self, bad: T, good: T, eps: Option<T>) -> T;
}

#[snippet("binary_search")]
impl<T, F> BinarySearch<T> for F
where
    T: Copy + PartialOrd + std::ops::Add<Output = T> + std::ops::Div<Output = T>,
    F: Fn(T) -> bool,
{
    /// Search a flipping point within a given domain of a function `F(T) -> bool`
    /// by binary search algorithm.
    ///
    /// It is asserted that `F` is weakly monotone from `good` inclusive to `bad` exclusive.
    /// That is, there is 0 or 1 `x` satisfies `F(x) && !(F(x - eps) && F(x + eps))`.
    ///
    /// # Arguments
    ///
    /// * `bad` - Domain boundary to exclude.
    /// * `good` - Domain boundary to inclue.
    /// * `eps` - Upper bound of |`bad` - `good`| to stop search.
    /// If `None`, multiplicative identity is applied.
    ///
    /// # Returns
    ///
    /// * All range is `true` -> `good`
    /// * All range is `false` -> `bad` + `eps`
    /// * Otherwise, flipping key `k`.
    ///
    /// # Panics
    ///
    /// * When `bad` is equal to `good`.
    /// * When either domain boundary is uncomparable object.
    ///
    /// # Examples
    ///
    /// ```
    /// //! Compute square root of 2 by bisection.
    /// use cpl_rust::algorithms::binary_search::BinarySearch;
    /// let f = |x| x * x >= 2.;
    /// let eps = 1e-3;
    /// let sqrt_2 = f.binary_search(1., 2., Some(eps));
    /// let delta = sqrt_2 - 2.0f64.sqrt();
    /// assert!(delta > 0. && delta <= eps);
    /// ```
    fn binary_search(&self, bad: T, good: T, eps: Option<T>) -> T {
        if bad == good {
            panic!("`bad` and `good` must be different.")
        }

        // Get multiplicative identity `1` by division while avoiding zero division.
        // Since it is assured that `bad != good`,
        // `bad + good == bad` means `good` is additive identity `0`.
        let one = if bad + good == bad {
            bad.div(bad)
        } else {
            good.div(good)
        };
        let eps = eps.unwrap_or(one);
        let two = one + one;

        // Tweak to avoid using `abs` method.
        let has_range = |bad: T, good: T| match good.partial_cmp(&bad) {
            Some(std::cmp::Ordering::Greater) => good > eps + bad,
            Some(std::cmp::Ordering::Less) => bad > eps + good,
            None => panic!("Put away `NaN`!"),
            _ => unreachable!(),
        };

        let (mut bad, mut good) = (bad, good);
        while has_range(bad, good) {
            let mid = (bad + good) / two;
            if self(mid) {
                good = mid;
            } else {
                bad = mid;
            }
        }
        good
    }
}

#[snippet("array_bisect", include = "binary_search")]
/// Trait to locate insertion point in slice by binary search.
/// Equivalent to `bisect.bisect_left/right` of Python3
pub trait ArrayBisect<T> {
    fn bisect_left(&self, x: &T) -> usize;
    fn bisect_right(&self, x: &T) -> usize;
}

#[snippet("array_bisect", include = "binary_search")]
impl<T: PartialOrd> ArrayBisect<T> for [T] {
    /// Locate the **left**-most insertion point for `x` in sorted `[T]`
    /// to maintain sorted order.
    fn bisect_left(&self, x: &T) -> usize {
        let f = |i: i64| self[i as usize] >= *x;
        f.binary_search(-1, self.len() as i64, None) as usize
    }

    /// Locate the **right**-most insertion point for `x` in sorted `[T]`
    /// to maintain sorted order.
    fn bisect_right(&self, x: &T) -> usize {
        let f = |i: i64| self[i as usize] > *x;
        f.binary_search(-1, self.len() as i64, None) as usize
    }
}

#[test]
fn test_binary_search() {
    // ABC174-E `Logs`
    // https://atcoder.jp/contests/abc174/tasks/abc174_e

    let samples = [
        (9, vec![4, 4, 4], 1),
        (0, vec![1_000_000_000, 1_000_000_000], 1_000_000_000),
        (3, vec![7, 9], 4),
    ];

    for (k, a, out) in &samples {
        let is_good = |v: u32| a.iter().map(|ai| (ai - 1) / v).sum::<u32>() <= *k;
        let ans = is_good.binary_search(0, 1_000_000_000, None);
        assert_eq!(ans, *out);
    }
}

#[test]
fn test_binary_search_with_partial_ord() {
    let f = |x| x * x >= 2.;
    let eps = 1e-3;
    let sqrt_2 = f.binary_search(1., 2., Some(eps));
    let delta = sqrt_2 - 2.0f64.sqrt();
    assert!(delta > 0. && delta <= eps);
}

#[test]
#[should_panic(expected = "`bad` and `good` must be different.")]
fn test_binary_search_panics_with_equal_bad_and_good() {
    (|v| v > 0).binary_search(1, 1, None);
}

#[test]
#[should_panic(expected = "Put away `NaN`!")]
fn test_binary_search_panics_with_nan_specified_as_good() {
    (|v: f64| v - 2. > 0.).binary_search(0., std::f64::NAN, Some(1e-5));
}

#[test]
#[should_panic(expected = "Put away `NaN`!")]
fn test_flips_at_panics_with_nan_specified_as_bad() {
    (|v: f64| v - 2. > 0.).binary_search(std::f64::NAN, 0., Some(1e-5));
}

#[test]
fn test_bisect() {
    let li = [1, 2, 2, 2, 4, 5, 7];

    // bisect_left
    assert_eq!(li.bisect_left(&-1), 0);
    assert_eq!(li.bisect_left(&2), 1);
    assert_eq!(li.bisect_left(&4), 4);
    assert_eq!(li.bisect_left(&7), 6);
    assert_eq!(li.bisect_left(&8), 7);

    // bisect_right
    assert_eq!(li.bisect_right(&-1), 0);
    assert_eq!(li.bisect_right(&2), 4);
    assert_eq!(li.bisect_right(&4), 5);
    assert_eq!(li.bisect_right(&7), 7);
    assert_eq!(li.bisect_right(&8), 7);
}

#[test]
fn test_bisect_str() {
    let li = ["aab", "aac", "aad"];
    assert_eq!(li.bisect_left(&"aab"), 0);
    assert_eq!(li.bisect_right(&"aab"), 1);
}

#[test]
fn test_bisect_partial_ord() {
    let li = [1.0, 1.2, 2.0, 2.0, 4.8, 5.7, 7.9];
    assert_eq!(li.bisect_left(&2.0), 2);
    assert_eq!(li.bisect_right(&2.0), 4);
}