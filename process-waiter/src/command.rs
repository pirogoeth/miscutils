use failure::Error;
use miscutils_core::Executable;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "process-waiter")]
/// `process-waiter` is a WIP simple process supervisor that listens for
/// instructions on a Unix named pipe
pub struct Command {
    /// Command to supervise
    command: Vec<String>,

    #[structopt(short, long)]
    /// Path where the control pipe should be created.
    ///
    /// The control pipe is a Unix named pipe.
    control_path: Option<PathBuf>,
}

impl Executable for Command {
    type Error = Error;

    fn execute(&self) -> Result<(), Error> {
        let control_path = make_control_path(self.control_path.as_ref())?;

        println!("Command to run: {:?}", self.command);
        println!("Using `{:?}` as control path!", control_path);

        Ok(())
    }
}

fn make_control_path(control_path: Option<&PathBuf>) -> Result<PathBuf, Error> {
    match control_path {
        Some(path) => Ok(path.to_path_buf()),
        None => Ok(tempfile::Builder::new()
            .prefix("S.proc_waiter.")
            .rand_bytes(5)
            .tempfile()?
            .path()
            .to_path_buf()),
    }
}
