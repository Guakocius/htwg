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

trait AvlFn {
    fn insert(&mut self, key: i32, value: i32, node: &mut NodePtr) -> bool;
    fn pre_order(&self);
    fn in_order(&self);
    fn post_order(&self);
    fn search(&self, key: i32, value: i32, node: &NodePtr) -> bool;
    fn remove(&mut self, key: i32, node: &mut NodePtr) -> bool;
    fn search_min<'a>(&mut self, node: &'a mut NodePtr) -> &'a mut NodePtr;
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
    fn search(&self, key: i32, value: i32, node: &NodePtr) -> bool {
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
    }

    fn search_min<'a>(&mut self, p: &'a mut NodePtr) -> &'a mut NodePtr {
        let has_left = p.as_ref().map_or(false, |n| n.left.is_some());

        if has_left {
            if let Some(n) = p {
                 return self.search_min(&mut n.left);
            }
        }
        p
    }

    fn remove(&mut self, key: i32, node: &mut NodePtr) -> bool {
        if node.is_none() {
            return false;
        }

        let should_remove = if let Some(p) = node {
            p.key == key
        } else {
            false
        };

        if should_remove {
            if let Some(p) = node {
                if p.left.is_none() || p.right.is_none() {
                    let replacement = if p.left.is_some() {
                        p.left.take()
                    } else {
                        p.right.take()
                    };
                    *node = replacement;
                    return true;
                } else {
                    // Node has two children
                    let min = self.search_min(&mut p.right);
                    if let Some(min_node) = min.take() {
                        p.key = min_node.key;
                        p.value = min_node.value;
                        *min = min_node.right;
                    }
                    return true;
                }
            }
        }

        // Recurse left or right
        if let Some(p) = node {
            if key < p.key {
                return self.remove(key, &mut p.left);
            } else {
                return self.remove(key, &mut p.right);
            }
        }

        false
    }
}
