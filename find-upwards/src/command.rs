use failure::Error;
use miscutils_core::Executable;
use std::borrow::Cow;
use std::cmp;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "find-upwards")]
/// `find-upwards` is a breadth-first, upwards finding utility, built to
/// quickly find a specific object in the given directory's parent tree.
pub struct Command {
    /// Name of the object to search for
    target: String,

    #[structopt(short, long)]
    /// Maximum depth to traverse upward during the search
    ///
    /// `max_depth` is optional. If not provided, `find-upwards` will
    /// traverse all the way up the parent tree, searching for `target`
    max_depth: Option<i32>,

    #[structopt(short, long)]
    /// Directory to begin the search in
    source_dir: Option<PathBuf>,

    #[structopt(short, long)]
    /// Return all matching paths instead of only the first
    all: bool,
}

impl Executable for Command {
    type Error = Error;

    fn execute(&self) -> Result<(), Self::Error> {
        // Get the current working directory if a source directory
        // is not provided
        let source_dir = match self.source_dir.as_ref() {
            Some(dir) => dir.clone(),
            None => get_working_directory()?,
        };

        // If the given path is not absolute, canonicalize it
        let source_dir = if !source_dir.has_root() {
            source_dir.canonicalize()?
        } else {
            source_dir
        };

        // Check that the source directory exists
        if !source_dir.exists() {
            let source_dir = source_dir.clone();
            let source_name = source_dir.to_string_lossy().to_owned();
            let source_name = match source_name {
                Cow::Borrowed(_) => {
                    return Err(super::error::Error::InternalError {
                        description: "expected an owned value for `source_name`".to_string(),
                    }
                    .into())
                }
                Cow::Owned(source_name) => source_name,
            };
            return Err(super::error::Error::PathDoesNotExist { name: source_name }.into());
        }

        // Ensure the fragment is relative, not absolute
        if self.target.starts_with("/") {
            return Err(super::error::Error::TargetIsAbsolute {
                name: self.target.clone(),
            }
            .into());
        }

        // Load the search paths for the source directory
        let search_paths = build_search_paths(&source_dir, self.max_depth);

        // Create paths to the requested item and check for existence
        let full_paths = search_paths
            .into_iter()
            .map(|path| path.join(&self.target))
            .filter(|path| path.exists())
            .map(|path| path.canonicalize().expect("could not get canonical path"))
            .collect::<Vec<PathBuf>>();

        dump_paths(full_paths, self.all);

        Ok(())
    }
}

fn build_search_paths<'a>(source_dir: &'a PathBuf, max_depth: Option<i32>) -> Vec<PathBuf> {
    let mut search_paths = Vec::new();
    search_paths.push(source_dir.clone());

    let mut parent = source_dir.parent();
    while let Some(path) = parent {
        search_paths.push(path.to_owned());
        parent = path.parent();
    }

    match max_depth {
        Some(depth) => {
            let depth = cmp::min(depth as usize, search_paths.len());
            search_paths.drain(0..depth as usize).collect()
        }
        None => search_paths,
    }
}

fn dump_paths(paths: Vec<PathBuf>, all: bool) {
    for path in paths {
        println!("{}", path.to_string_lossy());
        if !all {
            break;
        }
    }
}

fn get_working_directory() -> Result<PathBuf, Error> {
    Ok(std::env::current_dir()?)
}
