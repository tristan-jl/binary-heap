#[derive(Debug, Clone)]
pub struct BinaryHeap<T> {
    inner: Vec<T>,
}

impl<T> BinaryHeap<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T: Ord> BinaryHeap<T> {
    pub fn from(items: Vec<T>) -> Self {
        let mut res = Self { inner: items };
        res.make_valid();
        res
    }

    pub fn push(&mut self, item: T) {
        self.inner.push(item);
        self.perc_up(self.inner.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.inner.is_empty() {
            return None;
        }

        let res = self.inner.swap_remove(0);
        self.perc_down(0);
        Some(res)
    }

    pub fn push_pop(&mut self, item: T) -> T {
        if self.inner[0].cmp(&item).is_le() {
            let res = std::mem::replace(&mut self.inner[0], item);
            self.perc_down(0);
            res
        } else {
            item
        }
    }

    fn perc_up(&mut self, pos: usize) {
        if pos == 0 {
            return;
        }
        let parent_pos = (pos - 1) / 2;
        while self.inner[pos].cmp(&self.inner[parent_pos]).is_le() {
            self.inner.swap(pos, parent_pos);
            self.perc_up(parent_pos);
        }
    }

    fn perc_down(&mut self, pos: usize) {
        let left = 2 * pos + 1;
        let right = left + 1;

        if left < self.inner.len() {
            let has_right = right != self.inner.len();
            let to_swap = if has_right && self.inner[left].cmp(&self.inner[right]).is_gt() {
                right
            } else {
                left
            };

            if self.inner[to_swap].cmp(&self.inner[pos]).is_le() {
                self.inner.swap(to_swap, pos);
                self.perc_down(to_swap);
            }
        }
    }

    fn make_valid(&mut self) {
        let mut n = self.inner.len() / 2;
        while n > 0 {
            n -= 1;
            self.perc_down(n);
        }
    }

    #[cfg(test)]
    fn is_valid(&self) -> bool {
        self.inner
            .iter()
            .enumerate()
            .skip(1)
            .all(|(n, child)| (&self.inner[(n - 1) / 2]).cmp(child).is_le())
    }
}

impl<T> Default for BinaryHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts() {
        let mut min_heap = BinaryHeap::new();
        for i in vec![6, 7, 12, 10, 15, 17].into_iter() {
            min_heap.push(i);
        }

        assert_eq!(min_heap.inner, vec![6, 7, 12, 10, 15, 17]);
        assert!(min_heap.is_valid());
    }

    #[test]
    fn inserts1() {
        let mut min_heap = BinaryHeap {
            inner: vec![6, 7, 12, 10, 15, 17],
        };
        min_heap.push(5);

        assert_eq!(min_heap.inner, vec![5, 7, 6, 10, 15, 17, 12]);
        assert!(min_heap.is_valid());
    }

    #[test]
    fn inserts2() {
        use std::cmp::Reverse;
        let mut max_heap = BinaryHeap {
            inner: vec![Reverse(11), Reverse(5), Reverse(8), Reverse(3), Reverse(4)],
        };
        max_heap.push(Reverse(15));

        assert_eq!(
            max_heap.inner,
            vec![
                Reverse(15),
                Reverse(5),
                Reverse(11),
                Reverse(3),
                Reverse(4),
                Reverse(8)
            ]
        );
        assert!(max_heap.is_valid());
    }

    #[test]
    fn pops() {
        use std::cmp::Reverse;
        let mut max_heap = BinaryHeap {
            inner: vec![Reverse(11), Reverse(5), Reverse(8), Reverse(3), Reverse(4)],
        };

        assert_eq!(max_heap.pop().unwrap(), Reverse(11));
        assert_eq!(
            max_heap.inner,
            vec![Reverse(8), Reverse(5), Reverse(4), Reverse(3),]
        );
        assert!(max_heap.is_valid());
    }

    #[test]
    fn pops1() {
        let mut min_heap = BinaryHeap {
            inner: vec![5, 7, 6, 10, 15, 17, 12],
        };

        assert_eq!(min_heap.pop().unwrap(), 5);
        assert_eq!(min_heap.inner, vec![6, 7, 12, 10, 15, 17]);
        assert!(min_heap.is_valid());
    }

    #[test]
    fn push_pop() {
        let mut min_heap = BinaryHeap {
            inner: vec![6, 7, 12, 10, 15, 17],
        };

        assert_eq!(min_heap.push_pop(1), 1);
        assert!(min_heap.is_valid());
    }

    #[test]
    fn push_pop1() {
        let mut min_heap = BinaryHeap {
            inner: vec![6, 7, 12, 10, 15, 17],
        };

        assert_eq!(min_heap.push_pop(50), 6);
        assert!(min_heap.is_valid());
    }

    #[test]
    fn from_vect() {
        let mut x = vec![5, 7, 6, 10, 15, 17, 12];
        x.reverse();
        let min_heap = BinaryHeap::from(x);

        assert!(min_heap.is_valid());
    }
}
