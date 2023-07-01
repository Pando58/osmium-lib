use std::collections::HashMap;

pub trait AutoInc<V> {
    fn push(&mut self, value: V) -> usize;

    fn push_with_key(&mut self, f: impl Fn(usize) -> V) -> usize;
}

impl<V> AutoInc<V> for HashMap<usize, V> {
    fn push(&mut self, value: V) -> usize {
        let len = self.len();

        for i in 0..len {
            if !self.contains_key(&i) {
                self.insert(i, value);
                return i;
            }
        }

        self.insert(len, value);
        len
    }

    fn push_with_key(&mut self, f: impl Fn(usize) -> V) -> usize {
        let len = self.len();

        for i in 0..len {
            if !self.contains_key(&i) {
                self.insert(i, f(i));
                return i;
            }
        }

        self.insert(len, f(len));
        len
    }
}
