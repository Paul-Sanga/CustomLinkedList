use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: ToString> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }

    fn get_value_as_string(&self) -> String {
        self.value.to_string()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node: {}", self.value)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CustomLinkedList<T> {
    count: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: std::fmt::Display> std::fmt::Display for CustomLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_node = self.head.clone();
        while let Some(node) = current_node.clone() {
            write!(f, "{}, ", node.borrow())?;
            current_node = node.borrow().next.clone();
        }
        Ok(())
    }
}

impl<T: std::fmt::Display> CustomLinkedList<T> {
    pub fn new() -> Self {
        CustomLinkedList {
            count: 0,
            head: None,
            tail: None,
        }
    }

    pub fn add_first(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        match self.head.take() {
            Some(prev_node) => {
                new_node.borrow_mut().next = Some(prev_node);
                self.head = Some(new_node);
                self.count += 1;
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
                self.count += 1;
            }
        }
    }

    pub fn add_last(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        match self.tail.take() {
            Some(prev_node) => {
                prev_node.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
                self.count += 1;
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
                self.count += 1;
            }
        }
    }

    pub fn remove_first(&mut self) {
        match self.head.take() {
            Some(old_head) => match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    self.head = Some(new_head);
                    self.count -= 1;
                }
                None => {
                    self.tail = None;
                    self.count -= 1;
                }
            },
            None => {
                println!("can not remove from an empty list");
            }
        }
    }

    pub fn remove_last(&mut self) {
        let mut current_node = self.head.clone();
        let mut prev_node: Option<Rc<RefCell<Node<T>>>> = None;

        if self.count == 0 {
            panic!("can not remove from an empty list");
        }
        if self.count == 1 {
            self.tail = None;
            self.head = None;
            self.count -= 1;
        } else {
            while let Some(node) = current_node.clone() {
                if node.borrow().next.is_some() {
                    prev_node = Some(Rc::clone(&node));
                    current_node = node.borrow().next.clone();
                } else {
                    break;
                }
            }
            if let Some(node) = prev_node.clone() {
                node.borrow_mut().next = None;
                self.tail = Some(Rc::clone(&node));
                self.count -= 1;
            }
        }
    }

    pub fn print_list(&self) {
        let mut output_string = String::from("[");
        let mut current_node = self.head.clone();
        while let Some(node) = current_node.clone() {
            let value = node.borrow().get_value_as_string();
            if node.borrow().next.is_some() {
                output_string.push_str(&value);
                output_string.push(',');
                output_string.push_str(" ");
                current_node = node.borrow().next.clone();
            } else {
                output_string.push_str(&value);
                output_string.push(']');
                break;
            }
        }
        println!("{output_string}");
    }

    pub fn length(&self) -> usize {
        self.count
    }
}
