// Base linked list implementation for all chapter 2 problems.
// Each problem adds a decorator trait.
//
// Implementation based on resources ->
// https://doc.rust-lang.org/1.30.0/book/second-edition/ch15-05-interior-mutability.html#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt
// https://rust-unofficial.github.io/too-many-lists/fourth-final.html

use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::rc::Rc;

pub type Link<T> = Option<RcLink<T>>;
pub type RcLink<T> = Rc<RefCell<Node<T>>>;

pub trait List<T>
where
    T: PartialEq,
{
    fn add(&mut self, e: T);
    fn remove(&mut self, e: T);
    fn into_iter(&mut self) -> IntoIter<T>;
    fn into_rev_iter(&mut self) -> IntoIter<T>;
    fn size(&self) -> u32;
}

pub trait SinglyLinkedReferenceExtension<T>: List<T>
where
    T: PartialEq,
{
    fn add_node(&mut self, node: Link<T>);
}

pub struct Node<T> {
    pub e: T,
    pub next: Link<T>,
    pub prev: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
    pub tail: Link<T>,
    pub size: u32,
}

pub struct IntoIter<T> {
    next: Option<RcLink<T>>,
}

impl<T> Node<T> {
    pub fn new(e: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            e,
            prev: None,
            next: None,
        }))
    }

    pub fn new_(e: T, next: Link<T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            e,
            prev: None,
            next,
        }))
    }

    pub fn unlink(&mut self, list: &mut LinkedList<T>) {
        if self.is_only_element() {
            list.head = None;
            list.tail = None;
        } else if self.is_first() {
            list.head = self.next.clone();
        } else {
            if self.is_last() {
                list.tail = self.prev.clone();
            }

            // unwrap should be safe here
            self.prev.take().unwrap().borrow_mut().next = self.next.clone();
        }

        list.size -= 1;
    }

    fn is_first(&self) -> bool {
        self.prev.is_none() && self.next.is_some()
    }

    fn is_last(&self) -> bool {
        self.prev.is_some() && self.next.is_none()
    }

    fn is_only_element(&self) -> bool {
        self.prev.is_none() && self.next.is_none()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }
}

impl<T> List<T> for LinkedList<T>
where
    T: PartialEq,
{
    fn add(&mut self, e: T) {
        let new_node = Node::new(e);
        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(tail);

                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.size += 1;
    }

    fn remove(&mut self, e: T) {
        for node in self.into_iter() {
            if node.borrow().e == e {
                node.borrow_mut().unlink(self);
                break; // remove first
            }
        }
    }

    fn into_iter(&mut self) -> IntoIter<T> {
        IntoIter {
            next: self.head.clone(),
        }
    }

    fn into_rev_iter(&mut self) -> IntoIter<T> {
        IntoIter {
            next: self.tail.clone(),
        }
    }

    fn size(&self) -> u32 {
        self.size
    }
}

#[allow(unused_must_use)]
impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.head.clone();
        write!(f, "{}", "[");
        while current.is_some() {
            write!(f, "{}", current.clone().unwrap().borrow().e);
            current = current.unwrap().borrow_mut().next.clone();

            // current is next here - print element delimiter
            if current.is_some() {
                write!(f, "{}", ", ");
            }
        }
        write!(f, "{}", "]")
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = RcLink<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(current) => {
                self.next = current.borrow().next.clone();
                Some(current)
            }
            _ => None,
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(current) => {
                self.next = current.borrow().prev.clone();
                Some(current)
            }
            _ => None,
        }
    }
}

impl<A> FromIterator<A> for LinkedList<A>
where
    A: PartialEq,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut list = LinkedList::new();
        for i in iter {
            list.add(i);
        }

        list
    }
}

impl<T> SinglyLinkedReferenceExtension<T> for LinkedList<T>
where
    T: PartialEq,
{
    fn add_node(&mut self, link: Link<T>) {
        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = link;
            }
            _ => panic!("Doesn't support empty!"),
        }

        // Since it's a singly linked list I'll not fix tail reference here or prev reference.
        // Next is the only reference that should be used - tail in add_node is an exclusion
        // I don't feel like rewriting the whole thing as singly linked just for this 2_7 and 2_8 problems =)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_correct_size() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);

        assert_eq!(list.size, 3);
    }

    #[test]
    fn should_format_list() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);

        assert_eq!(format!("{}", list), "[1, 2, 3]");
    }

    #[test]
    fn should_remove_head() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);

        list.remove(1);

        assert_eq!(format!("{}", list), "[2, 3]");
        assert_eq!(list.head.as_ref().unwrap().borrow().e, 2);
        assert_eq!(list.tail.as_ref().unwrap().borrow().e, 3);
        assert_eq!(list.size, 2);
    }

    #[test]
    fn should_remove_tail() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);

        list.remove(3);

        assert_eq!(format!("{}", list), "[1, 2]");
        assert_eq!(list.head.as_ref().unwrap().borrow().e, 1);
        assert_eq!(list.tail.as_ref().unwrap().borrow().e, 2);
        assert_eq!(list.size, 2);
    }

    #[test]
    fn should_remove_middle() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);

        list.remove(2);

        assert_eq!(format!("{}", list), "[1, 3]");
        assert_eq!(list.head.as_ref().unwrap().borrow().e, 1);
        assert_eq!(list.tail.as_ref().unwrap().borrow().e, 3);
        assert_eq!(list.size, 2);
    }

    #[test]
    fn should_iter_from_tail() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);

        let mut rev_iter = list.into_rev_iter();

        assert_eq!(rev_iter.next_back().as_ref().unwrap().borrow().e, 3);
        assert_eq!(rev_iter.next_back().as_ref().unwrap().borrow().e, 2);
        assert_eq!(rev_iter.next_back().as_ref().unwrap().borrow().e, 1);
        assert!(rev_iter.next_back().take().is_none());
    }
}
