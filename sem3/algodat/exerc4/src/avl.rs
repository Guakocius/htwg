#![allow(dead_code)]

type NodePtr = Option<Box<Node>>;

#[derive(Default)]
struct Node {
    key: i32,
    value: i32,
    left: NodePtr,
    right: NodePtr,
}

pub struct AvlTree {
    root: NodePtr,
}

pub trait AvlFn {
    fn insert(&mut self, key: i32, value: i32, node: &mut NodePtr) -> bool;
    fn pre_order(&self);
    fn in_order(&self);
    fn post_order(&self);
    fn search(&self, key: i32, value: i32, node: &NodePtr) -> bool;
    fn remove(&self, key: i32, node: &NodePtr) -> bool;
    fn traverse<F1, F2, F3>(&self, node: &NodePtr, pre: &F1, mid: &F2, post: &F3)
        where
            F1: Fn(i32),
            F2: Fn(i32),
            F3: Fn(i32);
}

impl Node {
    pub fn new(k: i32, v: i32) -> Self {
        Node { key: k, value: v, left: None, right: None }
    }
}

impl AvlTree {
    pub fn new() -> Self {
        AvlTree { root: None }
    }
}

impl AvlFn for AvlTree {
    fn insert(&mut self, key: i32, value: i32, node: &mut NodePtr) -> bool {
        match node {
            None => {
                *node = Some(Box::new(Node::new(key, value)));
                return true;
            }
            Some(p) => {
                if key < p.key {
                    return self.insert(key, value, &mut p.left);
                } else if key > p.key {
                    return self.insert(key, value, &mut p.right);
                } else {
                    return false;
                }
            }
        }
    }

    fn traverse<F1, F2, F3>(&self, node: &NodePtr, pre: &F1, mid: &F2, post: &F3)
    where
        F1: Fn(i32),
        F2: Fn(i32),
        F3: Fn(i32),
        {
            if let Some(n) = node {
                pre(n.value);
                self.traverse(&n.left, pre, mid, post);
                mid(n.value);
                self.traverse(&n.right, pre, mid, post);
                post(n.value);
            }
        }

    fn pre_order(&self) {
        self.traverse(&self.root, &|v| println!("visiting {}", v), &|_| (), &|_| ());

    } 

    fn in_order(&self) {
        self.traverse(&self.root, &|_| (), &|v| println!("visiting {}", v), &|_| ());

    }
    fn post_order(&self) {
        self.traverse(&self.root, &|_| (), &|_| (), &|v| println!("visiting {}", v));

    }
    fn search(&self, key: i32, mut value: i32, node: &NodePtr) -> bool {
        match node {
            None => {
                return false;
            }
            Some(p) => {
                if key < p.key {
                    return self.search(key, value, &p.left);
                } else if key > p.key {
                    return self.search(key, value, &p.right);
                } else {
                    return true;
                }
            }
        }
        //return if node.is_none() { false } else if 
        //key < node.key { self::search(key, value, node.left) } else if key > node.key { search(key, value, node.right) } else { value = node.value; return true; }
    }
    fn remove(&self, key: i32, node: &mut NodePtr) -> bool {
        let mut tmp: &mut NodePtr = node;
        match node {
            None => {
                return false;
            } Some(_) => {
                while let Some(p) = tmp {
                if key < p.key {
                    return self.remove(key, &p.left);    
                } else if key > p.key {
                    return self.remove(key, &p.right);
                } else if p.left.is_none() || p.right.is_none() {
                    if !p.left.is_none() {
                        tmp = &mut p.left;
                    } else {
                        tmp = &mut p.right;
                    }
                    return false;
                }
            }

            } 
            
        }
        return false;
    }
}
