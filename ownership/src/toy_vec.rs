pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>,
    len: usize,
    pos: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &T {
        match self.get(index) {
            Some(v) => v,
            None => default,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

    pub fn iter(& self) -> Iter<T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0
        }
    }
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    type Item = &'vec T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

impl <'vec, T: Default> IntoIterator for &'vec ToyVec<T> {
    type Item = &'vec T;
    type IntoIter = Iter<'vec, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


#[cfg(test)]
mod tests {
    use super::ToyVec;

    #[test]
    fn push_and_get() {
        let mut v = ToyVec::new();
        v.push("Java Finch".to_string());
        v.push("Budgerigar".to_string());
        let e = v.get(1);
        assert_eq!(e, Some(&"Budgerigar".to_string()));
    }

    #[test]
    fn push_and_pop() {
        let mut v = ToyVec::new();
        v.push("Java Finch".to_string());
        v.push("Budgerigar".to_string());
        let e1 = v.pop();
        assert_eq!(e1, Some("Budgerigar".to_string()));
        let e2 = v.pop();
        assert_eq!(e2, Some("Java Finch".to_string()));
        let e3 = v.pop();
        assert_eq!(e3, None);
    }

    #[test]
    fn iter() {
        let mut v = ToyVec::new();
        v.push("Java Finch".to_string());
        v.push("Budgerigar".to_string());

        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&"Java Finch".to_string()));
        assert_eq!(iter.next(), Some(&"Budgerigar".to_string()));
        assert_eq!(iter.next(), None);
        v.push("Canary".to_string()); // can compile
    }
}
