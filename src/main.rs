use miscutils_core::Executable;
use miscutils_find_upwards;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "swiss-army knife of miscellaneous utilities")]
#[structopt(name = "mu")]
enum Command {
    /// `find-upwards` is a breadth-first, upwards finding utility, built to
    /// quickly find a specific object in the given directory's parent tree.
    FindUpwards(miscutils_find_upwards::Command),
}

fn main() {
    let command = Command::from_args();
    let result = match command {
        Command::FindUpwards(item) => item.execute()
    };

    result.expect("Shit broke, yo");
}