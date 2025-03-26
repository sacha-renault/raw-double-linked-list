use raw_double_linked_list::List;

fn main() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.insert(1, 5);

    println!("{list:?}")
}
