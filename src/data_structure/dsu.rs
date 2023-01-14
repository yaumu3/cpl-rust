use cargo_snippet::snippet;

#[snippet("dsu")]
pub struct DisjointSet {
    n: usize,
    parent_or_size: Vec<isize>,
}

#[snippet("dsu")]
impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.n);
        assert!(b < self.n);
        let mut x = self.leader(a);
        let mut y = self.leader(b);
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as isize;
        x
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }

    pub fn leader(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        if self.parent_or_size[a] < 0 {
            a
        } else {
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as isize;
            self.parent_or_size[a] as usize
        }
    }

    pub fn size(&mut self, a: usize) -> usize {
        let x = self.leader(a);
        -self.parent_or_size[x] as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_panics_out_of_bounds() {
        let mut dsu = DisjointSet::new(10);
        dsu.leader(10);
    }

    #[test]
    fn test_leader_returns_smallest_element_number() {
        let mut dsu = DisjointSet::new(10);
        dsu.merge(1, 3);
        dsu.merge(2, 3);
        dsu.merge(4, 1);
        assert_eq!(dsu.leader(2), 1);
    }

    #[test]
    fn test_merge_returns_leader_number() {
        let mut dsu = DisjointSet::new(10);
        let x = dsu.merge(1, 3);
        assert_eq!(x, dsu.leader(3));
    }

    #[test]
    fn test_in_same_set_when_merged() {
        let mut dsu = DisjointSet::new(10);
        dsu.merge(1, 3);
        dsu.merge(2, 3);
        assert!(dsu.same(1, 2));
    }

    #[test]
    fn test_size_gets_bigger_upon_merge() {
        let mut dsu = DisjointSet::new(10);
        dsu.merge(1, 3);
        dsu.merge(1, 5);
        assert_eq!(dsu.size(3), 3);
    }
}
