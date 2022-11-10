use std::path::Path;

mod file_ops;

pub struct Config {
    source: String,
    destination: String,
}

impl Config {
    /// Creates a `Config` type. Assumes the first iteration of `args` is the program name.
    ///
    /// # Errors
    ///
    /// The function can fail `args` does not contain source and destination paths.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let source = String::from(match args.next() {
            Some(file) => String::from(file),
            None => return Err("Didn't get a source directory"),
        });

        let destination = String::from(match args.next() {
            Some(file) => String::from(file),
            None => return Err("Didn't get a destination directory"),
        });

        Ok(Config {
            source,
            destination,
        })
    }
}

/// Runs the configured copy and organization of files.
///
/// # Errors
///
/// This function will return an error if:
/// - The source path does not exist.
/// - The destination path cannot be written to.
///
/// # Examples
///
/// ```
/// # use photo_organizer::*;
/// # let args = vec![
/// #     String::from("program_name"),
/// #     String::from("/"),
/// #     String::from("/"),
/// # ];
/// # let config = match Config::build(args.into_iter()) {
/// #     Ok(config) => config,
/// #     Err(error) => panic!("Failed to build `Config`: {}", error),
/// # };
/// if let Err(error) = run(config) {
///     panic!("Application error: {error}");
/// }
/// ```
pub fn run(config: Config) -> Result<(), &'static str> {
    // TODO: Implement proper error propagation.
    let (source, destination) = (config.source, config.destination);

    let source_path = Path::new(&source);
    let destination_path = Path::new(&destination);

    if !source_path.is_dir() {
        return Err("Source directory does not exist.");
    }

    if !destination_path.is_dir() {
        return Err("Destination directory does not exist.");
    }

    use file_ops::SourceTree;

    let dir_tree = match SourceTree::build(&source_path) {
        Ok(tree) => tree,
        Err(_) => return Err("Could not read source directory structure.")
    };

    let organized_dir_tree = dir_tree.organize(&source, &destination);

    if let Err(_) = organized_dir_tree.construct() {
        return Err("Error creating destination directory structure.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_builds() {
        let args = vec![
            String::from("program_name"),
            String::from("/path/to/source"),
            String::from("/path/to/destination"),
        ];
        let config = match Config::build(args.into_iter()) {
            Ok(config) => config,
            Err(error) => panic!("Failed to build `Config`: {}", error),
        };

        assert_eq!(String::from("/path/to/source"), config.source);
        assert_eq!(String::from("/path/to/destination"), config.destination);
    }
}
