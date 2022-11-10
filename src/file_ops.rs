use std::{
    cell::RefCell,
    error::Error,
    fs, io,
    path::{Path, PathBuf},
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    name: RefCell<PathBuf>,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub struct SourceTree {
    initial_node: Rc<Node>,
}

pub struct DestinationTree {
    initial_node: Rc<Node>,
}

impl SourceTree {
    pub fn build(source: &Path) -> Result<SourceTree, &'static str> {
        fn build_node_tree(node: Rc<Node>) -> io::Result<()> {
            let dir = node.name.borrow().to_path_buf();
            if dir.is_dir() {
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    let new_node = Rc::new(Node {
                        name: RefCell::new(PathBuf::from(&path)),
                        parent: RefCell::new(Rc::downgrade(&node)),
                        children: RefCell::new(vec![]),
                    });
                    node.children.borrow_mut().push(Rc::clone(&new_node));

                    if path.is_dir() {
                        build_node_tree(Rc::clone(&new_node))?;
                    }
                }
            }

            Ok(())
        }

        let initial_node = Rc::new(Node {
            name: RefCell::new(PathBuf::from(&source)),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        // TODO: Implement proper error propagation.
        if let Err(_) = build_node_tree(Rc::clone(&initial_node)) {
            return Err("Could not read source directory structure.");
        }

        Ok(SourceTree { initial_node })
    }

    pub fn organize(self) -> DestinationTree {
        DestinationTree {
            initial_node: Rc::new(Node {
                name: RefCell::new(PathBuf::new()),
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            }),
        }
    }
}

impl DestinationTree {
    pub fn construct(&self) -> io::Result<()> {
        Ok(())
    }
}
