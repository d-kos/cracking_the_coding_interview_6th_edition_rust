use std::cell::RefCell;
use std::cmp::max;
use std::fmt::{Debug, Display, Formatter};

use std::fmt;
use std::rc::Rc;

pub type Link<T> = Rc<RefCell<Node<T>>>;

/// A symbiote between a binary search tree and a binary tree.
/// It started as a bst before chapter 4 later I've just added functionalities as problems required.
/// Some methods have '_bt' suffix which is only for non BST
pub struct Tree<T> {
    pub root: Option<Link<T>>,
}

pub struct Node<T> {
    pub data: T,
    pub left: Option<Link<T>>,
    pub right: Option<Link<T>>,
}

impl<T> Tree<T>
where
    T: PartialOrd + Debug + Copy,
{
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            left: None,
            right: None,
        }));

        if let Some(ref r) = self.root {
            let mut current = r.clone();
            loop {
                // left
                if node.borrow().data < current.clone().borrow().data {
                    if current.borrow().left.is_some() {
                        let l = current.borrow_mut().left.as_mut().unwrap().clone();
                        current = l;
                    } else {
                        current.borrow_mut().left = Some(node.clone());
                        break;
                    }
                }
                // right
                else {
                    if current.borrow().right.is_some() {
                        let r = current.borrow_mut().right.as_mut().unwrap().clone();
                        current = r;
                    } else {
                        current.borrow_mut().right = Some(node.clone());
                        break;
                    }
                }
            }
        } else {
            self.root = Some(node);
        }
    }

    pub fn build_from(&mut self, node: Link<T>) {
        if self.root.is_some() {
            panic!("Root should be empty!")
        }
        self.root = Some(node)
    }

    pub fn in_order_traversal(&self) -> Vec<T> {
        let mut vec = vec![];
        if let Some(r) = &self.root {
            self.r_in_order_traversal(r, &mut vec);
        }

        vec
    }

    fn r_in_order_traversal(&self, node: &Link<T>, vec: &mut Vec<T>) {
        if let Some(ref left) = node.borrow().left {
            self.r_in_order_traversal(left, vec);
        }
        vec.push(node.borrow().data);
        if let Some(ref right) = node.borrow().right {
            self.r_in_order_traversal(right, vec);
        }
    }

    pub fn height(&self) -> usize {
        if let Some(r) = &self.root {
            self.r_height(r)
        } else {
            0
        }
    }

    pub fn r_height(&self, node: &Link<T>) -> usize {
        let (mut lh, mut rh) = (0, 0);
        if let Some(ref left) = node.borrow().left {
            lh = self.r_height(left);
        }
        if let Some(ref right) = node.borrow().right {
            rh = self.r_height(right);
        }

        max(lh, rh) + 1
    }

    pub fn min(&self) -> Option<T> {
        if let Some(r) = &self.root {
            return self.min_from(r);
        }

        None
    }

    pub fn min_from(&self, node: &Link<T>) -> Option<T> {
        if let Some(ref left) = node.borrow().left {
            return self.min_from(left);
        }

        Some(node.borrow().data)
    }

    pub fn find_node(&self, n_val: T) -> Option<Link<T>> {
        if let Some(r) = &self.root {
            return self.r_find_node(n_val, r);
        }

        None
    }

    pub fn r_find_node(&self, n_val: T, node: &Link<T>) -> Option<Link<T>> {
        if n_val == node.borrow().data {
            return Some(node.clone());
        }

        if n_val < node.borrow().data {
            if let Some(ref left) = node.borrow().left {
                return self.r_find_node(n_val, left);
            }
        } else {
            if let Some(ref right) = node.borrow().right {
                return self.r_find_node(n_val, right);
            }
        }
        None
    }

    pub fn find_node_bt(&self, n_val: T) -> Option<Link<T>> {
        if let Some(r) = &self.root {
            return self.r_find_node_bt(n_val, r);
        }

        None
    }

    pub fn r_find_node_bt(&self, n_val: T, node: &Link<T>) -> Option<Link<T>> {
        if n_val == node.borrow().data {
            return Some(node.clone());
        }

        let (mut l, mut r) = (None, None);
        if let Some(ref left) = node.borrow().left {
            l = self.r_find_node(n_val, left);
        }
        if let Some(ref right) = node.borrow().right {
            r = self.r_find_node(n_val, right);
        }

        if l.is_some() {
            l
        } else {
            r
        }
    }

    pub fn r_contains_node_bt(&self, n_val: T, node: &Link<T>) -> bool {
        if n_val == node.borrow().data {
            return true;
        }

        let (mut l, mut r) = (false, false);
        if let Some(ref left) = node.borrow().left {
            l = self.r_contains_node_bt(n_val, left);
        }
        if let Some(ref right) = node.borrow().right {
            r = self.r_contains_node_bt(n_val, right);
        }

        l || r
    }

    pub fn root(&self) -> Option<Link<T>> {
        self.root.clone()
    }

    pub fn is_subtree(&self, tree: Tree<T>) -> bool {
        if self.root.is_none() || tree.root().is_none() {
            return false;
        }

        let root_data = tree.root().unwrap().borrow().data;
        if let Some(root) = self.find_node(root_data) {
            self.r_is_subtree(&tree.root(), &Some(root))
        } else {
            false
        }
    }

    fn r_is_subtree(&self, this_root: &Option<Link<T>>, other_root: &Option<Link<T>>) -> bool {
        if this_root.is_none() && other_root.is_none() {
            return true;
        }

        if this_root.is_none() ^ other_root.is_none() {
            return false;
        }

        if this_root.as_ref().unwrap().borrow().data != other_root.as_ref().unwrap().borrow().data {
            return false;
        }

        self.r_is_subtree(
            &this_root.as_ref().unwrap().borrow().left,
            &other_root.as_ref().unwrap().borrow().left,
        ) && self.r_is_subtree(
            &this_root.as_ref().unwrap().borrow().right,
            &other_root.as_ref().unwrap().borrow().right,
        )
    }
}

#[allow(unused_must_use)]
impl<T: Display> Display for Tree<T>
where
    T: PartialOrd + Debug + Copy,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.in_order_traversal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_a_bst() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(4);
        tree.insert(6);
        tree.insert(2);

        let res = tree.in_order_traversal();
        assert_eq!(vec![2, 3, 4, 5, 6], res);
    }

    #[test]
    fn should_find_min() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(4);
        tree.insert(6);
        tree.insert(2);

        let res = tree.min();
        assert!(res.is_some());
        assert_eq!(2, res.unwrap());
    }

    #[test]
    fn should_find_node() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(4);
        tree.insert(6);
        tree.insert(2);

        let res = tree.find_node(3);
        assert!(res.is_some());
        assert_eq!(3, res.as_ref().unwrap().borrow().data);
        assert_eq!(
            2,
            res.as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .data
        );
        assert_eq!(
            4,
            res.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .data
        );
    }
}
