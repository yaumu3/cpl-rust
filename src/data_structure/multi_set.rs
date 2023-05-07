use cargo_snippet::snippet;

#[snippet("multi_set")]
#[derive(Debug)]
pub struct MultiSet<T> {
    len: usize,
    multi_set: std::collections::BTreeMap<T, usize>,
}

#[snippet("multi_set")]
pub struct Iter<'a, T> {
    iter: std::collections::btree_map::Iter<'a, T, usize>,
    front: Option<&'a T>,
    front_count: usize,
    back: Option<&'a T>,
    back_count: usize,
}

#[snippet("multi_set")]
impl<'a, T> Iter<'a, T> {
    fn new(ms: &'a MultiSet<T>) -> Self {
        Self {
            iter: ms.multi_set.iter(),
            front: None,
            front_count: 0,
            back: None,
            back_count: 0,
        }
    }
}

#[snippet("multi_set")]
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.front_count == 0 {
            if let Some((k, &v)) = self.iter.next() {
                self.front = Some(k);
                self.front_count = v;
            }
        }
        if self.front_count > 0 {
            self.front_count -= 1;
            self.front
        } else {
            None
        }
    }
}

#[snippet("multi_set")]
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back_count == 0 {
            if let Some((k, &v)) = self.iter.next_back() {
                self.back = Some(k);
                self.back_count = v;
            }
        }
        if self.back_count > 0 {
            self.back_count -= 1;
            self.back
        } else {
            None
        }
    }
}

#[snippet("multi_set")]
impl<T: Ord> Default for MultiSet<T> {
    fn default() -> Self {
        Self {
            len: 0,
            multi_set: std::collections::BTreeMap::new(),
        }
    }
}

#[snippet("multi_set")]
impl<T: Ord + Clone> MultiSet<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_slice(slice: &[T]) -> Self {
        let mut result = Self::new();
        for e in slice {
            result.insert(e.clone());
        }
        result
    }
    pub fn clear(&mut self) {
        self.len = 0;
        self.multi_set.clear()
    }
    pub fn is_empty(&self) -> bool {
        self.multi_set.is_empty()
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn count(&self, e: &T) -> usize {
        *self.multi_set.get(e).unwrap_or(&0)
    }
    pub fn insert(&mut self, e: T) {
        self.len += 1;
        *self.multi_set.entry(e).or_insert(0) += 1;
    }
    pub fn contains(&self, e: &T) -> bool {
        self.multi_set.contains_key(e)
    }
    pub fn remove(&mut self, e: &T) -> bool {
        if !self.contains(e) {
            return false;
        }
        self.len -= 1;
        *self.multi_set.get_mut(e).unwrap() -= 1;
        if self.count(e) == 0 {
            self.multi_set.remove(e);
        }
        true
    }
    pub fn first(&self) -> Option<&T> {
        self.multi_set.keys().next()
    }
    pub fn last(&self) -> Option<&T> {
        self.multi_set.keys().last()
    }
    pub fn pop_first(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let min_key = self.first().unwrap().clone();
        self.remove(&min_key);
        Some(min_key)
    }
    pub fn pop_last(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let max_key = self.last().unwrap().clone();
        self.remove(&max_key);
        Some(max_key)
    }
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_set_is_empty_on_construction() {
        let ms: MultiSet<usize> = MultiSet::new();
        assert!(ms.is_empty());
    }

    #[test]
    fn test_is_empty_after_clear() {
        let mut ms = MultiSet::from_slice(&[1, 2, 3, 4]);
        ms.clear();
        assert!(ms.is_empty());
    }

    #[test]
    fn test_multi_set_can_count_contained_value() {
        let ms = MultiSet::from_slice(&[1, 1]);
        assert!(ms.count(&1) == 2);
    }

    #[test]
    fn test_multi_set_len_is_total_of_elements() {
        let array = [1, 1, 2, 3];
        let ms = MultiSet::from_slice(&array);
        assert!(ms.len() == array.len());
    }

    #[test]
    fn test_remove_decrements_count_if_the_element_was_present() {
        let array = [2, 3, 5, 5, 7, 11];
        let mut ms = MultiSet::from_slice(&array);
        ms.remove(&5);
        assert_eq!(ms.count(&5), 1);
        assert_eq!(ms.len(), array.len() - 1);
    }

    #[test]
    fn test_empty_after_last_element_removed() {
        let array = [13];
        let mut ms = MultiSet::from_slice(&array);
        ms.remove(&13);
        assert!(ms.is_empty());
    }

    #[test]
    fn test_remove_does_not_change_len_if_element_was_not_present() {
        let array = [2, 3, 5, 5, 7, 11];
        let mut ms = MultiSet::from_slice(&array);
        ms.remove(&4);
        assert_eq!(ms.len(), array.len());
    }

    #[test]
    fn test_remove_returns_if_the_element_was_present() {
        let array = [2, 3, 5, 7, 11];
        let mut ms = MultiSet::from_slice(&array);
        assert!(ms.remove(&5));
        assert!(!ms.remove(&4));
    }

    #[test]
    fn test_pop_first_is_ord_based() {
        let array = [4, 2, 1, 3];
        let mut ms = MultiSet::from_slice(&array);
        let poped = ms.pop_first();
        assert_eq!(poped, Some(1));
        assert!(!ms.contains(&1));
        assert_eq!(ms.len(), 3);
    }

    #[test]
    fn test_pop_last_is_ord_based() {
        let array = [0, 4, 2, 1, 3];
        let mut ms = MultiSet::from_slice(&array);
        let poped = ms.pop_last();
        assert_eq!(poped, Some(4));
        assert!(!ms.contains(&4));
        assert_eq!(ms.len(), 4);
    }

    #[test]
    fn test_iter() {
        let array = [3, 2, 1, 1, 3, 0, 0, 2];
        let ms = MultiSet::from_slice(&array);
        let mut iter = ms.iter();
        assert_eq!(Some(&0), iter.next());
        assert_eq!(Some(&3), iter.next_back());
        assert_eq!(Some(&3), iter.next_back());
        assert_eq!(Some(&0), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());
    }
}
