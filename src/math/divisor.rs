use cargo_snippet::snippet;

#[snippet("divisor")]
pub trait Divisor {
    fn divisors(self) -> Vec<Self>
    where
        Self: Sized;
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
}

#[macro_export]
#[snippet("divisor")]
macro_rules! divisor_impl {
    ($t: ty) => {
        impl Divisor for $t {
            fn divisors(self) -> Vec<$t> {
                let mut front = vec![];
                let mut back = vec![];
                for i in (1..).take_while(|i| i * i <= self) {
                    if self % i != 0 {
                        continue;
                    }
                    front.push(i);
                    if self / i != i {
                        back.push(self / i);
                    }
                }
                while let Some(v) = back.pop() {
                    front.push(v);
                }
                front
            }
            fn gcd(self, other: Self) -> $t {
                if other == 0 {
                    self
                } else {
                    other.gcd(self % other)
                }
            }
            fn lcm(self, other: Self) -> $t {
                self / self.gcd(other) * other
            }
        }
    };
}

#[test]
fn test_divisors() {
    divisor_impl!(usize);
    assert_eq!(10.divisors(), [1, 2, 5, 10]);
    assert_eq!(25.divisors(), [1, 5, 25]);
    assert_eq!(17.divisors(), [1, 17]);
}

#[test]
fn test_gcd() {
    assert_eq!(10.gcd(4), 2);
    assert_eq!(42.gcd(11), 1);
    assert_eq!(10.gcd(0), 10);
    assert_eq!(10.gcd(1), 1);
}

#[test]
fn test_lcm() {
    assert_eq!(10.lcm(4), 20);
    assert_eq!(42.lcm(11), 462);
    assert_eq!(10.lcm(0), 0);
    assert_eq!(10.lcm(1), 10);
}
