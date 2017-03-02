extern crate glutin;

mod primitives;
mod tree;
mod window;

struct DummyElement(i32);

impl tree::Element for DummyElement {
     fn print(&self) {
         println!("{}", self.0);
     }
}

fn main() {
    let window = window::WindowBuilder::new()
        .with_title("My new window!")
        .build();

    let mut tree = tree::ElementTree::new();
    tree.set_root(Some(DummyElement(0)));

    {
        let mut root = tree.root().unwrap();
        root.add_child(DummyElement(1));

        {
            let mut one = root.children()[0].clone();

            one.add_child(DummyElement(3));
        }
        root.add_child(DummyElement(2));
    }

    print_tree(&tree);
}

fn print_tree(tree: &tree::ElementTree) {
    if let Some(root) = tree.root() {
        print_node(root, 0);
    }
}

fn print_node(node: tree::ElementNodeRef, indent: u8) {
    for i in 0..indent {
        print!(" ");
    }

    node.element().print();

    for child in node.children().iter() {
        print_node(child.clone(), indent + 1);
    }
}