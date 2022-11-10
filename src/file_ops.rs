use std::{
    cell::RefCell,
    fs, io,
    path::{Path, PathBuf},
    rc::{Rc, Weak},
};

pub struct DirTree {
    initial_node: Rc<Node>,
}

pub struct OrganizedDirTree {
    initial_node: Rc<Node>,
}

impl DirTree {
    pub fn build(source: &Path) -> Result<DirTree, &'static str> {
        fn build_node_tree(node: Rc<Node>) -> io::Result<()> {
            let dir = node.name.borrow().to_path_buf();
            if dir.is_dir() {
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    let node = Rc::new(Node {
                        name: RefCell::new(PathBuf::from(&path)),
                        parent: RefCell::new(Rc::downgrade(&node)),
                        children: RefCell::new(vec![]),
                    });
                    node.parent
                        .borrow()
                        .upgrade()
                        .expect("The inner value should not be dropped by the preceding code.")
                        .children
                        .borrow_mut()
                        .push(Rc::clone(&node));

                    if path.is_dir() {
                        build_node_tree(Rc::clone(&node))?;
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

        Ok(DirTree { initial_node })
    }

    pub fn organize(self) -> OrganizedDirTree {
        OrganizedDirTree {
            initial_node: Rc::new(Node {
                name: RefCell::new(PathBuf::new()),
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            }),
        }
    }
}

impl OrganizedDirTree {
    pub fn construct(&self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
struct Node {
    name: RefCell<PathBuf>,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
