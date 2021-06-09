use cargo_snippet::snippet;

#[snippet("z_algorithm")]
pub fn get_z_array<T: PartialEq>(s: &[T]) -> Vec<usize> {
    // `z[i]` = Length of the longest slice starting from `s[i]`
    // which is also a proper prefix of `s`.
    let n = s.len();
    let mut z = vec![0; n];

    // Z-box window [left, right)
    let (mut left, mut right) = (0, 0);

    for i in 1..n {
        if i >= right {
            left = i;
            right = i;
            while right < n && s[right - left] == s[right] {
                right += 1;
            }
            z[i] = right - left;
            continue;
        }
        let k = i - left;
        if z[k] < right - i {
            z[i] = z[k];
            continue;
        }
        left = i;
        while right < n && s[right - left] == s[right] {
            right += 1;
        }
        z[i] = right - left;
    }
    z
}

#[test]
fn test_z_algorithm() {
    let pattern = ['a', 'a', 'x', 'y', 'a', 'a', 'x', 'a', 'a', 'b'];
    assert_eq!(get_z_array(&pattern), vec![0, 1, 0, 0, 3, 1, 0, 2, 1, 0]);
}

#[test]
fn test_pattern_search() {
    let target = "ggccgggccctgtgaccacag";
    let pattern = "ggc";
    let n = pattern.len();
    let s = [pattern.as_bytes(), &[0], target.as_bytes()].concat();
    let pos = get_z_array(&s)
        .into_iter()
        .enumerate()
        .skip(n + 1)
        .filter(|&(_, cnt)| cnt == n)
        .map(|(i, _)| i - n - 1)
        .collect::<Vec<_>>();
    assert_eq!(pos, vec![0, 5]);
}
