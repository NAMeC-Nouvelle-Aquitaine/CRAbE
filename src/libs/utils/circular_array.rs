use std::ops::{Index, IndexMut};

pub struct CircularArray<T, const S: usize> {
    array: [Option<T>; S],
    pub(crate) pos: usize,
}

impl<T, const S: usize> Default for CircularArray<T, S> {
    fn default() -> Self {
        CircularArray::new()
    }
}

impl<T, const S: usize> CircularArray<T, S> {
    const INIT: Option<T> = None;

    fn new() -> Self {
        let arr: [Option<T>; S] = [Self::INIT; S];

        Self { array: arr, pos: 0 }
    }
}

impl<T, const S: usize> CircularArray<T, S> {
    pub(crate) fn push(&mut self, value: T) {
        if self.pos == 0 {
            self.pos = S;
        }

        self.pos -= 1;
        self.array[self.pos] = Some(value);
    }

    fn size(&self) -> usize {
        S
    }
}

impl<T, const S: usize> Index<usize> for CircularArray<T, S> {
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < S);
        if self.pos + index < S {
            &self.array[self.pos + index]
        } else {
            &self.array[self.pos + index - S]
        }
    }
}

impl<T, const S: usize> IndexMut<usize> for CircularArray<T, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < S);
        if self.pos + index < S {
            &mut self.array[self.pos + index]
        } else {
            &mut self.array[self.pos + index - S]
        }
    }
}
