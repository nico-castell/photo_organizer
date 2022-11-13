use std::{cell::RefCell, error::Error, fs, io, path::PathBuf, rc::Rc};

/// A struct that contains the file list as needed by the file_ops module.
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
        &self,
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

    /// Remove files present at DESTINATION but not SOURCE. Here SOURCE is `&self` and DESTINATION
    /// is `destination: &Self`.
    ///
    /// # Errors
    ///
    /// This method will result in an error if there are problems deleting the files, such as lack
    /// of permissions to do so.
    pub fn lean(&self, destination: &Self) -> Result<(), Box<dyn Error>> {
        let source_list = self.list.borrow();
        let destination_list = destination.list.borrow();

        let source_list: Vec<&PathBuf> = source_list.iter().filter(|file| !file.is_dir()).collect();
        let destination_list: Vec<&PathBuf> = destination_list
            .iter()
            .filter(|file| !file.is_dir())
            .collect();

        let mut index = 0;
        let mut offset = 0;

        while index < destination_list.len() {
            let source = source_list[index - offset]
                .file_name()
                .expect("The program was run using a path ending in `..`")
                .to_str()
                .expect("The program will have already ended if there's invalid UTF-8")
                .to_uppercase();
            let destination = destination_list[index]
                .file_name()
                .expect("The program was run using a path ending in `..`")
                .to_str()
                .expect("The program will have already ended if there's invalid UTF-8")
                .to_uppercase();

            if source != destination && destination_list[index].is_file() {
                fs::remove_file(destination_list[index])?;
                offset += 1;
            }
            index += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_list_builds() {
        use std::{fs, path::PathBuf};

        let source = PathBuf::from("mockSOURCE");

        fs::create_dir(&source).expect("File operation create failing.");
        let _ = FileList::build(&source).expect("Build function shouldn't fail.");
        fs::remove_dir(&source).expect("File operation remove failing.");
    }
}
