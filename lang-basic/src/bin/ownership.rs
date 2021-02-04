struct Node {
    l: OptionalNode,
    r: OptionalNode,
    data: i32
}

impl Node {
    fn new(data: i32) -> Node {
        Node {
            l: None,
            r: None,
            data
        }
    }
}

type OptionalNode = Option<Box<Node>>;

fn delete(root: OptionalNode, key: i32) -> OptionalNode {
    match root {
        Some(node) => {

            match (node.l, node.r) {
                (None, None) => None,
                (Some(child), None) => Some(child),
                (None, Some(child)) => Some(child),
                (Some(l), Some(r)) => {
                    let successor= find_min(&r);
                    let data = successor.data;

                    Some(Box::new(Node {
                        l: Some(l),
                        r: delete(Some(r), data),
                        data
                    }))
                }
            }
        },
        None => None
    }
}

fn find_min(root: &Node) -> &Node {
    let mut walk: Option<&Node> = Some(&root);
    let mut min: Option<&Node> = None;
    while let Some(node) = walk {
        min = Some(node);
        walk = min;
    }

    &min.unwrap()
}

fn main() {

    let mut root = Some(Box::new (Node{
        l: Some(Box::new(Node::new(1))),
        r: Some(Box::new(Node::new(2))),
        data: 1
    }));

    root = delete(root, 1)
}
