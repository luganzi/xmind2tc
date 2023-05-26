pub struct Stack<T> {
    size: usize,
    pub(crate) data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, item: T) {
        self.size += 1;
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        self.data.get(self.size - 1)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            offset: 0,
            container: &self,
        }
    }
}

pub struct Iter<'a, T> {
    offset: usize,
    container: &'a Stack<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset > self.container.size - 1 {
            return None;
        }

        let result = self.container.data.get(self.offset);
        self.offset += 1;

        result
    }
}
