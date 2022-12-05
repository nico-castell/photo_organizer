#![allow(deprecated)]

use std::{error::Error, fmt, path::PathBuf, result};

/// Module for file operations such as copying and removing.
pub mod file_ops;
pub use file_ops::{lean, organize, FileList};

/// The config for the organization of the iPhone images. Taken as an argument by the [`run`](run)
/// function.
///
/// You can create an instance of this struct using `Config::build()`.
#[deprecated(since="1.2.4", note="Please use `FileList` directly.")]
pub struct Config {
    source: String,
    destination: String,
    override_present: bool,
    lean: bool,
}

impl Config {
    /// Creates a `Config` type. **Assumes** the first iteration of `args` is the program name, so
    /// it's ignored.
    ///
    /// Parameter:
    /// - `args` - An iterator, meant to iterate over the binary's arguments and flags.
    ///
    /// # Errors
    ///
    /// The function can fail if `args` does not contain source and destination paths, or if one of
    /// the flags isn't recognized.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        args.next();

        let source = match args.next() {
            Some(file) => file,
            None => return Err("Didn't get a source directory".into()),
        };

        let destination = match args.next() {
            Some(file) => file,
            None => return Err("Didn't get a destination directory".into()),
        };

        let mut override_present = false;
        let mut lean = false;

        for arg in args {
            let arg = arg.as_str();
            match arg {
                "-o" | "--override" => override_present = true,
                "-s" | "--skip" => override_present = false,
                "-l" | "--lean" => lean = true,
                other => return Err(format!("Unknown option: `{}`", other).into()),
            }
        }

        Ok(Config {
            source,
            destination,
            override_present,
            lean,
        })
    }

    /// Prints the configuration options to stderr.
    ///
    /// # Example
    ///
    /// ```
    /// # use iphone_organizer::*;
    /// Config::print_config()
    /// ```
    pub fn print_config() {
        eprint!(
            "\
Usage:
      \x1B[01m{} SOURCE DESTINATION [OPTIONS]\x1B[00m\n
Options:
   -s | --skip     ) Skips all files that are already present at DESTINATION.
                     This is the default.\n
   -o | --override ) Replaces files already present at DESTINATION with the
                     version from SOURCE.\n
   -l | --lean     ) Remove files present at DESTINATION but not SOURCE.\n
   -h | --help     ) Prints this help information.\n
Version: {}, {} License
",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_LICENSE")
        );
    }
}

/// Runs the configured copy and organization of files.
///
/// Parameter:
/// - `config` - A [`Config`](Config) that contains the configuration for the function.
///
/// # Errors
///
/// This function will return an error if:
/// - The source path does not exist or isn't a directory.
/// - The files contain invalid UTF-8 names.
/// - The directory structures of source or destination fail to be read, because of lack of
///   permissions or otherwise.
/// - The files cannot be copied, read, or deleted.
///
/// # Examples
///
/// ```
/// # use iphone_organizer::*;
/// # use std::{fs, path::PathBuf};
/// #
/// # let mockSOURCE = PathBuf::from("mockSOURCE");
/// # let mockDESTINATION = PathBuf::from("mockDESTINATION");
/// #
/// # fs::create_dir(&mockSOURCE);
/// #
/// # let args = vec![
/// #     String::from("program_name"),
/// #     String::from("mockSOURCE"),
/// #     String::from("mockDESTINATION"),
/// # ];
/// # let config = match Config::build(args.into_iter()) {
/// #     Ok(config) => config,
/// #     Err(error) => panic!("Failed to build `Config`: {}", error),
/// # };
/// #
/// if let Err(error) = run(config) {
///     panic!("Application error: {error}");
/// }
/// #
/// # fs::remove_dir_all(&mockDESTINATION);
/// # fs::remove_dir_all(&mockSOURCE);
/// ```
#[deprecated(since="1.2.4", note="Use the `organize` and `lean` functions directly.")]
pub fn run(config: Config) -> result::Result<(), Box<dyn Error>> {
    let (source, destination) = (config.source, config.destination);

    let source_path = PathBuf::from(&source);
    if !source_path.is_dir() {
        return Err(SourceDirNotExists.into());
    }

    let source_list = FileList::build(&source_path)?;

    file_ops::organize(&source_list, config.override_present, &source, &destination)?;

    if config.lean {
        let destination_list = FileList::build(&PathBuf::from(&destination))?;

        file_ops::lean(&destination_list, &source_list)?;
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct SourceDirNotExists;

impl fmt::Display for SourceDirNotExists {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source doesn't exist or is not a directory")
    }
}

impl Error for SourceDirNotExists {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_builds() {
        let args = vec![
            String::from("program_name"),
            String::from("mockSOURCE_lib"),
            String::from("mockDESTINATION_lib"),
            String::from("-l"),
            String::from("-o"),
        ];
        let config = match Config::build(args.into_iter()) {
            Ok(config) => config,
            Err(error) => panic!("Failed to build `Config`: {}", error),
        };

        assert_eq!(String::from("mockSOURCE_lib"), config.source);
        assert_eq!(String::from("mockDESTINATION_lib"), config.destination);
        assert_eq!(true, config.lean);
        assert_eq!(true, config.override_present);
    }
}
