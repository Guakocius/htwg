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
    fn pre_order(&mut self, node: &mut NodePtr) -> !;
    fn in_order(&mut self, node: &mut NodePtr) -> !;
    fn post_order(&mut self, node: &mut NodePtr) -> !;
    fn search(&mut self, key: i32, value: i32, node: &mut NodePtr) -> bool;
}

impl Node {
    pub fn new(k: i32, v: i32) -> Self {
        Node { key: k, value: v, left: None, right: None};
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
                *node = Some(Box::new(Node::));
                node
            }
        }
    }
    fn pre_order(&mut self, node: &mut NodePtr) -> ! {

    }
    fn in_order(&mut self, node: &mut NodePtr) -> ! {

    }
    fn post_order(&mut self, node: &mut NodePtr) -> ! {

    }
    fn search(&mut self, key: i32, value: i32, node: &mut NodePtr) -> bool {
        return if node.is_none() { false } else if 
        key < node.key { self::search(key, value, node.left) } else if key > node.key { search(key, value, node.right) } else { value = node.value; return true; }

    }
    fn remove(key: i32, node: NodePtr) -> bool {

    }
}
