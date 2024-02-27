#![cfg(test)]
use crate::custom_linked_list::CustomLinkedList;

#[test]
fn test_add_one_item(){
    let mut list: CustomLinkedList<u32> = CustomLinkedList::new();
    list.add_first(10);
    assert_eq!(list.length(), 1);
    if let Some(head) = list.get_head(){
        if let Some(tail) = list.get_tail(){
            assert_eq!(head.borrow().to_string(), tail.borrow().to_string())
        }
    }
}

#[test]
fn test_get_head(){
    let mut list: CustomLinkedList<u32> = CustomLinkedList::new();
    list.add_first(4);
    list.add_first(5);
    list.add_first(6);
    assert_eq!(list.length(), 3);
    if let Some(head) = list.get_head(){
        assert_eq!(head.borrow().to_string(), "6")
    }
}

#[test]
fn test_get_tail(){
    let mut list: CustomLinkedList<u32> = CustomLinkedList::new();
    list.add_first(4);
    list.add_first(5);
    list.add_first(6);
    assert_eq!(list.length(), 3);
    if let Some(tail) = list.get_tail(){
        assert_eq!(tail.borrow().to_string(), "4")
    }
}

#[test]
fn test_add_first(){
    let mut list: CustomLinkedList<u32> = CustomLinkedList::new();
    list.add_first(4);
    list.add_first(5);
    list.add_first(6);
    assert_eq!(list.length(), 3);
    if let Some(node) = list.get_head(){
        assert_eq!(node.borrow().to_string(), "6")
    }
    if let Some(node) = list.get_tail(){
        assert_eq!(node.borrow().to_string(), "4")
    }
}


#[test]
fn test_add_last(){
    let mut list: CustomLinkedList<u32> = CustomLinkedList::new();
    list.add_last(4);
    list.add_last(5);
    list.add_last(6);
    assert_eq!(list.length(), 3);
    if let Some(node) = list.get_tail(){
        assert_eq!(node.borrow().to_string(), "6")
    }
    if let Some(node) = list.get_head(){
        assert_eq!(node.borrow().to_string(), "4")
    }
}

#[test]
fn test_remove_last(){
    let mut list: CustomLinkedList<u32> = CustomLinkedList::new();
    list.add_last(4);
    list.add_last(5);
    list.add_last(6);
    assert_eq!(list.length(), 3);
    list.remove_last();
    list.remove_last();
    assert_eq!(list.length(), 1);
    if let Some(head) = list.get_head(){
        if let Some(tail) = list.get_tail(){
            assert_eq!(head.borrow().to_string(), tail.borrow().to_string())
        }
    }
}


#[test]
fn test_remove_first(){
    let mut list: CustomLinkedList<u32> = CustomLinkedList::new();
    list.add_last(4);
    list.add_last(5);
    list.add_last(6);
    assert_eq!(list.length(), 3);
    list.remove_first();
    list.remove_first();
    assert_eq!(list.length(), 1);
    if let Some(head) = list.get_head(){
        if let Some(tail) = list.get_tail(){
            assert_eq!(head.borrow().to_string(), tail.borrow().to_string())
        }
    }
}