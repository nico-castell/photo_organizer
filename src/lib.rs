use std::{error::Error, fmt, path::PathBuf, result};

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
    pub fn build(mut args: impl Iterator<Item = String>) -> result::Result<Config, &'static str> {
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
pub fn run(config: Config) -> result::Result<(), Box<dyn Error>> {
    let (source, destination) = (config.source, config.destination);

    let source_path = PathBuf::from(&source);
    let destination_path = PathBuf::from(&destination);

    if !source_path.is_dir() {
        return Err(RunError {
            message: "Source directory does not exist.",
        }
        .into());
    }

    if !destination_path.is_dir() {
        return Err(RunError {
            message: "Destination directory does not exist.",
        }
        .into());
    }

    use file_ops::FileList;

    let file_list = match FileList::build(&source_path) {
        Ok(list) => list,
        Err(error) => return Err(error.into()),
    };

    if let Err(error) = file_list.organize(&source, &destination) {
        return Err(error.into());
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct RunError {
    message: &'static str,
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RunError {}

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
