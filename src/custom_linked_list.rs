use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd)]
struct Node<T>{
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CustomLinkedList<T>{
    count: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>
}

impl<T> Node<T>{
    fn new(value: T) -> Self{
        Node { value, next: None }
    }
}
impl<T> CustomLinkedList<T>{

    pub fn new()-> Self{
        CustomLinkedList{
            count: 0,
            head: None,
            tail: None
        }
    }

    pub fn add_first(&mut self, value: T){
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        match self.head.take() {
            Some(prev_node)=>{
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

    pub fn add_last(&mut self, value: T){
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        match self.tail.take() {
            Some(prev_node)=>{
                prev_node.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
                self.count += 1;
            }
            None =>{
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
                self.count += 1;
            }
        }
    }

    pub fn remove_first(&mut self){
        match self.head.take(){
            Some(old_head)=>{
                match old_head.borrow_mut().next.take() {
                    Some(new_head)=>{
                        self.head = Some(new_head);
                        self.count -= 1;
                    }
                    None=>{
                        self.tail = None;
                        self.count -= 1;
                    }
                }
            }
            None =>{
                println!("can not remove from an empty list");
            }
        }
    }

    pub fn remove_last(&mut self){
        
    }  

    pub fn length(&self) -> usize{
        self.count
    }  

}


