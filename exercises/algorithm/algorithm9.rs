/*
	heap
	This question requires you to implement a binary heap function
*/
//

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 添加元素到堆的末尾
        self.items.push(value);
        self.count += 1;
        
        // 向上调整堆（上浮）
        let mut idx = self.count;
        while idx > 1 {
            let parent = self.parent_idx(idx);
            
            // 如果当前节点满足堆的比较条件，则停止上浮
            if !(self.comparator)(&self.items[idx], &self.items[parent]) {
                break;
            }
            
            // 交换当前节点与父节点
            self.items.swap(idx, parent);
            idx = parent;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // 获取左子节点索引
        let left = self.left_child_idx(idx);
        
        // 如果左子节点超出范围，返回0（表示没有子节点）
        if left > self.count {
            return 0;
        }
        
        // 获取右子节点索引
        let right = self.right_child_idx(idx);
        
        // 如果右子节点超出范围，返回左子节点索引
        if right > self.count {
            return left;
        }
        
        // 根据比较器比较左右子节点，返回满足条件的子节点索引
        if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // 如果堆为空，返回None
        if self.is_empty() {
            return None;
        }
        
        // 保存堆顶元素
        let result = std::mem::replace(&mut self.items[1], T::default());
        
        // 将最后一个元素移到堆顶
        if self.count > 1 {
            self.items[1] = self.items.pop().unwrap();
        } else {
            self.items.pop();
        }
        
        self.count -= 1;
        
        // 向下调整堆（下沉）
        let mut current = 1;
        while self.children_present(current) {
            let smallest_child = self.smallest_child_idx(current);
            
            // 如果当前节点满足堆的比较条件，则停止下沉
            if (self.comparator)(&self.items[current], &self.items[smallest_child]) {
                break;
            }
            
            // 交换当前节点与最小（或最大）子节点
            self.items.swap(current, smallest_child);
            current = smallest_child;
        }
        
        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}