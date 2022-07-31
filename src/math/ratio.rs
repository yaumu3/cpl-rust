use cargo_snippet::snippet;

#[snippet]
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + PartialEq + std::ops::Rem<Output = T> + std::ops::Add<Output = T>,
{
    if b == b + b {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet]
#[snippet(include = "gcd")]
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy
        + PartialEq
        + std::ops::Rem<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    a / gcd(a, b) * b
}

#[snippet("ratio")]
#[snippet(include = "gcd")]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Ratio {
    numerator: i64,
    denominator: i64,
}
#[snippet("ratio")]
impl Ratio {
    pub fn new(num: i64, den: i64) -> Self {
        if den == 0 {
            panic!("Ratio: divide by zero");
        }
        let g = gcd(num, den);
        let num = num / g;
        let den = den / g;
        let s = if den < 0 { -1 } else { 1 };
        Ratio {
            numerator: s * num,
            denominator: s * den,
        }
    }
    pub fn from_integer(n: i64) -> Self {
        Ratio {
            numerator: n,
            denominator: 1,
        }
    }
    pub fn inverse(&self) -> Self {
        Ratio {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }
}
#[snippet("ratio")]
impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let g = gcd(self.denominator, other.denominator);
        (other.denominator / g * self.numerator)
            .partial_cmp(&(self.denominator / g * other.numerator))
    }
}
#[snippet("ratio")]
impl std::cmp::Ord for Ratio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
#[snippet("ratio")]
impl std::ops::Neg for Ratio {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Ratio::new(self.numerator, -self.denominator)
    }
}
#[snippet("ratio")]
impl std::ops::Add for Ratio {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let num = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        let den = self.denominator * rhs.denominator;
        Ratio::new(num, den)
    }
}
#[snippet("ratio")]
impl std::ops::Sub for Ratio {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
#[snippet("ratio")]
impl std::ops::Mul for Ratio {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Ratio::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}
#[snippet("ratio")]
#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Div for Ratio {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(10, 4), 2);
    assert_eq!(gcd(42, 11), 1);
    assert_eq!(gcd(10, 0), 10);
    assert_eq!(gcd(10, 1), 1);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(10, 4), 20);
    assert_eq!(lcm(42, 11), 462);
    assert_eq!(lcm(10, 0), 0);
    assert_eq!(lcm(10, 1), 10);
}

#[test]
#[should_panic]
fn test_panic_at_inifinity_ratio() {
    Ratio::new(1, 0);
}

#[test]
fn test_ratio_from_integer() {
    let a = Ratio::from_integer(3);
    let b = Ratio::new(3, 1);
    assert_eq!(a, b);
}

#[test]
fn test_ratio_is_irreducible() {
    let a = Ratio::new(3, 5);
    let b = Ratio::new(12, 20);
    assert_eq!(a, b);
}

#[test]
fn test_negation_is_regularized() {
    let a = Ratio::new(3, -5);
    assert_eq!(a.numerator, -3);
    let b = Ratio::new(0, -3);
    assert_eq!(b.denominator, 1);
}

#[test]
fn test_ratio_cmp() {
    let a = Ratio::new(3, 5);
    let b = Ratio::new(2, 7);
    let c = Ratio::new(3, 5);
    assert!(a > b);
    assert!(a >= c);
}

#[test]
fn test_ratio_addition() {
    let a = Ratio::new(3, 5);
    let b = Ratio::new(2, 7);
    let c = Ratio::new(31, 35);
    assert_eq!(a + b, c);
}

#[test]
fn test_ratio_negation() {
    let a = Ratio::new(3, 5);
    let b = Ratio::new(-3, 5);
    let c = Ratio::new(3, -5);
    assert_eq!(-a, b);
    assert_eq!(-a, c);
}

#[test]
fn test_ratio_subtraction() {
    let a = Ratio::new(3, 5);
    let b = Ratio::new(2, 7);
    let c = Ratio::new(11, 35);
    assert_eq!(a - b, c);
}

#[test]
fn test_ratio_multiplication() {
    let a = Ratio::new(3, 5);
    let b = Ratio::new(2, 7);
    let c = Ratio::new(6, 35);
    assert_eq!(a * b, c);
}

#[test]
fn test_ratio_inversion() {
    let a = Ratio::new(3, 5);
    let b = Ratio::new(5, 3);
    assert_eq!(a.inverse(), b);
}

#[test]
fn test_ratio_division() {
    let a = Ratio::new(3, 5);
    let b = Ratio::new(2, 7);
    let c = Ratio::new(21, 10);
    assert_eq!(a / b, c);
}
