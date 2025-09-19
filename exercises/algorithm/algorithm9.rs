/*
	heap
	This question requires you to implement a binary heap function
*/


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
        // 增加元素计数
        self.count += 1;
        
        // 如果需要扩展数组
        if self.count >= self.items.len() {
            self.items.push(T::default());
        }
        
        // 将新元素放在数组末尾
        self.items[self.count] = value;
        
        // 上浮操作，维护堆性质
        self.heapify_up(self.count);
    }
    
    // 上浮操作：将指定位置的元素向上移动直到满足堆性质
    fn heapify_up(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            
            // 如果当前元素满足堆性质，停止上浮
            if !(self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                break;
            }
            
            // 交换当前元素和父元素
            self.items.swap(idx, parent_idx);
            idx = parent_idx;
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
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        // 如果没有子节点，返回当前索引
        if left_idx > self.count {
            return idx;
        }
        
        // 如果只有左子节点
        if right_idx > self.count {
            return left_idx;
        }
        
        // 比较左右子节点，返回满足堆性质的那个
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }
    
    // 下沉操作：将指定位置的元素向下移动直到满足堆性质
    fn heapify_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let target_idx = self.smallest_child_idx(idx);
            
            // 如果当前元素已经满足堆性质，停止下沉
            if target_idx == idx || !(self.comparator)(&self.items[target_idx], &self.items[idx]) {
                break;
            }
            
            // 交换当前元素和目标子元素
            self.items.swap(idx, target_idx);
            idx = target_idx;
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
        if self.count == 0 {
            return None;
        }
        
        // 保存堆顶元素（要返回的值）
        let result = std::mem::replace(&mut self.items[1], T::default());
        
        // 将最后一个元素移到堆顶
        if self.count > 1 {
            self.items[1] = std::mem::replace(&mut self.items[self.count], T::default());
        }
        
        // 减少元素计数
        self.count -= 1;
        
        // 如果还有元素，执行下沉操作维护堆性质
        if self.count > 0 {
            self.heapify_down(1);
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