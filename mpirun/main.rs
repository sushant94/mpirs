//! Implements mpirun equivalent

extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

static USAGE: &'static str = "
mpirs. Run MPI Programs in rust.
Usage:
  mpirs [options] [<executable>]

Options:
  -n --num <num_of_procs>   Define the number of processes to spawn
  -h --help                 Show this help screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_executable: Option<String>,
    arg_num_of_procs: Option<u64>,
    flag_help: bool,
    flag_num: bool,
}

fn parse_args(args: Args) -> Result<(), String> {
    unimplemented!()
}

fn main() {

    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
}
