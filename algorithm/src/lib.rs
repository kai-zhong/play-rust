pub mod search {
    pub mod tree {
        use std::option::Option::{None, Some};

        pub struct BinarySearchTree<K, T> {
            root: OptionalNode<K, T>,
        }

        #[derive(Clone)]
        struct Node<K, T> {
            pub left: OptionalNode<K, T>,
            pub right: OptionalNode<K, T>,
            pub key: K,
            pub value: T,
        }

        type OptionalNode<K, T> = Option<Box<Node<K, T>>>;

        impl<K: PartialOrd + Clone, T:  Clone> BinarySearchTree<K, T> {

            pub fn new() -> BinarySearchTree<K, T> {
                BinarySearchTree {
                    root: None,
                }
            }

            pub fn new_with_kv(key: K, value: T) -> BinarySearchTree<K, T> {
                BinarySearchTree {
                    root: Some(Box::new(Node::new(key, value))),
                }
            }

            pub fn is_empty(&self) -> bool {
                match &self.root {
                    Some(_) => false,
                    None => true,
                }
            }

            pub fn search_recursively<'a>(&'a self, key: &'a K) -> Option<&'a T> {
                Node::search_recursively(&self.root, key)
            }

            pub fn search_iteratively<'a>(&'a self, key: &'a K) -> Option<&'a T> {
                Node::search_iteratively(&self.root, key)
            }

            pub fn insert_recursively(&mut self, key: K, value: T) {

                if let None = self.root {
                    self.root.replace(Box::new(Node::new(key, value)));
                } else {
                    Node::insert_recursively(&mut self.root, key, value);
                }
            }

            pub fn insert_iteratively(&mut self, key: K, value: T) {
                if let None = self.root {
                    self.root.replace(Box::new(Node::new(key, value)));
                } else {
                    Node::insert_iteratively(&mut self.root, key, value);
                }

            }

            pub fn delete(&mut self, key: &K) {
                self.root = Node::delete_recursively(self.root.take(), &key);
            }

            pub fn delete_v2(&mut self, key: &K) {
                Node::delete_recursively_v2(&mut self.root, &key);
            }
        }

        impl<K: PartialOrd + Clone, T: Clone> Node<K, T> {

            fn new(key: K, value: T) -> Node<K, T> {
                Node {
                    left: None,
                    right: None,
                    key,
                    value,
                }
            }

            fn insert_recursively(root: &mut OptionalNode<K, T>, key: K, value: T) {
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

            fn insert_iteratively(root: &mut OptionalNode<K, T>, key: K, value: T) {
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

            fn search_recursively<'a>(root: &'a OptionalNode<K, T>, key: &'a K) -> Option<&'a T> {
                match root {
                    Some(n) => {
                        if *key < (*n).key {
                            Node::search_recursively(&(*n).left, &key)
                        } else if *key > (*n).key {
                            Node::search_recursively(&(*n).right, &key)
                        } else {
                            Some(&(*n).value)
                        }
                    }
                    None => None
                }
            }

            fn search_iteratively<'a>(root: &'a OptionalNode<K, T>, key: &'a K) -> Option<&'a T> {

                let mut walk = root;

                while let Some(node) = walk {
                    if *key < (*node).key {
                        walk = &(*node).left;
                    } else if *key > (*node).key {
                        walk = &(*node).right;
                    } else {
                        return Some(&(*node).value)
                    }
                }

                None
            }

            fn delete_recursively(root: OptionalNode<K, T>, key: &K) -> OptionalNode<K, T> {
                match root {
                    Some(mut curr) => {
                        if *key < (*curr).key {
                            (*curr).left = Node::delete_recursively((*curr).left, key);
                            Some(curr)
                        } else if *key > (*curr).key {
                            (*curr).right = Node::delete_recursively((*curr).right, key);
                            Some(curr)
                        } else {

                            match ((*curr).left, (*curr).right) {
                                (None, None) => None,
                                (Some(child), None) => Some(child),
                                (None, Some(child)) => Some(child),
                                (Some(l), Some(r)) => {

                                    let left = Some(l);
                                    let right = Some(r);

                                    let successor = Node::find_inorder_successor(&right);

                                    let (key, value) = match successor {
                                        Some(node) =>  (node.key.clone(), node.value.clone()),
                                        None =>  (curr.key, curr.value),
                                    };

                                    let right = Node::delete_recursively(right, &key);

                                    Some(Box::new(Node {
                                        key, value,
                                        left, right,
                                    }))
                                }
                            }
                        }
                    },
                    None => None
                }
            }

            fn delete_recursively_v2(root: &mut OptionalNode<K, T>, key: &K) {
                if let Some(mut curr) = root.take() {

                    if *key < (*curr).key {
                        Node::delete_recursively_v2(&mut curr.left, key);
                        root.replace(curr);

                    } else if *key > (*curr).key {
                        Node::delete_recursively_v2(&mut curr.right, key);
                        root.replace(curr);
                    } else {

                        match (curr.left.take(), curr.right.take()) {
                            (None, None) => None,
                            (Some(child), None) => root.replace(child),
                            (None, Some(child)) => root.replace(child),
                            (Some(l), Some(r)) => {

                                let left = Some(l);
                                let mut right = Some(r);
                                let successor = Node::find_inorder_successor(&right);

                                if let Some(node) = successor {
                                    curr.key = node.key.clone();
                                    curr.value = node.value.clone();
                                }

                                Node::delete_recursively_v2(&mut right, &key);

                                curr.left = left;
                                curr.right = right;

                                root.replace(curr)
                            }
                        };
                    }

                }
            }

            fn find_inorder_successor(mut root: &OptionalNode<K, T>) -> &OptionalNode<K, T> {

                while let Some(node) = root {
                    if node.left.is_none() {
                        break;
                    }

                    root = &node.left;
                }

                root
            }

        }


        #[cfg(test)]
        mod test {
            use crate::search::tree::{BinarySearchTree, OptionalNode};
            use NodePath::*;

            enum NodePath { Left, Right }

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

                let node = &bst.root;
                assert!(validate_kv(&node, 1, 1));

                let node = get_node(&[Right], &bst.root);
                assert!(validate_kv(node, 2, 3));

                let node = get_node(&[Right, Right], &bst.root);
                assert!(validate_kv(node, 3, 3));
            }

            #[test]
            fn insert_iteratively() {
                let mut bst: BinarySearchTree<i32, i32> = BinarySearchTree::new();
                bst.insert_iteratively(234, 1);
                bst.insert_iteratively(-2, 3);
                bst.insert_iteratively(34, 5);
                bst.insert_iteratively(42, 2);

                let val = bst.search_iteratively(&3);
                assert_eq!(val, None);

                let val = bst.search_iteratively(&234);
                assert_eq!(val, Some(&1));

                let val = bst.search_iteratively(&-2);
                assert_eq!(val, Some(&3));
            }

            #[test]
            fn search_recursively() {
                let mut bst: BinarySearchTree<i32, i32> = BinarySearchTree::new();
                bst.insert_iteratively(10, 2);
                bst.insert_iteratively(11, 3);
                bst.insert_iteratively(3, 4);
                bst.insert_iteratively(9, 9);

                let val = bst.search_recursively(&3);
                assert_eq!(val, Some(&4));
                assert_ne!(val, Some(&7));

                let val = bst.search_recursively(&111);
                assert_eq!(val, None);
            }

            #[test]
            fn delete() {
                let mut bst: BinarySearchTree<i32, i32> = BinarySearchTree::new();
                bst.insert_iteratively(10, 2);
                bst.insert_iteratively(11, 3);
                bst.insert_iteratively(3, 4);
                bst.insert_iteratively(-1, 23);
                bst.insert_iteratively(9, 9);
                bst.insert_iteratively(-2, 434);
                bst.insert_iteratively(0, 123);

                bst.delete(&3);
                let val = bst.search_recursively(&3);
                assert_eq!(val, None);

                bst.delete_v2(&0);
                let val = bst.search_recursively(&0);
                assert_eq!(val, None);

                let val = bst.search_recursively(&-1);
                assert_eq!(val, Some(&23));
            }


            fn get_node<'a, K, T>(path: &'a [NodePath], root: &'a OptionalNode<K, T>) -> &'a OptionalNode<K, T> {

                let mut node = root;

                for child in path.iter() {
                    node = match (node, child) {
                        (Some(node), Left) => & (*node).left,
                        (Some(node), Right) => & (*node).right,
                        (None, _) => &None
                    }
                }

                node
            }

            fn validate_kv<K: PartialOrd, T: PartialOrd>(node: &OptionalNode<K, T>, key: K, value: T) -> bool {
                match node {
                    Some(n) => (*n).key == key && (*n).value == value,
                    None => false
                }
            }
        }
    }
}