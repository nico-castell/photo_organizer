use std::{
    cell::RefCell,
    fs, io,
    path::{Path, PathBuf},
    rc::Rc,
};

pub struct SourceTree {
    list: Rc<RefCell<Vec<PathBuf>>>,
}

impl SourceTree {
    pub fn build(source: &Path) -> Result<SourceTree, &'static str> {
        fn build_list(mut list: Rc<RefCell<Vec<PathBuf>>>) -> io::Result<()> {
            let dir = list
                .borrow()
                .get(list.borrow().len() - 1)
                .unwrap()
                .to_path_buf();
            if dir.is_dir() {
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();

                    unsafe {
                        let mut list = list.as_ptr();
                        (*list).push(PathBuf::from(&path));
                    }

                    if path.is_dir() {
                        build_list(Rc::clone(&list));
                    }
                }
            }

            Ok(())
        }

        let list = Rc::new(RefCell::new(vec![PathBuf::from(source)]));

        // TODO: Implement proper error propagation.
        if let Err(_) = build_list(Rc::clone(&list)) {
            return Err("Could not read source directory structure.");
        }

        Ok(SourceTree { list })
    }

    pub fn organize(self, source: &str, destination: &str) -> Result<(), &'static str> {
        let mut list = RefCell::borrow_mut(&self.list);

        for mut entry in list.iter_mut() {
            let mut s_entry = entry.to_str().unwrap().replace(&source, &destination);

            let s_entry_chars = s_entry.chars().count();
            let destination_chars = destination.chars().count() + 5;

            if s_entry_chars > destination_chars {
                s_entry.insert(destination_chars, '/');
                s_entry = s_entry.chars().filter(|char| char != &'_').collect();
            }

            if s_entry.contains(".AAE") {
                println!("{}", s_entry);
            }

            let extension = match &entry.extension() {
                Some(extension) => extension.to_str().unwrap(),
                None => "",
            };
            s_entry = s_entry.replace(&extension, &extension.to_lowercase());

            let from = entry.as_path().clone();

            let to = PathBuf::from(&s_entry);

            if extension == "" {
                fs::create_dir_all(to.as_path());
            }
            else
            {
                fs::copy(from, to.as_path());
            }
        }

        Ok(())
    }
}
