use raw_double_linked_list::prelude::*;

fn main() {
    let mut list = Queue::new();
    list.push(1);
    list.push(2);

    println!("{list:?}");
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
}
