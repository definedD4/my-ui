mod tree;

struct DummyElement(i32);

impl tree::Element for DummyElement {
     fn print(&self) {
         println!("{}", self.0);
     }
}

fn main() {
    let mut tree = tree::ElementTree::new();

    tree.set_root(Some(DummyElement(0)));

    tree.root().unwrap().element().print();
}
