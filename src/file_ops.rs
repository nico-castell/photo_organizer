use std::{cell::RefCell, error::Error, fs, io, path::PathBuf, rc::Rc};

/// The list of files for organizing. Taken as an argument by the [`organize()`](organize) and
/// [`lean()`](lean) functions.
///
/// # Examples
/// You can create a `FileList` using `FileList::build()`:
///
/// ```
/// # use iphone_organizer::file_ops::FileList;
/// # use std::{path::PathBuf, fs};
/// # let path = PathBuf::from("mockPATH");
/// # fs::create_dir(&path);
/// let file_list = match FileList::build(&path) {
///     Ok(list) => list,
///     Err(error) => panic!("{error}"),
/// };
/// # fs::remove_dir(&path);
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

        {
            let list = Rc::new(RefCell::new(vec![PathBuf::from(source)]));

            build_list(Rc::clone(&list))?;

            Ok(FileList { list })
        }
    }
}

/// Creates the directory with the organized files.
///
/// Parameters
/// - `file_list` - A [`FileList`](FileList).
/// - `override_present` - A [`bool`](bool) that determines whether to skip files that are already
///   at destination or to override them.
/// - `source` - A source [`&str`](&str).
/// - `destination` - A destination [`&str`](&str).
///
/// # Errors
///
/// This function may result in an error in case:
/// - The files don't contain valid UTF-8 data.
/// - You don't have permissions to edit the destination.
pub fn organize(
    file_list: &FileList,
    override_present: bool,
    source: &str,
    destination: &str,
) -> Result<(), Box<dyn Error>> {
    let mut list = RefCell::borrow_mut(&file_list.list);

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

/// Removes files present at destination but not source.
///
/// Parameters:
/// - `destination` - A [`FileList`](FileList) of the destination.
/// - `source` -> [`FileList`](FileList) of the source.
///
/// # Errors
///
/// This method will result in an error if there are problems deleting the files, such as lack
/// of permissions to do so.
pub fn lean(destination: &FileList, source: &FileList) -> Result<(), Box<dyn Error>> {
    let source_list = source.list.borrow();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::{self, File},
        path::PathBuf,
    };

    // Using mockSOURCE_XX and mockDESTINATION_XX to avoid trouble during multithreaded testing.

    #[test]
    fn file_list_builds() {
        let source = PathBuf::from("mockSOURCE_01");

        fs::create_dir(&source).expect("File operation create failing.");
        let _ = FileList::build(&source).expect("Build function shouldn't fail.");
        fs::remove_dir(&source).expect("File operation remove failing.");
    }

    #[test]
    fn lean_function_works() {
        // Create the source file and directory structure.
        {
            fs::create_dir_all("mockSOURCE_02/202211__").expect("Could not create directories");
            fs::create_dir_all("mockSOURCE_02/202212__").expect("Could not create directories");

            File::create("mockSOURCE_02/202211__/IMG_8001.JPG").expect("Could not create files");
            // mockSOURCE_02/202211__/IMG_8002.JPG missing
            File::create("mockSOURCE_02/202212__/IMG_8003.JPG").expect("Could not create files");
            File::create("mockSOURCE_02/202212__/IMG_8004.JPG").expect("Could not create files");
        }

        // Crate the destination file and directory structure.
        {
            fs::create_dir_all("mockDESTINATION_02/2022/11").expect("Could not create directories");
            fs::create_dir_all("mockDESTINATION_02/2022/12").expect("Could not create directories");

            File::create("mockDESTINATION_02/2022/11/IMG_8001.jpg")
                .expect("Could not create files");
            File::create("mockDESTINATION_02/2022/11/IMG_8002.jpg")
                .expect("Could not create files");
            File::create("mockDESTINATION_02/2022/12/IMG_8003.jpg")
                .expect("Could not create files");
            File::create("mockDESTINATION_02/2022/12/IMG_8004.jpg")
                .expect("Could not create files");
        }

        // Run the `lean()` function
        {
            let mock_source = PathBuf::from("mockSOURCE_02");
            let mock_source = FileList::build(&mock_source).expect("Failed to build `FileList`s");

            let mock_destination = PathBuf::from("mockDESTINATION_02");
            let mock_destination =
                FileList::build(&mock_destination).expect("Failed to build `FileList`s");

            lean(&mock_destination, &mock_source).expect("`lean()` function panicked");
        }

        // Test that IMG_8002.jpg is no longer present
        {
            let test_file = PathBuf::from("mockDESTINATION_02/2022/11/IMG_8002.jpg");
            if test_file.exists() {
                panic!("The file that should have been deleted at DESTINATION is still present");
            }
        }

        // Clean up
        {
            fs::remove_dir_all("mockSOURCE_02").expect("Failed to remove mock directories");
            fs::remove_dir_all("mockDESTINATION_02").expect("Failed to remove mock directories");
        }
    }

    #[test]
    fn organize_function_works() {
        // Create the source file and directory structure.
        {
            fs::create_dir_all("mockSOURCE_03/202211__").expect("Could not create directories");
            fs::create_dir_all("mockSOURCE_03/202212__").expect("Could not create directories");

            File::create("mockSOURCE_03/202211__/IMG_8001.JPG").expect("Could not create files");
            File::create("mockSOURCE_03/202211__/IMG_8002.JPG").expect("Could not create files");
            File::create("mockSOURCE_03/202212__/IMG_8003.JPG").expect("Could not create files");
            File::create("mockSOURCE_03/202212__/IMG_8004.JPG").expect("Could not create files");
        }

        // Run the `organize()` function
        {
            let mock_source = PathBuf::from("mockSOURCE_03");
            let mock_source = FileList::build(&mock_source).expect("Failed to build FileList");

            organize(&mock_source, false, "mockSOURCE_03", "mockDESTINATION_03")
                .expect("`organize()` function panicked");
        }

        // Test that all files and directories are properly organized
        {
            let file_list = vec![
                PathBuf::from("mockDESTINATION_03/2022/11/IMG_8001.jpg"),
                PathBuf::from("mockDESTINATION_03/2022/11/IMG_8002.jpg"),
                PathBuf::from("mockDESTINATION_03/2022/12/IMG_8003.jpg"),
                PathBuf::from("mockDESTINATION_03/2022/12/IMG_8004.jpg"),
            ];

            for file in file_list {
                if !file.exists() {
                    panic!("File {} was not copied", file.display());
                }
            }
        }

        // Clean up
        {
            fs::remove_dir_all("mockSOURCE_03").expect("Failed to remove mock directories");
            fs::remove_dir_all("mockDESTINATION_03").expect("Failed to remove mock directories");
        }
    }
}
