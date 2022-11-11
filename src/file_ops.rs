use std::{cell::RefCell, error::Error, fs, io, path::PathBuf, rc::Rc};

/// A struct that contains the file list as needed by the file_ops module.
///
/// # Examples
///
/// ```ignore
/// # mod photo_organizer::file_ops;
/// # use std::path::PathBuf;
/// let source = PathBuf::from("/home/user");
/// let file_list = match FileList::build(&source) {
///     Ok(list) => list,
///     Err(error) => panic!{"{}", error},
/// };
/// ```
pub struct FileList {
    list: Rc<RefCell<Vec<PathBuf>>>,
}

impl FileList {
    /// Reads the `source` directory and returns a Result which wraps the file list if Ok and an
    /// error message if Err.
    ///
    /// # Errors
    ///
    /// this constructor can fail if it fails to read the source directory structure.
    pub fn build(source: &PathBuf) -> Result<FileList, Box<dyn Error>> {
        fn build_list(list: Rc<RefCell<Vec<PathBuf>>>) -> io::Result<()> {
            let dir = list
                .borrow()
                .get(list.borrow().len() - 1)
                .expect("The program will error during the config phase if there are no directories in the list.")
                .to_path_buf();
            if dir.is_dir() {
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();

                    unsafe {
                        let list = list.as_ptr();
                        (*list).push(PathBuf::from(&path));
                    }

                    if path.is_dir() {
                        build_list(Rc::clone(&list))?;
                    }
                }
            }

            Ok(())
        }

        let list = Rc::new(RefCell::new(vec![PathBuf::from(source)]));

        build_list(Rc::clone(&list))?;

        Ok(FileList { list })
    }

    /// Creates the directory with the organized files.
    ///
    /// # Errors
    ///
    /// This function may result in an error in case:
    /// - The files don't contain valid UTF-8 data.
    /// - You don't have permissions to edit the destination.
    pub fn organize(
        self,
        override_present: bool,
        source: &str,
        destination: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut list = RefCell::borrow_mut(&self.list);

        for entry in list.iter_mut() {
            let mut s_entry = entry
                .to_str()
                .expect("The program will have already ended if there's invalid UTF-8")
                .replace(source, destination);

            let s_entry_chars = s_entry.chars().count();
            let destination_chars = destination.chars().count() + 5;

            if s_entry_chars > destination_chars {
                s_entry.insert(destination_chars, '/');
                _ = s_entry.remove(destination_chars + 3);
                _ = s_entry.remove(destination_chars + 3);
            }

            let extension = match entry.extension() {
                Some(extension) => extension
                    .to_str()
                    .expect("The program will have already ended if there's invalid UTF-8"),
                None => "",
            };
            s_entry = s_entry.replace(extension, &extension.to_lowercase());

            let from = entry.as_path();

            let to = PathBuf::from(&s_entry);

            if extension.is_empty() {
                fs::create_dir_all(to.as_path())?;
            } else if !to.exists() || override_present {
                fs::copy(from, to.as_path())?;
            }

            if s_entry.contains(".aae") {
                println!("{}", s_entry);
            }
        }

        Ok(())
    }
}
