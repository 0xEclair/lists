pub struct List {
    head: Link
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> List {
        List {
            head: None
        }
    }

    pub fn push(&mut self, elem: i32) {
        let node = Box::new(Node {
            elem: elem,
            next: self.head.take()
        });
        self.head = Some(node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
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