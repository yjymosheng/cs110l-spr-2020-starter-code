use std::fmt::{self, Display};
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

pub struct LinkedListIter<'a,T> {
    current: &'a Option<Box<Node<T>>>,
}


impl<'a, T> Iterator for LinkedListIter<'a,T> where T:Clone{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                // YOU FILL THIS IN!
                self.current = &node.next;
                Some(node.value.clone())

            },
            None => None// YOU FILL THIS IN!
        }
    }
}

impl<'a,T> IntoIterator for &'a LinkedList <T> where T:Clone{
    type Item = T;
    type IntoIter = LinkedListIter<'a,T>;
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter {current: &self.head}
    }
}


struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}


impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        LinkedList {
            head: self.head.clone(),
            size: self.size,
        }
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {
            value: self.value.clone(),
            next: self.next.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        return self.head.eq(&other.head);
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.value.ne(&other.value) {
            return false;
        }

        return self.next.eq(&other.next);
    }
}


impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl<T> fmt::Display for LinkedList<T>  where T : Display{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}



