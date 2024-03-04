use super::productions::ProductionType;

#[derive(Debug)]
pub struct StackItem {
    value: ProductionType,
    next: Option<Box<StackItem>>,
}

#[derive(Debug)]
pub struct Stack {
    top: Option<Box<StackItem>>,
    size: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            top: None,
            size: 0,
        }
    }

    pub fn push(&mut self, value: ProductionType) {
        let new_item = StackItem {
            value,
            next: self.top.take(),
        };
        self.top = Some(Box::new(new_item));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<ProductionType> {
        match self.top.take() {
            Some(item) => {
                self.top = item.next;
                self.size -= 1;
                Some(item.value)
            },
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}