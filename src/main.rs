use single_linked_list::custom_linked_list::CustomLinkedList;

fn main() {
    let mut list: CustomLinkedList<u32> = CustomLinkedList::new();
    list.add_last(10);
    list.add_last(30);
    list.add_first(1);
    list.add_first(0);
    list.print_list();
    list.remove_first();
    list.print_list();
    println!("length of list: {}", list.length());
}
