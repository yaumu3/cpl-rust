use cargo_snippet::snippet;

#[snippet("binary_search")]
/// Binary search trait.
pub trait BinarySearch<T> {
    fn binary_search(&self, good: T, bad: T, eps: Option<T>) -> Option<T>;
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
    /// That is, there is 0 or 1 `x` satisfies `F(x) && (F(x - eps) ^ F(x + eps))`.
    ///
    /// # Arguments
    ///
    /// * `good` - Domain boundary to inclue.
    /// * `bad` - Domain boundary to exclude.
    /// * `eps` - Upper bound of |`good` - `bad`| to stop search.
    /// If `None`, multiplicative identity is applied.
    ///
    /// # Returns
    ///
    /// * `good == bad` or either domain boundary is uncomparable object -> `None`
    /// * All range is `true` -> `Some(good)`
    /// * All range is `false` -> `Some(bad ± eps)` (whichever close to `good`)
    /// * Otherwise, flipping key `Some(x)`.
    ///
    /// # Examples
    ///
    /// ```
    /// //! Compute square root of 2
    /// use cpl_rust::algorithms::binary_search::BinarySearch;
    /// let f = |x| x * x >= 2.;
    /// let eps = 1e-3;
    /// let sqrt_2 = f.binary_search(2., 1., Some(eps)).unwrap();
    /// let delta = sqrt_2 - 2.0f64.sqrt();
    /// assert!(delta > 0. && delta <= eps);
    /// ```
    fn binary_search(&self, good: T, bad: T, eps: Option<T>) -> Option<T> {
        if good == bad || good.partial_cmp(&bad).is_none() {
            return None;
        }

        // Get multiplicative identity `1` by division while avoiding zero division.
        // Since it is assured that `good != bad`,
        // `good + bad == bad` means `good` is additive identity `0`.
        let one = if good + bad == bad {
            bad.div(bad)
        } else {
            good.div(good)
        };
        let eps = eps.unwrap_or(one);
        let two = one + one;

        // Tweak to avoid using `abs` method.
        let has_range = |good: T, bad: T| match good.partial_cmp(&bad) {
            Some(std::cmp::Ordering::Greater) => good > eps + bad,
            Some(std::cmp::Ordering::Less) => bad > eps + good,
            _ => unreachable!(),
        };

        let (mut good, mut bad) = (good, bad);
        while has_range(good, bad) {
            let mid = (good + bad) / two;
            if self(mid) {
                good = mid;
            } else {
                bad = mid;
            }
        }
        Some(good)
    }
}

#[snippet("element_bisect", include = "binary_search")]
/// Trait to locate insertion point in slice by binary search.
/// Equivalent to `bisect.bisect_left/right` of Python3
pub trait ElementBisect<T> {
    fn bisect_left(&self, x: &T) -> usize;
    fn bisect_right(&self, x: &T) -> usize;
}

#[snippet("element_bisect", include = "binary_search")]
impl<T: PartialOrd> ElementBisect<T> for [T] {
    /// Locate the **left**-most insertion point for `x` in sorted `[T]`
    /// to maintain sorted order.
    fn bisect_left(&self, x: &T) -> usize {
        let f = |i: i64| self[i as usize] >= *x;
        f.binary_search(self.len() as i64, -1, None).unwrap() as usize
    }

    /// Locate the **right**-most insertion point for `x` in sorted `[T]`
    /// to maintain sorted order.
    fn bisect_right(&self, x: &T) -> usize {
        let f = |i: i64| self[i as usize] > *x;
        f.binary_search(self.len() as i64, -1, None).unwrap() as usize
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
        let ans = is_good.binary_search(1_000_000_000, 0, None).unwrap();
        assert_eq!(ans, *out);
    }
}

#[test]
fn test_binary_search_with_partial_ord() {
    let f = |x| x * x >= 2.;
    let eps = 1e-3;
    let sqrt_2 = f.binary_search(2., 1., Some(eps)).unwrap();
    let delta = sqrt_2 - 2.0f64.sqrt();
    assert!(delta > 0. && delta <= eps);
}

#[test]
fn test_binary_search_returns_none_with_equal_good_and_bad() {
    assert_eq!((|v| v > 0).binary_search(1, 1, None), None);
}

#[test]
fn test_binary_search_returns_none_with_nan_specified_as_good() {
    assert_eq!(
        (|v: f64| v - 2. > 0.).binary_search(std::f64::NAN, 0., Some(1e-5)),
        None
    );
}

#[test]
fn test_binary_search_returns_none_with_nan_specified_as_bad() {
    assert_eq!(
        (|v: f64| v - 2. > 0.).binary_search(0., std::f64::NAN, Some(1e-5)),
        None
    );
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
