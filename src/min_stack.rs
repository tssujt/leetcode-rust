#[derive(Debug)]
struct MinStack {
    vec: Vec<i32>,
    min: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            vec: Vec::new(),
            min: i32::max_value(),
        }
    }

    fn push(&mut self, x: i32) {
        if x <= self.min {
            self.vec.push(self.min);
            self.min = x;
        }
        self.vec.push(x);
    }

    fn pop(&mut self) {
        if self.vec.pop().unwrap() == self.min {
            self.min = self.vec.pop().unwrap();
        }
    }

    fn top(&self) -> i32 {
        *self.vec.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_155() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}

