
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    stream: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut stream = BinaryHeap::new();

        for num in nums {
            stream.push(Reverse(num));

            if stream.len() > k as usize {
                stream.pop();
            }
        }

        Self {
            stream,
            k: k as usize,
        }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.stream.push(Reverse(val));

        if self.stream.len() > self.k {
            self.stream.pop();
        }

        self.stream.peek().unwrap().0
    }
}
