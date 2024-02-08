use single_linked_list::custom_linked_list::CustomLinkedList;

fn main() {
    let mut list: CustomLinkedList<u32> = CustomLinkedList::new();
    list.add_first(1);
    list .add_last(10);
    list.remove_first();
    list.add_first(2);
    list.remove_first();
    list.add_last(20);
    list.remove_first();
    list.remove_first();
    list.remove_first();
    println!("length of list: {}", list.length());
    dbg!(list);
}
