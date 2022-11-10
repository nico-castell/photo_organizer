use std::{env, error::Error};

pub struct Config {
    source: String,
    destination: String,
}

impl Config {
    /// Creates a `Config` type
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
/// # let config = Config {
/// #     source: String::from("/source"),
/// #     destination: String::from("/destination")
/// # }
/// if let Err(error) = run(config) {
///     panic!"Application error: {e}");
/// }
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    return Ok(());
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
