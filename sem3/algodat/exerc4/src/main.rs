pub mod avl;

use crate::avl::AvlTree;

fn main() {
    let mut avl_tree = AvlTree::new();

    // Einfügen von Knoten
    avl_tree.insert(10, 10); 
    avl_tree.insert(5, 5);
    avl_tree.insert(20, 20);
    avl_tree.insert(3, 3);
    avl_tree.insert(9, 9);
    avl_tree.insert(15, 15);
    avl_tree.insert(25, 25);
    avl_tree.insert(4, 4);
    avl_tree.insert(13, 13);
    avl_tree.insert(18, 18);
    avl_tree.insert(26, 26);
    avl_tree.insert(17, 17);
    avl_tree.insert(19, 19);

    // Suchen
    println!("Suche 10: {}", avl_tree.search(10, 0)); // true
    println!("Suche 99: {}", avl_tree.search(99, 0)); // false
    avl_tree.remove(5);
    // Traversierung
    println!("\nIn-Order:");
    avl_tree.in_order();
    println!("\nPost-Order:");
    avl_tree.post_order();
    println!("\nPre-Order:");
    avl_tree.pre_order();

    // Löschen
    //avl_tree.remove(20);
    /*println!("\nNach Löschen von {}:", 200);    
    avl_tree.in_order();
    println!("\nNach Löschen von {}:", 100);
    //avl_tree.remove(10);
    avl_tree.in_order();*/
}
