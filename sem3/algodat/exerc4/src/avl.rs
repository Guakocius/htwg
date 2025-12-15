#![allow(dead_code)]

type NodePtr = Option<Box<Node>>;

#[derive(Default)]
struct Node {
    key: i32,
    value: i32,
    height: i32,
    left: NodePtr,
    right: NodePtr,
}

pub struct AvlTree {
    root: NodePtr,
}



impl Node {
    pub fn new(k: i32, v: i32) -> Self {
        Node { key: k, value: v, height: 1, left: None, right: None }
    }
}

impl AvlTree {
    pub fn new() -> Self {
        AvlTree { root: None }
    }

    // Öffentliche API-Methoden
    pub fn insert(&mut self, key: i32, value: i32) -> bool {
        fn insert_rec(key: i32, value: i32, node: &mut NodePtr) -> bool {
            let inserted = match node {
                None => {
                    *node = Some(Box::new(Node::new(key, value)));
                    true
                }
                Some(p) => {
                    if key < p.key {
                        insert_rec(key, value, &mut p.left)
                    } else if key > p.key {
                        insert_rec(key, value, &mut p.right)
                    } else {
                        return false;
                    }
                }
            };

            if inserted {
                AvlTree::update_height(node);
                AvlTree::balance(node);
            }
            inserted
        }
        insert_rec(key, value, &mut self.root)
    }

    pub fn remove(&mut self, key: i32) -> bool {
        fn search_min_inline(p: &mut NodePtr) -> &mut NodePtr {
            let has_left = p.as_ref().map_or(false, |n| n.left.is_some());
            if has_left {
                if let Some(n) = p {
                    return search_min_inline(&mut n.left);
                }
            }
            p
        }

        fn remove_rec(key: i32, node: &mut NodePtr) -> bool {
            if node.is_none() {
                return false;
            }

            let should_remove = if let Some(p) = node {
                p.key == key
            } else {
                false
            };

            let removed = if should_remove {
                if let Some(p) = node {
                    if p.left.is_none() || p.right.is_none() {
                        let replacement = if p.left.is_some() {
                            p.left.take()
                        } else {
                            p.right.take()
                        };
                        *node = replacement;
                        true
                    } else {
                        // Node has two children
                        let min = search_min_inline(&mut p.right);
                        if let Some(min_node) = min.take() {
                            p.key = min_node.key;
                            p.value = min_node.value;
                            *min = min_node.right;
                        }
                        true
                    }
                } else {
                    false
                }
            } else {
                // Recurse left or right
                if let Some(p) = node {
                    if key < p.key {
                        remove_rec(key, &mut p.left)
                    } else {
                        remove_rec(key, &mut p.right)
                    }
                } else {
                    false
                }
            };

            if removed {
                AvlTree::update_height(node);
                AvlTree::balance(node);
            }
            removed
        }
        remove_rec(key, &mut self.root)
    }

    pub fn search(&self, key: i32, _value: i32) -> bool {
        fn search_rec(key: i32, node: &NodePtr) -> bool {
            match node {
                None => false,
                Some(p) => {
                    if key < p.key {
                        search_rec(key, &p.left)
                    } else if key > p.key {
                        search_rec(key, &p.right)
                    } else {
                        true
                    }
                }
            }
        }
        search_rec(key, &self.root)
    }

    pub fn pre_order(&self) {
        self.traverse(&self.root, &|v| println!("visiting {}", v), &|_| (), &|_| ());
    }

    pub fn in_order(&self) {
        self.traverse(&self.root, &|_| (), &|v| println!("visiting {}", v), &|_| ());
    }

    pub fn post_order(&self) {
        self.traverse(&self.root, &|_| (), &|_| (), &|v| println!("visiting {}", v));
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
}

impl AvlTree {
    fn search_min<'a>(&mut self, p: &'a mut NodePtr) -> &'a mut NodePtr {
        let has_left = p.as_ref().map_or(false, |n| n.left.is_some());

        if has_left {
            if let Some(n) = p {
                return self.search_min(&mut n.left);
            }
        }
        p
    }

    // AVL-spezifische Hilfsfunktionen
    fn height(node: &NodePtr) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn balance_factor(node: &NodePtr) -> i32 {
        node.as_ref().map_or(0, |n| {
            Self::height(&n.left) - Self::height(&n.right)
        })
    }

    fn update_height(node: &mut NodePtr) {
        if let Some(n) = node {
            let left_height = Self::height(&n.left);
            let right_height = Self::height(&n.right);
            n.height = 1 + left_height.max(right_height);
        }
    }

    fn rotate_right(node: &mut NodePtr) {
        if let Some(mut x) = node.take() {
            if let Some(mut y) = x.left.take() {
                x.left = y.right.take();
                let mut x_node = Some(x);
                Self::update_height(&mut x_node);
                y.right = x_node;
                let mut y_node = Some(y);
                Self::update_height(&mut y_node);
                *node = y_node;
            } else {
                *node = Some(x);
            }
        }
    }

    fn rotate_left(node: &mut NodePtr) {
        if let Some(mut x) = node.take() {
            if let Some(mut y) = x.right.take() {
                x.right = y.left.take();
                let mut x_node = Some(x);
                Self::update_height(&mut x_node);
                y.left = x_node;
                let mut y_node = Some(y);
                Self::update_height(&mut y_node);
                *node = y_node;
            } else {
                *node = Some(x);
            }
        }
    }

    fn balance(node: &mut NodePtr) {
        let bf = Self::balance_factor(node);

        if bf > 1 {
            // Left-heavy
            if let Some(n) = node {
                if Self::balance_factor(&n.left) < 0 {
                    // Left-Right Fall
                    Self::rotate_left(&mut n.left);
                }
            }
            // Left-Left Fall
            Self::rotate_right(node);
        } else if bf < -1 {
            // Right-heavy
            if let Some(n) = node {
                if Self::balance_factor(&n.right) > 0 {
                    // Right-Left Fall
                    Self::rotate_right(&mut n.right);
                }
            }
            // Right-Right Fall
            Self::rotate_left(node);
        }
    }
}
