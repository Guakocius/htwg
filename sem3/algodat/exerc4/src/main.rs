pub mod avl;

use crate::avl::AvlTree;

fn main() {
    let mut avl_tree = AvlTree::new();

    // Einfügen von Knoten
    avl_tree.insert(10, 100);
    avl_tree.insert(20, 200);
    avl_tree.insert(5, 50);
    avl_tree.insert(15, 150);

    // Suchen
    println!("Suche 10: {}", avl_tree.search(10, 0)); // true
    println!("Suche 99: {}", avl_tree.search(99, 0)); // false

    // Traversierung
    println!("\nIn-Order:");
    avl_tree.in_order();
    println!("\nPost Order:");
    avl_tree.post_order();
    println!("\nPre Order:");
    avl_tree.pre_order();

    // Löschen
    avl_tree.remove(20);
    println!("\nNach Löschen von 20:");
    avl_tree.in_order();
}
