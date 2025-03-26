use raw_double_linked_list::List;

fn main() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);

    for elt in list.into_iter() {
        println!("{elt}");
    }
}
