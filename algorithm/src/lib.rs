pub mod search {
    pub mod tree {
        use std::option::Option::None;

        pub struct BinarySearchTree<K, T> {
            root: InnerNode<K, T>,
        }

        struct Node<K, T> {
            pub left: InnerNode<K, T>,
            pub right: InnerNode<K, T>,
            pub key: K,
            pub value: T,
        }

        type InnerNode<K, T> = Option<Box<Node<K, T>>>;

        impl<K: PartialOrd, T> BinarySearchTree<K, T> {

            pub fn new() -> BinarySearchTree<K, T> {
                BinarySearchTree {
                    root: None,
                }
            }

            pub fn new_with_kv(key: K, value: T) -> BinarySearchTree<K, T> {
                BinarySearchTree {
                    root: Option::from(Box::from(Node::new(key, value))),
                }
            }

            pub fn is_empty(&self) -> bool {
                match &self.root {
                    Some(_) => false,
                    None => true,
                }
            }

            pub fn insert_recursively(&mut self, key: K, value: T) {

                if let None = self.root {
                    self.root = Some(Box::new(Node::new(key, value)));
                } else {
                    Node::insert_recursively(&mut self.root, key, value);
                }
            }

            pub fn insert_iteratively(&mut self, key: K, value: T) {
                if let None = self.root {
                    self.root = Some(Box::new(Node::new(key, value)));
                } else {
                    Node::insert_iteratively(&mut self.root, key, value);
                }

            }
        }

        impl<K: PartialOrd, T> Node<K, T> {

            fn new(key: K, value: T) -> Node<K, T> {
                Node {
                    left: None,
                    right: None,
                    key,
                    value,
                }
            }

            fn insert_recursively(root: &mut InnerNode<K, T>, key: K, value: T) {
                if let Some(node) = root {
                    if key < node.key {
                        Node::insert_recursively(&mut node.left, key, value);
                    } else if key > node.key {
                        Node::insert_recursively(&mut node.right, key, value);
                    } else {
                        node.value = value;
                    }
                } else {
                    root.replace(Box::from(Node::new(key, value)));
                }
            }

            fn insert_iteratively(root: &mut InnerNode<K, T>, key: K, value: T) {
                let mut walk = root;

                while let Some(node) = walk {

                    if key < node.key {
                        walk = &mut (*node).left;
                    } else if key > node.key {
                        walk = &mut (*node).right;
                    } else {
                        node.value = value;
                        return;
                    }
                }

                walk.replace(Box::new(Node::new(key, value)));
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::tree::*;

        #[test]
        fn new_with_empty_tree() {
            let bst : BinarySearchTree<i32, i32> = BinarySearchTree::new();
            let is_empty = bst.is_empty();
            assert!(is_empty, "the result is empty: {}.", is_empty);
        }

        #[test]
        fn insert_recursively() {
            let mut bst: BinarySearchTree<i32, i32> = BinarySearchTree::new();
            bst.insert_recursively(1, 1);
            bst.insert_recursively(2, 3);
            bst.insert_recursively(3, 3);
            bst.insert_recursively(4, 3);

        }

        #[test]
        fn insert_iteratively() {
            let mut bst: BinarySearchTree<i32, i32> = BinarySearchTree::new();
            bst.insert_iteratively(1, 1);
            bst.insert_iteratively(2, 3);
            bst.insert_iteratively(3, 3);
            bst.insert_iteratively(4, 3);
        }
    }
}