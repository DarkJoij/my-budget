use std::fmt::{Debug, Display, Formatter, Result};

// Strictly requires [`Clone`] trait.
pub struct Cont<T: Clone> {
    values: Vec<T>,
    index: usize
}

impl<T: Clone> Cont<T> {
    pub fn new(values: Vec<T>) -> Self {
        Cont { values, index: 0 }
    }

    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn current_ref(&self) -> &T {
        &self.values[self.index]
    }

    pub fn current(&self) -> T {
        let current = self.current_ref();
        current.clone()
    }

    pub fn next_ref(&mut self) -> &T {
        self.index = if self.index >= self.values.len() { 0 } else { self.index + 1 };
        self.current_ref()
    }

    pub fn next(&mut self) -> T {
        let next = self.next_ref();
        next.clone()
    }
}

impl<T: Clone + Debug> Debug for Cont<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.current().fmt(f)
    }
}

impl<T: Clone + Display> Display for Cont<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.current().fmt(f)
    }
}
