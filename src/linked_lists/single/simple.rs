#[derive(Debug)]
pub struct SimpleNode<T> {
    value: T,
    next: Option<Box<SimpleNode<T>>>,
}

impl<T> SimpleNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: value,
            next: None,
        }
    }

    pub fn get_value(&self) -> &T {
        return &self.value;
    }

    pub fn get_next(&self) -> Option<&Box<SimpleNode<T>>> {
        return self.next.as_ref();
    }

    pub fn get_next_mut(&mut self) -> Option<&mut Box<SimpleNode<T>>> {
        return self.next.as_mut();
    }

    pub fn has_next(&self) -> bool {
        return self.next.is_some();
    }

    pub fn add_next(&mut self, node_value: T) {
        self.next = Some(Box::new(SimpleNode::new(node_value)));
    }
}

pub struct SimpleNodeIter<'a, T> {
    next: Option<&'a SimpleNode<T>>,
}

impl<'a, T> Iterator for SimpleNodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(node) => {
                match node.get_next() {
                    Some(child) => self.next = Some(child),
                    None => {
                        self.next = None;
                    }
                }
                Some(node.get_value())
            }
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    root: Option<SimpleNode<T>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn get_root(&self) -> Option<&SimpleNode<T>> {
        return self.root.as_ref();
    }

    pub fn push(&mut self, node_value: T) {
        if self.root.is_none() {
            self.root = Some(SimpleNode::new(node_value));
        } else {
            let mut node = self.root.as_mut().unwrap();
            while node.has_next() {
                node = node.get_next_mut().as_mut().unwrap();
            }
            node.add_next(node_value);
        }
    }

    pub fn iter(&self) -> SimpleNodeIter<'_, T> {
        match self.get_root() {
            Some(root) => SimpleNodeIter { next: self.get_root() },
            None => SimpleNodeIter { next: None }
        }
    }
}
