use cargo_snippet::snippet;

#[snippet("segment_tree")]
pub struct SegmentTree<T, Op, Id> {
    n: usize,
    node: Vec<T>,
    op: Op,
    id: Id,
}

#[snippet("segment_tree")]
impl<T, Op, Id> std::ops::Index<usize> for SegmentTree<T, Op, Id> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        assert!(i < self.n);
        &self.node[i + self.n]
    }
}

#[snippet("segment_tree")]
impl<T, Op, Id> std::fmt::Debug for SegmentTree<T, Op, Id>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.node[self.n..])
    }
}

#[snippet("segment_tree")]
/// Abstract segment tree.
impl<T, Op, Id> SegmentTree<T, Op, Id>
where
    T: Copy,
    Op: Fn(T, T) -> T,
    Id: Fn() -> T,
{
    pub fn new(slice: &[T], op: Op, id: Id) -> Self {
        let n = slice.len().next_power_of_two();
        let mut node = vec![id(); n << 1];
        for (i, &x) in slice.iter().enumerate() {
            node[i + n] = x;
        }
        for i in (1..n).rev() {
            node[i] = op(node[i << 1], node[i << 1 | 1]);
        }
        Self { n, node, op, id }
    }

    /// Update value for `i`th element.
    pub fn update(&mut self, i: usize, x: T) {
        assert!(i < self.n);
        let mut i = i + self.n;
        self.node[i] = x;
        while i > 1 {
            i >>= 1;
            self.node[i] = (self.op)(self.node[i << 1], self.node[i << 1 | 1]);
        }
    }

    /// Query value `op` acted on range [`left`, `right`).
    pub fn query(&self, left: Option<usize>, right: Option<usize>) -> T {
        let mut l = left.unwrap_or(0) + self.n;
        let mut r = right.unwrap_or(self.n) + self.n;
        assert!(l <= r && l <= self.node.len() && r <= self.node.len());
        let mut res = (self.id)();
        while l < r {
            if l & 1 == 1 {
                res = (self.op)(res, self.node[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res = (self.op)(res, self.node[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }
}

#[test]
fn test_tree_is_indexable() {
    let node = [1, 2, -91, 20, 5, 10, 970];
    let t = SegmentTree::new(&node, |a, b| a + b, || 0);
    assert_eq!(t[2], -91);
    assert_eq!(t[7], 0);
}

#[test]
fn test_tree_is_debuggable() {
    let node = [1, 2];
    let t = SegmentTree::new(&node, |a, b| a + b, || 0);
    assert_eq!(format!("{:?}", t), "[1, 2]");
}

#[test]
fn test_query() {
    let node = [1, 2, -91, 20, 5, 10, 970];
    let t = SegmentTree::new(&node, |a, b| a + b, || 0);
    for i in 0..=node.len() {
        for j in i..=node.len() {
            let res = t.query(Some(i), Some(j));
            assert_eq!(res, node[i..j].iter().sum::<i32>());
        }
    }
}

#[test]
fn test_whole_query() {
    let node = [1, 2, -91, 20, 5, 10, 970];
    let tree = SegmentTree::new(
        &node,
        |a, b| std::cmp::min(a, b),
        || *node.iter().max().unwrap(),
    );
    let whole_min = tree.query(None, None);
    assert_eq!(whole_min, -91);
    let right_min = tree.query(Some(3), None);
    assert_eq!(right_min, 5);
    let left_min = tree.query(None, Some(2));
    assert_eq!(left_min, 1);
}
