use failure;
use miscutils_core::Executable;
use miscutils_find_upwards;
use miscutils_process_waiter;
use miscutils_ssl_tester;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "swiss-army knife of miscellaneous utilities")]
#[structopt(name = "mu")]
enum Command {
    FindUpwards(miscutils_find_upwards::Command),
    ProcessWaiter(miscutils_process_waiter::Command),
    SslTester(miscutils_ssl_tester::Command),
}

fn main() {
    let command = Command::from_args();
    let result = match command {
        Command::FindUpwards(item) => item.execute(),
        Command::ProcessWaiter(item) => item.execute(),
        Command::SslTester(item) => item.execute(),
    };

    if result.is_err() {
        dump_error_chain(&result.unwrap_err());
    }
}

fn dump_error_chain(err: &failure::Error) {
    for cause in failure::Error::iter_chain(err) {
        println!("{}: {}", cause.name().unwrap_or("Error"), cause);
    }
}
