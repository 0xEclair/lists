pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None
        }
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem: elem,
            next: self.head.take()
        });
        self.head = Some(node)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
            //cur = node.next;
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        // todo
    }
}