#[allow(dead_code)]

type NodePtr = Option<Box<Node>>;

#[derive(Default)]
struct Node {
    key: i32,
    value: i32,
    height: i32,
    left: NodePtr,
    right: NodePtr,
}

impl Node {
    pub fn new(k: i32, v: i32) -> Self {
        Node { key: k, value: v, height: 1, left: None, right: None }
    }
}

fn swap(a: mut i32, b: mut i32) {
    let temp: i32 = &a;
    a = b;
    b = temp;
}

fn heapify_down(a: &mut [Node], i: i32) {
    let mut max: i32 = i;
    let li: i32 = 2 * i;
    let re: i32 = li + 1;
    if a[li].key > a[max].key {
        max = li;
    }
    if a[re].key > a[max].key {
        max = re;
    }
    if i != max {
        swap(a[i], a[max]);
        heapify_down(a, max);
    }
}

fn build_heap(a: &mut [Node]) {
    for i in (0..a.len() / 2).rev() {
        heapify_down(a, i);
    }
}

fn heap_sort(a: &mut [Node]) {
    build_heap(a);
    for i in (2..a.len()).rev() {
        swap(a[1], a[i]);
        decrease_heap_size(a);
        heapify_down(a, 1);
    }
}

fn main() {

}