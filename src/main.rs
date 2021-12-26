use std::rc::Rc;
use lists::{second};

struct A {

}
impl A {
    #[warn(dead_code)]
    fn mutable(&mut self) {

    }
}
impl Drop for A {
    fn drop(&mut self) {
        println!("dropping A");
    }
}

fn main() {
    let mut t = second::List::new();

    t.push(5);
    t.push(4);
    t.pop();
    t.peek();
    t.push(3);
    let mut list = second::List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let r = Rc::new(A{});
    // because of no DerefMut in rc.
    //let mut cur = r.take();

    let mut r = Box::new(Some(A{}));
    let cur = r.take();

    use std::cell::RefCell;
    let c = RefCell::new(5);


    let borrowed_mut = c.borrow_mut();
    let borrowed_mut2 = c.borrow_mut();
    let borrowed_five = c.borrow();
    let borrowed_five2 = c.borrow();
}
