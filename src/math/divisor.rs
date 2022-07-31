use cargo_snippet::snippet;

#[snippet("divisor")]
pub fn enumerate_divisors(n: usize) -> Vec<usize> {
    let mut front = vec![];
    let mut back = vec![];
    for i in (1..).take_while(|&i| i * i <= n) {
        if n % i != 0 {
            continue;
        }
        front.push(i);
        if n / i != i {
            back.push(n / i);
        }
    }
    while let Some(v) = back.pop() {
        front.push(v);
    }
    front
}

#[test]
fn test_divisors() {
    assert_eq!(enumerate_divisors(10), [1, 2, 5, 10]);
    assert_eq!(enumerate_divisors(25), [1, 5, 25]);
    assert_eq!(enumerate_divisors(17), [1, 17]);
}
