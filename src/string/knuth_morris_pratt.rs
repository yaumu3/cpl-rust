use cargo_snippet::snippet;

#[snippet("knuth_morris_pratt")]
/// Knuth-Morris-Pratt algorithm for pattern search.
pub struct KnuthMorrisPratt<'a, T: PartialEq> {
    target: &'a [T],
}

#[snippet("knuth_morris_pratt")]
impl<'a, T: PartialEq> KnuthMorrisPratt<'a, T> {
    fn get_failure_function(pattern: &[T]) -> Vec<usize> {
        // `fail[j]` = Length of the longest proper prefix of `&pattern[0..j]`
        // which is also a suffix of the slice.
        let m = pattern.len();
        let mut fail = vec![0; m + 1];

        for i in 2..=m {
            let mut j = fail[i - 1];
            loop {
                if pattern[j] == pattern[i - 1] {
                    fail[i] = j + 1;
                    break;
                }
                if j == 0 {
                    fail[i] = 0;
                    break;
                }
                j = fail[j];
            }
        }
        fail
    }

    pub fn new(target: &'a [T]) -> Self {
        Self { target }
    }

    /// Find all start indices where `pattern` occur
    pub fn find_all(&self, pattern: &[T]) -> Vec<usize> {
        let (n, m) = (self.target.len(), pattern.len());
        let mut indices = vec![];
        let fail = Self::get_failure_function(pattern);
        let (mut i, mut j) = (0, 0);
        while i < n {
            if self.target[i] == pattern[j] {
                i += 1;
                j += 1;
                // Matched
                if j == m {
                    indices.push(i - m);
                    j = fail[j];
                }
            } else if j > 0 {
                j = fail[j];
            } else {
                i += 1;
            }
        }
        indices
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failure_function() {
        let text = "ABCABDA";
        let fail = KnuthMorrisPratt::get_failure_function(text.as_bytes());
        assert_eq!(fail, vec![0, 0, 0, 0, 1, 2, 0, 1]);
    }

    #[test]
    fn test_failure_function_with_single_element() {
        let text = "A";
        let fail = KnuthMorrisPratt::get_failure_function(text.as_bytes());
        assert_eq!(fail, vec![0, 0]);
    }

    #[test]
    fn test_failure_function_with_no_element() {
        let text = "";
        let fail = KnuthMorrisPratt::get_failure_function(text.as_bytes());
        assert_eq!(fail, vec![0]);
    }

    #[test]
    fn test_find_all() {
        let text = KnuthMorrisPratt::new("AABAACAADAABAABA".as_bytes());
        let matched = text.find_all("AABA".as_bytes());
        assert_eq!(matched, vec![0, 9, 12]);
    }

    #[test]
    fn test_find_all_no_match() {
        let text = KnuthMorrisPratt::new("AAAA".as_bytes());
        let matched = text.find_all("ZZ".as_bytes());
        assert_eq!(matched, vec![]);
    }

    #[test]
    fn test_find_all_no_match_as_pattern_len_exceeds_text() {
        let text = KnuthMorrisPratt::new("AA".as_bytes());
        let matched = text.find_all("AAA".as_bytes());
        assert_eq!(matched, vec![]);
    }
}
