#[derive(Debug)]
pub struct Stack {
    sp: usize,
    data: Vec<i32>
}

impl Stack {
    pub fn new(size: usize) -> Self {
        Self {
            sp: 0,
            data: vec![i32::MIN; size]
        }
    }

    pub fn push(&mut self, item: i32) {
        self.data[self.sp] = item;
    }

    pub fn pop(&mut self) -> i32 {
        let res = self.data[self.sp];
        self.data[self.sp] = i32::MIN;
        self.decrement_pointer();
        
        res
    }

    pub fn top(&self) -> i32 {
        self.data[self.sp]
    }

    pub fn increment_pointer(&mut self) {
        if self.sp < self.data.len() - 1 {
            self.sp += 1;
        }
    }

    pub fn decrement_pointer(&mut self) {
        if self.sp > 0 {
            self.sp -= 1;
        }
    }

    pub fn get_stack(&self) -> Vec<i32> {
        let mut debug_stack = self.data.clone();
        debug_stack = debug_stack.into_iter().filter(|item| *item != i32::MIN).collect();
        
        debug_stack
    }
}