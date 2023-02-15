pub mod linked_lists;

use linked_lists::single::simple::SimpleLinkedList;

fn test_simple_single_linked_list() {
    let mut root: SimpleLinkedList<i32> = SimpleLinkedList::new();
    for n in 0..10000 {
        root.push(n);
    }
    for x in root.iter() {
        println!("{}", x);
    }
}

fn main() {
    test_simple_single_linked_list();
}
